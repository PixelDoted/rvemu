// ---- R-Type ----

use crate::util::sign_extend;

pub struct RType {
    pub funct7: u8,
    pub rs2: u8,
    pub rs1: u8,
    pub funct3: u8,
    pub rd: u8,
}

impl RType {
    pub fn decode(ins: u32) -> Self {
        Self {
            funct7: (ins >> 25) as u8,
            rs2: ((ins >> 20) & 0b11111) as u8,
            rs1: ((ins >> 15) & 0b11111) as u8,
            funct3: ((ins >> 12) & 0b111) as u8,
            rd: ((ins >> 7) & 0b11111) as u8,
        }
    }
}

// ---- I-Type ----

pub struct IType {
    pub imm: i32,
    pub rs1: u8,
    pub funct3: u8,
    pub rd: u8,
}

impl IType {
    pub fn decode(ins: u32) -> Self {
        let imm11_0 = ins >> 20;

        Self {
            imm: sign_extend(imm11_0, 12),
            rs1: ((ins >> 15) & 0b11111) as u8,
            funct3: ((ins >> 12) & 0b111) as u8,
            rd: ((ins >> 7) & 0b11111) as u8,
        }
    }
}

// ---- U-Type ----

pub struct UType {
    pub imm: i32,
    pub rd: u8,
}

impl UType {
    pub fn decode(ins: u32) -> Self {
        let imm31_12 = ins & (0b11111111111111111111 << 12);

        Self {
            imm: sign_extend(imm31_12, 20),
            rd: ((ins >> 7) & 0b11111) as u8,
        }
    }
}

// ---- S-Type ----

pub struct SType {
    pub imm: i32,
    pub rs2: u8,
    pub rs1: u8,
    pub funct3: u8,
}

impl SType {
    pub fn decode(ins: u32) -> Self {
        let imm11_5 = (ins >> 20) & (0b111111 << 5);
        let imm4_0 = (ins >> 7) & 0b11111;

        Self {
            imm: sign_extend(imm11_5 | imm4_0, 11),
            rs2: ((ins >> 20) & 0b11111) as u8,
            rs1: ((ins >> 15) & 0b11111) as u8,
            funct3: ((ins >> 12) & 0b111) as u8,
        }
    }
}

// ---- B-Type ----

pub struct BType {
    pub imm: i32,
    pub rs2: u8,
    pub rs1: u8,
    pub funct3: u8,
}

impl BType {
    pub fn decode(ins: u32) -> Self {
        let imm12 = (ins >> 19) & (0b1 << 12);
        let imm10_5 = (ins >> 20) & (0b111111 << 5);
        let imm4_1 = (ins >> 7) & (0b1111 << 1);
        let imm11 = (ins << 4) & (0b1 << 11);

        Self {
            imm: sign_extend(imm12 | imm11 | imm10_5 | imm4_1, 12),
            rs2: ((ins >> 20) & 0b11111) as u8,
            rs1: ((ins >> 15) & 0b11111) as u8,
            funct3: ((ins >> 12) & 0b111) as u8,
        }
    }
}

// ---- J-Type ----

pub struct JType {
    pub imm: i32,
    pub rd: u8,
}

impl JType {
    pub fn decode(ins: u32) -> Self {
        let imm20 = (ins >> 11) & (0b1 << 20);
        let imm10_1 = (ins >> 20) & (0b1111111111 << 1);
        let imm11 = (ins >> 9) & (0b1 << 11);
        let imm19_12 = ins & (0b11111111 << 12);

        Self {
            imm: sign_extend(imm10_1 | imm11 | imm19_12 | imm20, 19),
            rd: ((ins >> 7) & 0b11111) as u8,
        }
    }
}
