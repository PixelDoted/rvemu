use rv32i::RV32I;
use rvcore::{ins::OPCODE_MASK, Extension, Memory, Volatile};

use crate::{TypeLoadF, TypeOpFp, TypeStoreF, OPCODE_LOADF, OPCODE_OPFP, OPCODE_STOREF};

pub struct RV32FData<'a> {
    pub base: &'a mut RV32I,
    pub memory: &'a mut Memory,
}

#[derive(Default)]
pub struct RV32F {
    registers: [f32; 32],
    csr: u32,
}

impl<'a> Extension<RV32FData<'a>> for RV32F {
    fn execute(&mut self, ins: u32, data: &mut RV32FData<'a>) -> Option<()> {
        match ins & OPCODE_MASK {
            OPCODE_LOADF => {
                let ins = TypeLoadF::decode(ins);
                if ins.funct3 != 2 {
                    return None;
                }

                let rs1 = (self.get(ins.rs1 as usize) as i32 + ins.imm) as usize;
                let value = data.memory.load_word(rs1) as f32;
                self.set(ins.rd as usize, value);
            }
            OPCODE_STOREF => {
                let ins = TypeStoreF::decode(ins);
                if ins.funct3 != 2 {
                    return None;
                }

                let rs1 = (self.get(ins.rs1 as usize) as i32 + ins.imm) as usize;
                data.memory
                    .store_word(rs1, self.get(ins.rs2 as usize) as i32);
            }
            OPCODE_OPFP => {
                let ins = TypeOpFp::decode(ins);
                let funct5 = ins.funct7 >> 2;
                let fmt = ins.funct7 & 0b11;
                let rm = ins.funct3;
                if fmt != 0 {
                    return None;
                }

                match funct5 {
                    0 => todo!(), // fadd.s
                    1 => todo!(), // fsub.s
                    2 => todo!(), // fmul.s
                    3 => todo!(), // fdiv.s
                    11 => {
                        if ins.rs2 == 0 {
                            // fsqrt.s
                            todo!()
                        } else {
                            return None;
                        }
                    }
                    4 => {
                        if rm == 0 {
                            // fsgnj.s
                            todo!()
                        } else if rm == 1 {
                            // fsgnjn.s
                            todo!()
                        } else if rm == 2 {
                            //fsgnjx.s
                            todo!()
                        } else {
                            return None;
                        }
                    }
                    5 => {
                        if rm == 0 {
                            // fmin.s
                            todo!()
                        } else if rm == 1 {
                            // fmax.s
                            todo!()
                        } else {
                            return None;
                        }
                    }
                    24 => {
                        if ins.rs2 == 0 {
                            // fcvt.w.s
                            todo!()
                        } else if ins.rs2 == 1 {
                            // rcvt.wu.s
                            todo!()
                        } else {
                            return None;
                        }
                    }
                    28 => {
                        if ins.rs2 == 0 && rm == 0 {
                            //fmv.x.w
                            todo!()
                        } else if ins.rs2 == 0 && rm == 1 {
                            // fclass.s
                            todo!()
                        } else {
                            return None;
                        }
                    }
                    20 => {
                        if rm == 2 {
                            // feq.s
                            todo!()
                        } else if rm == 1 {
                            // rlt.s
                            todo!()
                        } else if rm == 0 {
                            // fle.s
                            todo!()
                        } else {
                            return None;
                        }
                    }
                    26 => {
                        if ins.rs2 == 0 {
                            // fcvt.s.w
                            todo!()
                        } else if ins.rs2 == 1 {
                            // fcvt.s.wu
                            todo!()
                        } else {
                            return None;
                        }
                    }
                    30 => {
                        if ins.rs2 == 0 && rm == 0 {
                            // fmv.w.x
                            todo!()
                        } else {
                            return None;
                        }
                    }

                    _ => return None,
                }
            }

            _ => return None,
        }

        Some(())
    }
}

impl Volatile<f32> for RV32F {
    fn set(&mut self, index: usize, value: f32) {
        self.registers[index] = value;
    }

    fn get(&self, index: usize) -> f32 {
        self.registers[index]
    }
}
