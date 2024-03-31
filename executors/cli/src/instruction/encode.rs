use std::str::Chars;

use rvcore::ins::{
    TypeAuiPc, TypeBranch, TypeJal, TypeJalR, TypeLoad, TypeLui, TypeMiscMem, TypeOp, TypeOpImm,
    TypeStore, TypeSystem, OPCODE_AUIPC, OPCODE_BRANCH, OPCODE_JAL, OPCODE_JALR, OPCODE_LOAD,
    OPCODE_LUI, OPCODE_MISCMEM, OPCODE_OP, OPCODE_OPIMM, OPCODE_STORE, OPCODE_SYSTEM,
};

struct ArgParser<'a> {
    inner: Chars<'a>,
}

impl<'a> ArgParser<'a> {
    pub fn new(s: &'a str) -> Self {
        Self { inner: s.chars() }
    }

    pub fn register(&mut self) -> Option<u8> {
        if self.inner.next()? != 'x' {
            return None;
        }

        let (v, n) = self.number_internal(false)?;
        if n || v > 31 {
            None
        } else {
            Some(v as u8)
        }
    }

    fn number(&mut self, length: u32) -> Option<i32> {
        let (number, negative) = self.number_internal(true)?;

        if (!negative && 32 - number.leading_zeros() > length)
            || (negative && 32 - number.leading_ones() > length)
        {
            None
        } else {
            Some(number)
        }
    }

    fn number_internal(&mut self, special: bool) -> Option<(i32, bool)> {
        let mut text = String::new();
        loop {
            let c = self.inner.next();
            if c.is_none() {
                break;
            } else if c == Some(',') {
                self.inner.next()?;
                break;
            } else if special && (c == Some('(') || c == Some(')')) {
                break;
            }

            text.push(c.unwrap());
        }

        Some((text.parse::<i32>().ok()?, text.starts_with('-')))
    }
}

// ---- Encode ----
pub fn encode_instruction(text: &str) -> Option<u32> {
    encode_rv32i(text).or_else(|| encode_rv32m(text))
}

fn encode_rv32i(text: &str) -> Option<u32> {
    let (ins, data) = text.split_once(' ')?;
    let mut parser = ArgParser::new(data);
    Some(match ins {
        "addi" | "slti" | "sltiu" | "xori" | "ori" | "andi" => {
            let funct3 = if ins == "addi" {
                0
            } else if ins == "slti" {
                1
            } else if ins == "sltiu" {
                2
            } else if ins == "xori" {
                3
            } else if ins == "ori" {
                4
            } else if ins == "andi" {
                5
            } else {
                unreachable!()
            };

            OPCODE_OPIMM
                | TypeOpImm {
                    rd: parser.register()?,
                    rs1: parser.register()?,
                    imm: parser.number(12)? as i32,
                    funct3,
                }
                .encode()
        }
        "lui" => {
            OPCODE_LUI
                | TypeLui {
                    rd: parser.register()?,
                    imm: parser.number(20)? as i32,
                }
                .encode()
        }
        "auipc" => {
            OPCODE_AUIPC
                | TypeAuiPc {
                    rd: parser.register()?,
                    imm: parser.number(20)? as i32,
                }
                .encode()
        }
        "add" | "sub" | "sll" | "slt" | "sltu" | "xor" | "srl" | "sra" | "or" | "and" => {
            let (funct7, funct3) = if ins == "add" {
                (0, 0)
            } else if ins == "sub" {
                (32, 0)
            } else if ins == "sll" {
                (0, 1)
            } else if ins == "slt" {
                (0, 2)
            } else if ins == "sltu" {
                (0, 3)
            } else if ins == "xor" {
                (0, 4)
            } else if ins == "srl" {
                (0, 5)
            } else if ins == "sra" {
                (32, 5)
            } else if ins == "or" {
                (0, 6)
            } else if ins == "and" {
                (0, 7)
            } else {
                unreachable!()
            };

            OPCODE_OP
                | TypeOp {
                    rd: parser.register()?,
                    rs1: parser.register()?,
                    rs2: parser.register()?,
                    funct7,
                    funct3,
                }
                .encode()
        }
        "jal" => {
            OPCODE_JAL
                | TypeJal {
                    rd: parser.register()?,
                    imm: parser.number(19)? as i32,
                }
                .encode()
        }
        "jalr" => {
            OPCODE_JALR
                | TypeJalR {
                    rd: parser.register()?,
                    imm: parser.number(12)? as i32,
                    rs1: parser.register()?,
                    funct3: 0,
                }
                .encode()
        }
        "beq" | "bne" | "blt" | "bge" | "bltu" | "bgeu" => {
            let funct3 = if ins == "beq" {
                0
            } else if ins == "bne" {
                1
            } else if ins == "blt" {
                2
            } else if ins == "bge" {
                3
            } else if ins == "bltu" {
                4
            } else if ins == "bgeu" {
                5
            } else {
                unreachable!()
            };

            OPCODE_BRANCH
                | TypeBranch {
                    rs1: parser.register()?,
                    rs2: parser.register()?,
                    imm: parser.number(12)? as i32,
                    funct3,
                }
                .encode()
        }
        "lb" | "lh" | "lw" | "lbu" | "lhu" => {
            let funct3 = if ins == "lb" {
                0
            } else if ins == "lh" {
                1
            } else if ins == "lw" {
                2
            } else if ins == "lbu" {
                3
            } else if ins == "lhu" {
                4
            } else {
                unreachable!()
            };

            OPCODE_LOAD
                | TypeLoad {
                    rd: parser.register()?,
                    imm: parser.number(12)? as i32,
                    rs1: parser.register()?,
                    funct3,
                }
                .encode()
        }
        "sb" | "sh" | "sw" => {
            let funct3 = if ins == "sb" {
                0
            } else if ins == "sh" {
                1
            } else if ins == "sw" {
                2
            } else {
                unreachable!()
            };

            OPCODE_STORE
                | TypeStore {
                    rs2: parser.register()?,
                    imm: parser.number(11)? as i32,
                    rs1: parser.register()?,
                    funct3,
                }
                .encode()
        }
        "ecall" | "ebreak" => {
            let (funct12, funct3) = if ins == "ecall" {
                (0, 0)
            } else if ins == "ebreak" {
                (1, 0)
            } else {
                unreachable!()
            };

            OPCODE_SYSTEM
                | TypeSystem {
                    imm: funct12,
                    funct3,
                    rd: 0,
                    rs1: 0,
                }
                .encode()
        }

        // --- Fence ---
        "fence" => {
            let funct3 = 0;
            let pred = parser.number(4)?;
            let succ = parser.number(4)?;
            let imm = pred << 4 | succ;

            OPCODE_MISCMEM
                | TypeMiscMem {
                    imm: imm as i32,
                    funct3,
                    rs1: 0,
                    rd: 0,
                }
                .encode()
        }

        _ => return None,
    })
}

fn encode_rv32m(text: &str) -> Option<u32> {
    let (ins, data) = text.split_once(' ')?;
    let mut parser = ArgParser::new(data);
    Some(match ins {
        "mul" | "mulh" | "mulhu" | "mulhsu" | "div" | "divu" | "rem" | "remu" => {
            let (funct7, funct3) = if ins == "mul" {
                (1, 0)
            } else if ins == "mulh" {
                (1, 1)
            } else if ins == "mulhu" {
                (1, 2)
            } else if ins == "mulhsu" {
                (1, 3)
            } else if ins == "div" {
                (1, 4)
            } else if ins == "divu" {
                (1, 5)
            } else if ins == "rem" {
                (1, 6)
            } else if ins == "remu" {
                (1, 7)
            } else {
                unreachable!()
            };

            OPCODE_OP
                | TypeOp {
                    rd: parser.register()?,
                    rs1: parser.register()?,
                    rs2: parser.register()?,
                    funct7,
                    funct3,
                }
                .encode()
        }

        _ => return None,
    })
}

fn encode_rv32m(text: &str) -> Option<u32> {
    let (ins, data) = text.split_once(' ')?;
    let mut parser = ArgParser::new(data);
    Some(match ins {
        "flw" => {
            todo!();
        }
        "fsw" => {
            todo!();
        }
        "fadd.s" | "fsub.s" | "fmul.x" | "fdiv.s" => {
            todo!();
        }

        _ => return None,
    })
}
