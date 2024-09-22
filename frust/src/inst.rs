#[derive(Debug)]
pub enum Opcode {
    Add,
    Sub,
    Mul,
    Div,
    And,
    Or,
    Slt,
    Sgt,
    Beq,
    Bne,
    Li,
    Lw,
    Sw,
    Addi,
    J,
    Jalr,
    Xor,
    Neg,
}

impl Opcode {
    pub fn to_string(&self) -> &str {
        match self {
            Opcode::Add => "add",
            Opcode::Sub => "sub",
            Opcode::Mul => "mul",
            Opcode::Div => "div",
            Opcode::And => "and",
            Opcode::Or => "or",
            Opcode::Slt => "slt",
            Opcode::Sgt => "sgt",
            Opcode::Beq => "beq",
            Opcode::Bne => "bne",
            Opcode::Li => "li",
            Opcode::Lw => "lw",
            Opcode::Sw => "sw",
            Opcode::Addi => "addi",
            Opcode::J => "j",
            Opcode::Jalr => "jalr",
            Opcode::Xor => "xor",
            Opcode::Neg => "neg",
        }
    }
}

#[derive(Debug)]
pub struct Instruction {
    opcode: Opcode,
    rd: Option<String>,
    rs1: Option<String>,
    rs2: Option<String>,
    imm: Option<String>,
}

impl Instruction {
    /// R-Type: opcode rd, rs1, rs2
    pub fn new_rtype(opcode: Opcode, rd: String, rs1: String, rs2: String) -> Self {
        Instruction {
            opcode,
            rd: Some(rd),
            rs1: Some(rs1),
            rs2: Some(rs2),
            imm: None,
        }
    }

    /// I-Type: opcode rd, rs1, imm
    pub fn new_itype(opcode: Opcode, rd: String, rs1: String, imm: String) -> Self {
        Instruction {
            opcode,
            rd: Some(rd),
            rs1: Some(rs1),
            rs2: None,
            imm: Some(imm),
        }
    }

    /// S-Type: opcode rs1, rs2, imm
    pub fn new_stype(opcode: Opcode, rs1: String, rs2: String, imm: String) -> Self {
        Instruction {
            opcode,
            rd: None,
            rs1: Some(rs1),
            rs2: Some(rs2),
            imm: Some(imm),
        }
    }

    /// U-Type: opcode rd, imm
    pub fn new_utype(opcode: Opcode, rd: String, imm: String) -> Self {
        Instruction {
            opcode,
            rd: Some(rd),
            rs1: None,
            rs2: None,
            imm: Some(imm),
        }
    }

    /// B-Type: opcode rs1, rs2, label
    pub fn new_btype(opcode: Opcode, rs1: String, rs2: String, label: String) -> Self {
        Instruction {
            opcode,
            rd: None,
            rs1: Some(rs1),
            rs2: Some(rs2),
            imm: Some(label),
        }
    }

    /// J-Type: opcode label
    pub fn new_jtype(opcode: Opcode, label: String) -> Self {
        Instruction {
            opcode,
            rd: None,
            rs1: None,
            rs2: None,
            imm: Some(label),
        }
    }

    pub fn to_string(&self) -> String {
        let mut parts = vec![self.opcode.to_string().to_string()];
        if let Some(rd) = self.rd.clone() {
            parts.push(rd.to_string());
        }
        if let Some(rs1) = self.rs1.clone() {
            parts.push(rs1.to_string());
        }
        if let Some(rs2) = self.rs2.clone() {
            parts.push(rs2.to_string());
        }
        if let Some(imm) = self.imm.clone() {
            parts.push(imm.to_string());
        }
        parts.join(", ")
    }
}
