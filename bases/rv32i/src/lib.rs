use std::fmt::Debug;

use rvcore::{
    bus::Bus,
    ins::{
        TypeAuiPc, TypeBranch, TypeJal, TypeJalR, TypeLoad, TypeLui, TypeMiscMem, TypeOp,
        TypeOpImm, TypeStore, TypeSystem, OPCODE_AUIPC, OPCODE_BRANCH, OPCODE_JAL, OPCODE_JALR,
        OPCODE_LOAD, OPCODE_LUI, OPCODE_MASK, OPCODE_MISCMEM, OPCODE_OP, OPCODE_OPIMM,
        OPCODE_STORE,
    },
    Base, EResult, Volatile,
};

#[derive(Debug)]
pub struct RV32I {
    registers: [i32; 32],
    pc: i32,
    bus: Bus,
}

impl RV32I {
    pub fn new(bus: Bus) -> Self {
        let mut registers = [0i32; 32];
        registers[2] = bus.dram.size() as i32;

        Self {
            registers,
            pc: 0,
            bus,
        }
    }

    pub fn bus(&mut self) -> &mut Bus {
        &mut self.bus
    }

    /// DEBUG
    pub fn bus_ref(&self) -> &Bus {
        &self.bus
    }

    /// DEBUG
    pub fn pc(&self) -> &i32 {
        &self.pc
    }
}

impl Volatile<i32> for RV32I {
    fn set(&mut self, i: usize, value: i32) {
        if i == 0 {
            return;
        }

        self.registers[i] = value;
    }

    fn get(&self, i: usize) -> i32 {
        self.registers[i]
    }
}

impl Base<i32> for RV32I {
    // ---- Fetch ----
    fn fetch(&mut self) -> i32 {
        let value = self.bus.load(self.pc as usize, 32);
        self.pc += 4;
        value as i32
    }

    // ---- Execution ----
    fn execute(&mut self, ins: u32) -> EResult {
        match ins & OPCODE_MASK {
            0 => return EResult::NotFound, // If this isn't here `OPCODE_SYSTEM` is matched instead

            OPCODE_OPIMM => {
                let data = TypeOpImm::decode(ins);
                let rs1 = self.get(data.rs1 as usize);
                let imm11_0 = data.imm;
                let value = match data.funct3 {
                    0 => rs1 + imm11_0,                            // addi
                    2 => (rs1 < imm11_0) as i32,                   // slti
                    3 => ((rs1 as u32) < (imm11_0 as u32)) as i32, // sltiu
                    4 => rs1 ^ imm11_0,                            // xori
                    6 => rs1 | imm11_0,                            // ori
                    7 => rs1 & imm11_0,                            // andi

                    _ => return EResult::NotFound,
                };

                self.set(data.rd as usize, value);
            }
            OPCODE_LUI => {
                let data = TypeLui::decode(ins);
                self.set(data.rd as usize, data.imm << 12);
            }
            OPCODE_AUIPC => {
                let data = TypeAuiPc::decode(ins);
                let value = (data.imm << 12) + self.pc - 4;
                self.set(data.rd as usize, value);
            }
            OPCODE_OP => {
                let data = TypeOp::decode(ins);
                let rs1 = self.get(data.rs1 as usize);
                let rs2 = self.get(data.rs2 as usize);
                let value = match (data.funct7, data.funct3) {
                    (0, 0) => rs1 + rs2,                             // add
                    (32, 0) => rs1 - rs2,                            // sub
                    (0, 1) => ((rs1 as u32) << (rs2 as u32)) as i32, // sll
                    (0, 2) => (rs1 < rs2) as i32,                    // slt
                    (0, 3) => ((rs1 as u32) < (rs2 as u32)) as i32,  // sltu
                    (0, 4) => rs1 ^ rs2,                             // xor
                    (0, 5) => ((rs1 as u32) >> (rs2 as u32)) as i32, // srl
                    (32, 5) => rs1 >> rs2,                           // sra
                    (0, 6) => rs1 | rs2,                             // or
                    (0, 7) => rs1 & rs2,                             // and

                    _ => return EResult::NotFound,
                };

                self.set(data.rd as usize, value);
            }
            OPCODE_JAL => {
                let data = TypeJal::decode(ins);
                self.set(data.rd as usize, self.pc);
                self.pc = self.pc - 4 + (data.imm >> 1);
            }
            OPCODE_JALR => {
                let data = TypeJalR::decode(ins);
                let rs1 = self.get(data.rs1 as usize) + data.imm;
                self.set(data.rd as usize, self.pc);
                self.pc = self.pc - 4 + rs1;
            }
            OPCODE_BRANCH => {
                let data = TypeBranch::decode(ins);
                let rs1 = self.get(data.rs1 as usize);
                let rs2 = self.get(data.rs2 as usize);
                let result = match data.funct3 {
                    0 => rs1 == rs2,                   // beq
                    1 => rs1 != rs2,                   // bne
                    4 => rs1 < rs2,                    // blt
                    5 => rs1 >= rs2,                   // bge
                    6 => (rs1 as u32) < (rs2 as u32),  // bltu
                    7 => (rs1 as u32) >= (rs2 as u32), // bgeu

                    _ => return EResult::NotFound,
                };

                if result {
                    self.pc = self.pc - 4 + data.imm;
                }
            }
            OPCODE_LOAD => {
                let data = TypeLoad::decode(ins);
                let rs1 = (self.get(data.rs1 as usize) + data.imm) as usize;

                let bus = self.bus();
                let value = match data.funct3 {
                    0 => bus.load(rs1, 8) as i8 as i32,   // lb
                    1 => bus.load(rs1, 16) as i16 as i32, // lh
                    2 => bus.load(rs1, 32) as i32,        // lw
                    4 => bus.load(rs1, 8) as i32,         // lbu
                    5 => bus.load(rs1, 16) as i32,        // lhu

                    _ => return EResult::NotFound,
                };

                self.set(data.rd as usize, value);
            }
            OPCODE_STORE => {
                let data = TypeStore::decode(ins);
                let rs1 = (self.get(data.rs1 as usize) + data.imm) as usize;
                let rs2 = self.get(data.rs2 as usize);

                let bus = self.bus();
                match data.funct3 {
                    0 => bus.store(rs1, 8, rs2 as u32),  // sb
                    1 => bus.store(rs1, 16, rs2 as u32), // sh
                    2 => bus.store(rs1, 32, rs2 as u32), // sw

                    _ => return EResult::NotFound,
                }
            }
            OPCODE_SYSTEM => {
                let data = TypeSystem::decode(ins);
                let funct12 = data.imm;
                match (funct12, data.funct3) {
                    (0, 0) => return EResult::ECall,  // ecall
                    (1, 0) => return EResult::EBreak, // ebreak

                    _ => return EResult::NotFound,
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
                            todo!("fence.tso"); // fence.tso
                        } else if fm == 0
                            && ((pi << 3) | (po << 2) | (pr << 1) | pw) == 1
                            && ((si << 3) | (so << 2) | (sr << 1) | sw) == 0
                            && data.rs1 == 0
                        {
                            todo!("pause"); // pause
                        } else {
                            todo!("fence"); // fence
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
