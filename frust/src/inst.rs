#[derive(Debug, Clone, Copy)]
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
    Jal,
    Jalr,
    Xor,
    Neg,
}


impl Opcode {
    pub fn to_string(&self) -> String {
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
            Opcode::Jal => "jal",
            Opcode::Jalr => "jalr",
            Opcode::Xor => "xor",
            Opcode::Neg => "neg",
        }.to_string()
    }
}

// https://en.wikipedia.org/wiki/RISC-V
#[derive(Debug,Clone, Copy)]
pub enum Reg {
    Zero, // x0
    ReturnAddress, //x1
    StackPointer, //x2
    GlobalPointer, //x3
    ThreadPointer, //x4
    Temp(u8), // x6-7 x28-31
    Saved(u8), // x8-9 x18-27
    Arguments(u8), // x10-11 return val, x12-17 args
}

impl Reg {
    pub fn to_string(&self) -> String {
        match self {
            Reg::Zero => "x0",
            Reg::ReturnAddress => "x1",
            Reg::StackPointer => "x2",
            Reg::GlobalPointer => "x3",
            Reg::ThreadPointer => "x4",
            Reg::Temp(id) => match id {
                0 => "x5",
                1 => "x6",
                2 => "x7",
                3 => "x28",
                4 => "x29",
                5 => "x30",
                6 => "x31",
                _ => panic!("Wrong temp reg {}", id),
            },
            Reg::Saved(id) => match id {
                0 => "x8",
                1 => "x9",
                _ => todo!("Not implemetned"),
            },
            Reg::Arguments(id) => todo!("Not implemented!"),
        }.to_string()
    }
}

#[derive(Debug,Clone, Copy)]
pub enum Type {
    R,
    I,
    S,
    U,
    B,
    J
}

#[derive(Debug,Clone, Copy)]
pub struct Instruction {
    notation: Type,
    opcode: Opcode,
    rd: Option<Reg>,
    rs1: Option<Reg>,
    rs2: Option<Reg>,
    imm: Option<usize>,
}

impl Instruction {
    /// R-Type: opcode rd, rs1, rs2
    pub fn new_rtype(opcode: Opcode, rd: Reg, rs1: Reg, rs2: Reg) -> Self {
        Instruction {
            notation: Type::R,
            opcode,
            rd: Some(rd),
            rs1: Some(rs1),
            rs2: Some(rs2),
            imm: None,
        }
    }

    /// I-Type: opcode rd, rs1, imm
    pub fn new_itype(opcode: Opcode, rd: Reg, rs1: Reg, imm: usize) -> Self {
        Instruction {
            notation: Type::I,
            opcode,
            rd: Some(rd),
            rs1: Some(rs1),
            rs2: None,
            imm: Some(imm),
        }
    }

    /// S-Type: opcode rs1, rs2, imm
    pub fn new_stype(opcode: Opcode, rs1: Reg, rs2: Reg, imm: usize) -> Self {
        Instruction {
            notation: Type::S,
            opcode,
            rd: None,
            rs1: Some(rs1),
            rs2: Some(rs2),
            imm: Some(imm),
        }
    }

    /// U-Type: opcode rd, imm
    pub fn new_utype(opcode: Opcode, rd: Reg, imm: usize) -> Self {
        Instruction {
            notation: Type::U,
            opcode,
            rd: Some(rd),
            rs1: None,
            rs2: None,
            imm: Some(imm),
        }
    }

    /// B-Type: opcode rs1, rs2, label(imm)
    pub fn new_btype(opcode: Opcode, rs1: Reg, rs2: Reg, imm: usize) -> Self {
        Instruction {
            notation: Type::B,
            opcode,
            rd: None,
            rs1: Some(rs1),
            rs2: Some(rs2),
            imm: Some(imm),
        }
    }

    /// J-Type: opcode label(imm)
    pub fn new_jtype(opcode: Opcode, imm: usize) -> Self {
        Instruction {
            notation: Type::J,
            opcode,
            rd: None,
            rs1: None,
            rs2: None,
            imm: Some(imm),
        }
    }

    pub fn to_string(&self) -> String {
        let mut parts = vec![self.opcode.to_string()];
        match self.notation {
            Type::R => todo!(),
            Type::I => todo!(),
            Type::S => todo!(),
            Type::U => todo!(),
            Type::B => todo!(),
            Type::J => todo!(),
        }
    }
}
