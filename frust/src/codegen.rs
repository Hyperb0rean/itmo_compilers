use crate::ast::*;
use crate::inst::*;
use std::collections::HashMap;
use std::io::IoSlice;
use std::vec;

pub struct CodeGenContext {
    symbol_table: HashMap<String, u32>, // var -> address
    instructions: Vec<Instruction>,
    label_count: usize,
    stack_offset: u32,
}

impl CodeGenContext {
    pub fn new() -> Self {
        let stack_size = 32;
        CodeGenContext {
            symbol_table: HashMap::new(),
            instructions: vec![Instruction::new_itype(
                Opcode::Addi,
                Reg::StackPointer,
                Reg::Zero,
                stack_size,
            )],
            label_count: 0,
            stack_offset: stack_size,
        }
    }

    pub fn instructions(&self) -> &[Instruction] {
        self.instructions.as_slice()
    }

    fn generate_label(&mut self, base: String) -> String {
        let label = format!("{}_{}", base, self.label_count);
        self.label_count += 1;
        label
    }

    fn allocate_variable(&mut self, var: String) {
        self.symbol_table.insert(var.to_string(), self.stack_offset);
        self.stack_offset -= 8; // allocate 8 bytes for i64 (num type)
    }

    fn load_variable(&mut self, var: String, dest: Reg) -> Result<(), String> {
        if let Some(addr) = self.symbol_table.get(&var) {
            self.instructions.push(Instruction::new_itype(
                Opcode::Lw,
                dest,
                Reg::StackPointer,
                *addr,
            ));
            Ok(())
        } else {
            Err(format!("Variable {} not declared", var))
        }
    }

    fn store_variable(&mut self, var: String, src: Reg) -> Result<(), String> {
        if let Some(addr) = self.symbol_table.get(&var) {
            self.instructions.push(Instruction::new_stype(
                Opcode::Sw,
                Reg::StackPointer,
                src,
                *addr,
            ));
            Ok(())
        } else {
            Err(format!("Variable {} not declared", var))
        }
    }

    fn load_int_literal(&mut self, val: u32, dest: Reg) {
        self.instructions.push(Instruction::new_itype(
            Opcode::Lui,
            dest,
            Reg::Zero,
            val >> 12,
        ));
        self.instructions.push(Instruction::new_itype(
            Opcode::Addi,
            dest,
            Reg::Zero,
            val & 0b111111111111,
        ));
    }

    fn load_bool_literal(&mut self, val: bool, dest: Reg) {
        self.instructions.push(Instruction::new_itype(
            Opcode::Addi,
            dest,
            Reg::Zero,
            if val { 1 } else { 0 },
        ));
    }

    pub fn generate(&mut self, expr: &Expr) -> Result<(), String> {
        match expr {
            Expr::Number(n) => {
                self.load_int_literal(*n as u32, Reg::Temp(0));
            }
            Expr::Bool(b) => {
                self.load_bool_literal(*b, Reg::Temp(0));
            }
            Expr::Var(var) => {
                self.load_variable(var.clone(), Reg::Temp(0))?;
            }
            Expr::Assign { name, expr } => {
                self.generate(expr)?;
                self.store_variable(name.clone(), Reg::Temp(0))?;
            }
            Expr::Let {
                name,
                var_type, // TODO: Add type safety
                expr,
            } => {
                self.allocate_variable(name.clone());
                self.generate(expr)?;
                self.store_variable(name.clone(), Reg::Temp(0))?;
            }
            Expr::Binary { left, op, right } => {
                self.generate(left)?;

                self.instructions.push(Instruction::new_itype(
                    Opcode::Addi,
                    Reg::Temp(1),
                    Reg::Temp(0),
                    0,
                ));

                self.generate(right)?;

                self.instructions.push(Instruction::new_rtype(
                    match op {
                        BinaryOp::Add => Opcode::Add,
                        BinaryOp::Sub => Opcode::Sub,
                        BinaryOp::Mul => Opcode::Mul,
                        BinaryOp::Div => Opcode::Div,
                        BinaryOp::Mod => Opcode::Rem,
                        BinaryOp::And => Opcode::And,
                        BinaryOp::Or => Opcode::Or,
                        BinaryOp::Eq => Opcode::Seq,
                        BinaryOp::Neq => Opcode::Sne,
                        BinaryOp::Lt => Opcode::Slt,
                        BinaryOp::Ge => Opcode::Sge,
                        BinaryOp::Gt => todo!(),
                        BinaryOp::Le => todo!(),
                    },
                    Reg::Temp(0),
                    Reg::Temp(1),
                    Reg::Temp(0),
                ));
            }
            Expr::Unary { op, expr } => {
                // TODO typesafety
                match op {
                    UnaryOp::Not => {
                        self.instructions.push(Instruction::new_itype(
                            Opcode::Xori,
                            Reg::Temp(0),
                            Reg::Temp(0),
                            1,
                        ));
                    }
                    UnaryOp::Neg => {
                        self.instructions.push(Instruction::new_rtype(
                            Opcode::Sub,
                            Reg::Temp(0),
                            Reg::Zero,
                            Reg::Temp(0),
                        ));
                    }
                }
            }
            Expr::If {
                condition,
                then_branch,
                else_branch,
            } => {
                self.generate(condition)?;
                let condition_index = self.instructions.len();

                self.instructions.push(Instruction::new_itype(
                    Opcode::Beq,
                    Reg::Temp(0),
                    Reg::Zero,
                    0,
                ));

                for expr in then_branch {
                    self.generate(expr)?;
                }

                let end_then_index = self.instructions.len();
                if else_branch.is_some() {
                    self.instructions
                        .push(Instruction::new_jtype(Opcode::Jal, 0));
                }
                self.instructions[condition_index]
                    .set_offset(end_then_index as i32 - (condition_index as i32));

                if let Some(else_branch) = else_branch {
                    for expr in else_branch {
                        self.generate(expr)?;
                    }
                    let end_else_index = self.instructions.len();
                    self.instructions[end_then_index]
                        .set_offset(end_else_index as i32 - 1 - (end_then_index as i32));
                }
            }
            Expr::While { condition, body } => {
                let label = self.generate_label(String::from("while"));
                let condition_start = self.instructions.len();
                self.generate(condition)?;
                let condition_end = self.instructions.len();

                self.instructions.push(Instruction::new_itype(
                    Opcode::Beq,
                    Reg::Temp(0),
                    Reg::Zero,
                    0,
                ));

                for expr in body {
                    self.generate(expr)?;
                }
                let jmp_index = self.instructions.len();
                self.instructions.push(Instruction::new_jtype(
                    Opcode::Jal,
                    (condition_start as i32 - 1 - jmp_index as i32) as u32,
                ));
                self.instructions[condition_end]
                    .set_offset(jmp_index as i32 - condition_end as i32);
            }
        }
        Ok(())
    }
}
