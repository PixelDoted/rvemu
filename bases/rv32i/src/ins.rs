use rvcore::types::*;

pub enum Instruction {
    OpImm(IType),
    Lui(UType),
    AuiPc(UType),
    Op(RType),
    Jal(JType),
    JalR(IType),
    Branch(BType),
    Load(IType),
    Store(SType),
}

impl Instruction {
    pub fn decode(data: u32) -> Option<Self> {
        let opcode = data & 0b1111111;
        Some(match opcode {
            0b0010011 => Self::OpImm(IType::decode(data)),
            0b0110111 => Self::Lui(UType::decode(data)),
            0b0010111 => Self::AuiPc(UType::decode(data)),
            0b0110011 => Self::Op(RType::decode(data)),
            0b1101111 => Self::Jal(JType::decode(data)),
            0b1100111 => Self::JalR(IType::decode(data)),
            0b1100011 => Self::Branch(BType::decode(data)),
            0b0000011 => Self::Load(IType::decode(data)),
            0b0100011 => Self::Store(SType::decode(data)),

            _ => return None,
        })
    }
}
