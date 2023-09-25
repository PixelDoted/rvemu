use rvcore::{
    ins::{IType, OPCODE_MASK},
    Extension, Volatile,
};

const OPCODE_SYSTEM: u32 = 0b00011111;
type TypeSystem = IType;

pub struct RVZICSR {
    registers: [u64; 64], // 4096 1-bit registers
}

impl Volatile<bool> for RVZICSR {
    /// Sets the bit in register `index` to `value`
    fn set(&mut self, index: usize, value: bool) {
        let outer = index / 64;
        let inner = 63 - index % 64;
        self.registers[outer] ^= (value as u64) << inner;
    }

    /// Gets the bit in register `index`
    fn get(&self, index: usize) -> bool {
        let outer = index / 64;
        let inner = 63 - index % 64;
        (self.registers[outer] >> inner) & 1 == 1
    }
}

impl Extension<()> for RVZICSR {
    fn execute(&mut self, ins: u32, _: &mut ()) -> Option<()> {
        match ins & OPCODE_MASK {
            OPCODE_SYSTEM => {
                let ins = TypeSystem::decode(ins);
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

                    _ => return None,
                }
            }
            _ => return None,
        }

        Some(())
    }
}
