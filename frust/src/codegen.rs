use crate::ast::*;
use crate::inst::*;
use std::collections::HashMap;

pub struct CodeGenContext {
    symbol_table: HashMap<String, usize>, // var -> address
    instructions: Vec<Instruction>,
    label_count: usize,
    stack_offset: usize,
}

impl CodeGenContext {
    pub fn new() -> Self {
        CodeGenContext {
            symbol_table: HashMap::new(),
            instructions: Vec::new(),
            label_count: 0,
            stack_offset: 0,
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
        self.stack_offset += 8; // allocate 8 bytes for i64 (num type)
        self.symbol_table.insert(var.to_string(), self.stack_offset);
    }

    fn load_variable(&mut self, var: String, dest: Reg) -> Result<(), String> {
        if let Some(addr) = self.symbol_table.get(&var) {
            // self.instructions.push(Instruction::new_itype(
            //     Opcode::Lw,
            //     dest,
            //     String::from("sp"),
            //     addr.to_string(),
            // ));
            Ok(())
        } else {
            Err(format!("Variable {} not declared", var))
        }
    }

    fn store_variable(&mut self, var: String, src: Reg) -> Result<(), String> {
        if let Some(addr) = self.symbol_table.get(&var) {
            // self.instructions.push(Instruction::new_stype(
            //     Opcode::Sw,
            //     String::from("sp"),
            //     src,
            //     addr.to_string(),
            // ));
            Ok(())
        } else {
            Err(format!("Variable {} not declared", var))
        }
    }

    fn load_int_literal(&mut self, imm: i64, dest: Reg) {
        // self.instructions.push(Instruction::new_itype(
        //     Opcode::Li,
        //     dest,
        //     String::from("zero"),
        //     imm.to_string(),
        // ));
    }

    fn load_bool_literal(&mut self, imm: bool, dest: Reg) {
        // Convert bool to 1 or 0 int type
        let val = if imm {
            "1".to_string()
        } else {
            "0".to_string()
        };
        // self.instructions.push(Instruction::new_itype(
        //     Opcode::Li,
        //     dest,
        //     String::from("zero"),
        //     val,
        // ));
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
                var_type,
                expr,
            } => {
                self.allocate_variable(name.clone());
                self.generate(expr)?;
                self.store_variable(name.clone(), Reg::Temp(0))?;
            }
            Expr::Binary { left, op, right } => {
                self.generate(left)?;

                // self.instructions.push(Instruction::new_rtype(
                //     Opcode::Addi,
                //     String::from("t1"),
                //     String::from("t0"),
                //     String::from("zero"),
                // ));

                self.generate(right)?;
                self.instructions.push(Instruction::new_rtype(
                    match op {
                        BinaryOp::Add => Opcode::Add,
                        BinaryOp::Sub => Opcode::Sub,
                        BinaryOp::Mul => Opcode::Mul,
                        BinaryOp::Div => todo!(),
                        BinaryOp::And => todo!(),
                        BinaryOp::Or => todo!(),
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
