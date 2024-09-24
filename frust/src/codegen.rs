use crate::ast::*;
use crate::inst::*;
use std::collections::HashMap;
use std::vec;

pub struct CodeGenContext {
    symbol_table: HashMap<String, usize>, // var -> address
    instructions: Vec<Instruction>,
    label_count: usize,
    stack_offset: usize,
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

    fn load_int_literal(&mut self, val: i64, dest: Reg) {
        self.instructions.push(Instruction::new_itype(
            Opcode::Addi,
            dest,
            Reg::Zero,
            val as usize,
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
                self.load_int_literal(*n, Reg::Temp(0));
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
                        BinaryOp::And => Opcode::And,
                        BinaryOp::Or => Opcode::Or,
                        BinaryOp::Eq => todo!(),
                        BinaryOp::Neq => todo!(),
                        BinaryOp::Lt => todo!(),
                        BinaryOp::Gt => todo!(),
                        BinaryOp::Le => todo!(),
                        BinaryOp::Ge => todo!(),
                    },
                    Reg::Temp(0),
                    Reg::Temp(1),
                    Reg::Temp(0),
                ));
            }
            Expr::Unary { op, expr } => todo!(),
            Expr::If {
                condition,
                then_branch,
                else_branch,
            } => todo!(),
            Expr::While { condition, body } => todo!(),
        }
        Ok(())
    }
}
