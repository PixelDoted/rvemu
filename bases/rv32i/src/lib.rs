mod memory;

pub use memory::Memory;
use rvcore::Base;
use rvcore::Instruction;

#[derive(Default, Debug)]
pub struct RV32I {
    registers: [i32; 32],
    pc: i32,
}

impl Base<i32> for RV32I {
    type DATA = Memory;

    // ---- Fetch ----

    fn fetch(&mut self) -> i32 {
        let pc = self.pc;
        self.pc += 4;
        pc
    }

    // ---- Register ----

    fn set(&mut self, i: usize, value: i32) {
        if i == 0 {
            return;
        }

        self.registers[i] = value;
    }

    fn get(&self, i: usize) -> i32 {
        self.registers[i]
    }

    // ---- Execution ----

    fn execute(&mut self, ins: &Instruction, memory: &mut Memory) -> Option<()> {
        match ins {
            Instruction::OpImm(data) => {
                let rs1 = self.get(data.rs1 as usize);
                let imm11_0 = data.imm;
                let value = match data.funct3 {
                    0 => rs1 + imm11_0,                            // addi
                    2 => (rs1 < imm11_0) as i32,                   // slti
                    3 => ((rs1 as u32) < (imm11_0 as u32)) as i32, // sltiu
                    4 => rs1 ^ imm11_0,                            // xori
                    6 => rs1 | imm11_0,                            // ori
                    7 => rs1 & imm11_0,                            // andi

                    _ => return None,
                };

                self.set(data.rd as usize, value);
            }
            Instruction::Lui(data) => {
                self.set(data.rd as usize, data.imm << 12);
            }
            Instruction::AuiPc(data) => {
                let value = (data.imm << 12) + self.pc - 4;
                self.set(data.rd as usize, value);
            }
            Instruction::Op(data) => {
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

                    _ => return None,
                };

                self.set(data.rd as usize, value);
            }
            Instruction::Jal(data) => {
                self.set(data.rd as usize, self.pc);
                self.pc = self.pc - 4 + (data.imm >> 1);
            }
            Instruction::JalR(data) => {
                let rs1 = self.get(data.rs1 as usize) + data.imm;
                self.set(data.rd as usize, self.pc);
                self.pc = self.pc - 4 + rs1;
            }
            Instruction::Branch(data) => {
                let rs1 = self.get(data.rs1 as usize);
                let rs2 = self.get(data.rs2 as usize);
                let result = match data.funct3 {
                    0 => rs1 == rs2,                   // beq
                    1 => rs1 != rs2,                   // bne
                    4 => rs1 < rs2,                    // blt
                    5 => rs1 >= rs2,                   // bge
                    6 => (rs1 as u32) < (rs2 as u32),  // bltu
                    7 => (rs1 as u32) >= (rs2 as u32), // bgeu

                    _ => return None,
                };

                if result {
                    self.pc = self.pc - 4 + data.imm;
                }
            }
            Instruction::Load(data) => {
                let rs1 = (self.get(data.rs1 as usize) + data.imm) as usize;
                let value = match data.funct3 {
                    0 => memory.load_byte(rs1),      // lb
                    1 => memory.load_halfword(rs1),  // lh
                    2 => memory.load_word(rs1),      // lw
                    4 => memory.load_ubyte(rs1),     // lbu
                    5 => memory.load_uhalfword(rs1), // lhu

                    _ => return None,
                };

                self.set(data.rd as usize, value);
            }
            Instruction::Store(data) => {
                let rs1 = (self.get(data.rs1 as usize) + data.imm) as usize;
                let rs2 = self.get(data.rs2 as usize);
                match data.funct3 {
                    0 => memory.store_byte(rs1, rs2),     // sb
                    1 => memory.store_halfword(rs1, rs2), // sh
                    2 => memory.store_word(rs1, rs2),     // sw

                    _ => return None,
                }
            }
        }

        Some(())
    }
}
