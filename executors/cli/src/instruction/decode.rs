use rvcore::ins::{
    TypeAuiPc, TypeBranch, TypeJal, TypeJalR, TypeLoad, TypeLui, TypeMiscMem, TypeOp, TypeOpImm,
    TypeStore, TypeSystem, OPCODE_AUIPC, OPCODE_BRANCH, OPCODE_JAL, OPCODE_JALR, OPCODE_LOAD,
    OPCODE_LUI, OPCODE_MASK, OPCODE_MISCMEM, OPCODE_OP, OPCODE_OPIMM, OPCODE_STORE, OPCODE_SYSTEM,
};

pub fn decode_instruction(binary: u32) -> String {
    decode_rv32i(binary).unwrap_or("?".into())
}

fn decode_rv32i(ins: u32) -> Option<String> {
    Some(match ins & OPCODE_MASK {
        OPCODE_OPIMM => {
            let data = TypeOpImm::decode(ins);
            let imm11_0 = data.imm;
            match data.funct3 {
                0 => format!("addi x{}, x{}, {}", data.rd, data.rs1, imm11_0), // addi
                2 => format!("slti x{}, x{}, {}", data.rd, data.rs1, imm11_0), // slti
                3 => format!("sltiu x{}, x{}, {}", data.rd, data.rs1, imm11_0), // sltiu
                4 => format!("xori x{}, x{}, {}", data.rd, data.rs1, imm11_0), // xori
                6 => format!("ori x{}, x{}, {}", data.rd, data.rs1, imm11_0),  // ori
                7 => format!("andi x{}, x{}, {}", data.rd, data.rs1, imm11_0), // andi

                _ => return None,
            }
        }
        OPCODE_LUI => {
            let data = TypeLui::decode(ins);
            format!("lui x{}, {}", data.rd, data.imm)
        }
        OPCODE_AUIPC => {
            let data = TypeAuiPc::decode(ins);
            format!("auipc x{}, {}", data.rd, data.imm)
        }
        OPCODE_OP => {
            let data = TypeOp::decode(ins);
            match (data.funct7, data.funct3) {
                (0, 0) => format!("add x{}, x{}, x{}", data.rd, data.rs1, data.rs2), // add
                (32, 0) => format!("sub x{}, x{}, x{}", data.rd, data.rs1, data.rs2), // sub
                (0, 1) => format!("sll x{}, x{}, x{}", data.rd, data.rs1, data.rs2), // sll
                (0, 2) => format!("slt x{}, x{}, x{}", data.rd, data.rs1, data.rs2), // slt
                (0, 3) => format!("sltu x{}, x{}, x{}", data.rd, data.rs1, data.rs2), // sltu
                (0, 4) => format!("xor x{}, x{}, x{}", data.rd, data.rs1, data.rs2), // xor
                (0, 5) => format!("srl x{}, x{}, x{}", data.rd, data.rs1, data.rs2), // srl
                (32, 5) => format!("sra x{}, x{}, x{}", data.rd, data.rs1, data.rs2), // sra
                (0, 6) => format!("or x{}, x{}, x{}", data.rd, data.rs1, data.rs2),  // or
                (0, 7) => format!("and x{}, x{}, x{}", data.rd, data.rs1, data.rs2), // and

                _ => return None,
            }
        }
        OPCODE_JAL => {
            let data = TypeJal::decode(ins);
            format!("jal x{}, {}", data.rd, data.imm)
        }
        OPCODE_JALR => {
            let data = TypeJalR::decode(ins);
            format!("jalr x{}, {}(x{})", data.rd, data.imm, data.rs1)
        }
        OPCODE_BRANCH => {
            let data = TypeBranch::decode(ins);
            match data.funct3 {
                0 => format!("beq x{}, x{}, {}", data.rs1, data.rs2, data.imm), // beq
                1 => format!("bne x{}, x{}, {}", data.rs1, data.rs2, data.imm), // bne
                4 => format!("blt x{}, x{}, {}", data.rs1, data.rs2, data.imm), // blt
                5 => format!("bge x{}, x{}, {}", data.rs1, data.rs2, data.imm), // bge
                6 => format!("bltu x{}, x{}, {}", data.rs1, data.rs2, data.imm), // bltu
                7 => format!("bgeu x{}, x{}, {}", data.rs1, data.rs2, data.imm), // bgeu

                _ => return None,
            }
        }
        OPCODE_LOAD => {
            let data = TypeLoad::decode(ins);
            match data.funct3 {
                0 => format!("lb x{}, {}(x{})", data.rd, data.imm, data.rs1), // lb
                1 => format!("lh x{}, {}(x{})", data.rd, data.imm, data.rs1), // lh
                2 => format!("lw x{}, {}(x{})", data.rd, data.imm, data.rs1), // lw
                4 => format!("lbu x{}, {}(x{})", data.rd, data.imm, data.rs1), // lbu
                5 => format!("lhu x{}, {}(x{})", data.rd, data.imm, data.rs1), // lhu

                _ => return None,
            }
        }
        OPCODE_STORE => {
            let data = TypeStore::decode(ins);
            match data.funct3 {
                0 => format!("sb x{}, {}(x{})", data.rs2, data.imm, data.rs1), // sb
                1 => format!("sh x{}, {}(x{})", data.rs2, data.imm, data.rs1), // sh
                2 => format!("sw x{}, {}(x{})", data.rs2, data.imm, data.rs1), // sw

                _ => return None,
            }
        }
        OPCODE_SYSTEM => {
            let data = TypeSystem::decode(ins);
            let funct12 = data.imm;
            match (funct12, data.funct3) {
                (0, 0) => "ecall".into(),  // ecall
                (1, 0) => "ebreak".into(), // ebreak

                _ => return None,
            }
        }
        OPCODE_MISCMEM => {
            let data = TypeMiscMem::decode(ins);
            let fm = data.imm >> 27;
            let pi = (data.imm >> 26) & 1;
            let po = (data.imm >> 25) & 1;
            let pr = (data.imm >> 24) & 1;
            let pw = (data.imm >> 23) & 1;
            let si = (data.imm >> 22) & 1;
            let so = (data.imm >> 21) & 1;
            let sr = (data.imm >> 20) & 1;
            let sw = (data.imm >> 19) & 1;
            match data.funct3 {
                0 => {
                    if fm == 8
                        && ((pi << 3) | (po << 2) | (pr << 1) | pw) == 3
                        && ((si << 3) | (so << 2) | (sr << 1) | sw) == 3
                    {
                        // TODO: fence.tso
                    } else if fm == 0
                        && ((pi << 3) | (po << 2) | (pr << 1) | pw) == 1
                        && ((si << 3) | (so << 2) | (sr << 1) | sw) == 0
                        && data.rs1 == 0
                    {
                        // TODO: pause
                    } else {
                        // TODO:  fence
                    }
                }

                _ => return None,
            }

            return None;
        }

        _ => return None,
    })
}
