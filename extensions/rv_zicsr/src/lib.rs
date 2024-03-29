use rv32i::RV32I;
use rvcore::{
    ins::{IType, OPCODE_MASK},
    EResult, Extension, Volatile,
};

const OPCODE_SYSTEM: u32 = 0b00011111;
type TypeSystem = IType;

pub struct RVZICSR {
    registers: [u32; 4096], // 4096 registers
}

impl Volatile<u32> for RVZICSR {
    /// Sets the bit in register `index` to `value`
    fn set(&mut self, index: usize, value: u32) {
        self.registers[index] = value;
    }

    /// Gets the bit in register `index`
    fn get(&self, index: usize) -> u32 {
        self.registers[index]
    }
}

impl Extension<RV32I> for RVZICSR {
    fn execute(&mut self, ins: u32, base: &mut RV32I) -> EResult {
        match ins & OPCODE_MASK {
            OPCODE_SYSTEM => {
                let ins = TypeSystem::decode(ins);
                let csr = ins.imm as usize;

                match ins.funct3 {
                    1 => {
                        // csrrw
                        todo!();
                    }
                    2 => {
                        // csrrs
                        todo!();
                    }
                    3 => {
                        // csrrc
                        todo!();
                    }
                    5 => {
                        // csrrwi
                        todo!();
                    }
                    6 => {
                        // csrrsi
                        todo!();
                    }
                    7 => {
                        // csrrci
                        todo!();
                    }

                    _ => return EResult::NotFound,
                }
            }
            _ => return EResult::NotFound,
        }

        EResult::Found
    }
}
