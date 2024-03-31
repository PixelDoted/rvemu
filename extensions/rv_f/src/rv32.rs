use std::convert::Infallible;

use rv32i::RV32I;
use rvcore::{ins::OPCODE_MASK, EResult, Extension, Volatile};

use crate::{TypeLoadF, TypeOpFp, TypeStoreF, OPCODE_LOADF, OPCODE_OPFP, OPCODE_STOREF};

fn round(value: f32, rm: u8) -> f32 {
    if rm == 0 {
        value.round_ties_even()
    } else if rm == 1 || rm == 4 {
        if value > 0.0 {
            value.floor()
        } else {
            value.ceil()
        }
    } else if rm == 2 {
        value.floor()
    } else if rm == 3 {
        value.ceil()
    } else {
        eprintln!("unsupported rounding mode, assuming `ties to even`");
        value.round_ties_even()
    }
}

#[derive(Default)]
pub struct RV32F {
    registers: [f32; 32],
    fcsr: u32,
}

impl Extension<RV32I> for RV32F {
    fn execute(&mut self, ins: u32, base: &mut RV32I) -> EResult {
        match ins & OPCODE_MASK {
            OPCODE_LOADF => {
                let ins = TypeLoadF::decode(ins);
                if ins.funct3 != 2 {
                    return EResult::NotFound;
                }

                let rs1 = (self.get(ins.rs1 as usize) as i32 + ins.imm) as usize;
                let value = base.bus().load(rs1, 32) as f32;
                self.set(ins.rd as usize, value);
            }
            OPCODE_STOREF => {
                let ins = TypeStoreF::decode(ins);
                if ins.funct3 != 2 {
                    return EResult::NotFound;
                }

                let rs1 = (self.get(ins.rs1 as usize) as i32 + ins.imm) as usize;
                base.bus().store(rs1, 32, self.get(ins.rs2 as usize) as u32);
            }
            OPCODE_OPFP => {
                let ins = TypeOpFp::decode(ins);
                let funct5 = ins.funct7 >> 2;
                let fmt = ins.funct7 & 0b11;
                let rm = ins.funct3;
                if fmt != 0 {
                    return EResult::NotFound;
                }

                match funct5 {
                    0 => {
                        // fadd.s
                        let rs1 = self.get(ins.rs1 as usize);
                        let rs2 = self.get(ins.rs2 as usize);
                        self.set(ins.rd as usize, rs1 + rs2);
                    }
                    1 => {
                        // fsub.s
                        let rs1 = self.get(ins.rs1 as usize);
                        let rs2 = self.get(ins.rs2 as usize);
                        self.set(ins.rd as usize, rs1 - rs2);
                    }
                    2 => {
                        // fmul.s
                        let rs1 = self.get(ins.rs1 as usize);
                        let rs2 = self.get(ins.rs2 as usize);
                        self.set(ins.rd as usize, rs1 - rs2);
                    }
                    3 => {
                        // fdiv.s
                        let rs1 = self.get(ins.rs1 as usize);
                        let rs2 = self.get(ins.rs2 as usize);
                        self.set(ins.rd as usize, rs1 / rs2);
                    }
                    11 => {
                        if ins.rs2 == 0 {
                            // fsqrt.s
                            let rs1 = self.get(ins.rs1 as usize);
                            self.set(ins.rd as usize, rs1.sqrt());
                        } else {
                            return EResult::NotFound;
                        }
                    }
                    4 => {
                        let rs1 = self.get(ins.rs1 as usize);
                        let rs2 = self.get(ins.rs2 as usize);
                        let value = if rm == 0 {
                            // fsgnj.s
                            rs1.abs() * rs2.signum()
                        } else if rm == 1 {
                            // fsgnjn.s
                            rs1.abs() * -rs2.signum()
                        } else if rm == 2 {
                            //fsgnjx.s
                            let sign =
                                1 - (rs1.is_sign_positive() ^ rs2.is_sign_positive()) as i8 * 2;
                            rs1.abs() * sign as f32
                        } else {
                            return EResult::NotFound;
                        };

                        self.set(ins.rd as usize, value);
                    }
                    5 => {
                        let rs1 = self.get(ins.rs1 as usize);
                        let rs2 = self.get(ins.rs2 as usize);

                        let value = if rm == 0 {
                            // fmin.s
                            rs1.min(rs2)
                        } else if rm == 1 {
                            // fmax.s
                            rs1.max(rs2)
                        } else {
                            return EResult::NotFound;
                        };

                        self.set(ins.rd as usize, value);
                    }
                    24 => {
                        if ins.rs2 == 0 {
                            // fcvt.w.s
                            let rs1 = self.get(ins.rs1 as usize);
                            let rs1_r = round(rs1, rm);
                            base.set(ins.rd as usize, rs1_r as i32);
                        } else if ins.rs2 == 1 {
                            // rcvt.wu.s
                            let rs1 = self.get(ins.rs1 as usize);
                            let rs1_r = round(rs1, rm);
                            base.set(ins.rd as usize, rs1_r as u32 as i32);
                        } else {
                            return EResult::NotFound;
                        }
                    }
                    28 => {
                        if ins.rs2 == 0 && rm == 0 {
                            //fmv.x.w
                            let rs1 = self.get(ins.rs1 as usize);
                            base.set(ins.rd as usize, rs1.to_bits() as i32);
                        } else if ins.rs2 == 0 && rm == 1 {
                            // fclass.s
                            let rs1 = self.get(ins.rs1 as usize);
                            let infinite = rs1.is_infinite();
                            let positive = rs1.is_sign_positive();
                            let normal = rs1.is_normal();

                            let value = ((rs1.is_nan() as u32) << 9) // TODO: quiet NaN
                                | ((rs1.is_nan() as u32) << 8) // TODO: signaling Nan
                                | (((infinite && positive) as u32) << 7)
                                | (((normal && positive) as u32) << 6)
                                | (((!normal && positive) as u32) << 5)
                                | (((rs1 == 0.0 && positive) as u32) << 4)
                                | (((rs1 == 0.0 && !positive) as u32) << 3)
                                | (((!normal && !positive) as u32) << 2)
                                | (((normal && !positive) as u32) << 1)
                                | (infinite && !positive) as u32;

                            base.set(ins.rd as usize, value as i32);
                        } else {
                            return EResult::NotFound;
                        }
                    }
                    20 => {
                        let rs1 = self.get(ins.rs1 as usize);
                        let rs2 = self.get(ins.rs2 as usize);

                        let value = if rm == 2 {
                            // feq.s
                            rs1 == rs2
                        } else if rm == 1 {
                            // flt.s
                            rs1 < rs2
                        } else if rm == 0 {
                            // fle.s
                            rs1 <= rs2
                        } else {
                            return EResult::NotFound;
                        };

                        base.set(ins.rd as usize, value as i32);
                    }
                    26 => {
                        if ins.rs2 == 0 {
                            // fcvt.s.w
                            let rs1 = base.get(ins.rs1 as usize);
                            self.set(ins.rd as usize, rs1 as f32);
                        } else if ins.rs2 == 1 {
                            // fcvt.s.wu
                            let rs1 = base.get(ins.rs1 as usize) as u32;
                            self.set(ins.rd as usize, rs1 as f32);
                        } else {
                            return EResult::NotFound;
                        }
                    }
                    30 => {
                        if ins.rs2 == 0 && rm == 0 {
                            // fmv.w.x
                            let rs1 = base.get(ins.rs1 as usize) as u32;
                            self.set(ins.rd as usize, f32::from_bits(rs1));
                        } else {
                            return EResult::NotFound;
                        }
                    }

                    _ => return EResult::NotFound,
                }
            }

            _ => return EResult::NotFound,
        }

        EResult::Found
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
