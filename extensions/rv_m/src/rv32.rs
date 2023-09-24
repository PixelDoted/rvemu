use rv32i::RV32I;
use rvcore::{
    ins::{TypeOp, OPCODE_MASK, OPCODE_OP},
    Extension, Volatile,
};

pub struct RV32M;

impl Extension<RV32I> for RV32M {
    fn execute<'a>(&mut self, ins: u32, base: &mut RV32I) -> Option<()> {
        match ins & OPCODE_MASK {
            OPCODE_OP => {
                let ins = TypeOp::decode(ins);
                let rs1 = base.get(ins.rs1 as usize);
                let rs2 = base.get(ins.rs2 as usize);
                let value = match (ins.funct7, ins.funct3) {
                    (1, 0) => rs1.saturating_mul(rs2),                      // mul
                    (1, 1) => (((rs1 as i64) * (rs2 as i64)) >> 32) as i32, // mulh
                    (1, 2) | (1, 3) => (((rs1 as u64) * (rs2 as u64)) >> 32) as i32, // mulhu/mulhsu
                    (1, 4) => rs1 / rs2,                                    // div
                    (1, 5) => ((rs1 as u32) / (rs2 as u32)) as i32,         // divu
                    (1, 6) => rs1 % rs2,                                    // rem
                    (1, 7) => ((rs1 as u32) % (rs2 as u32)) as i32,         // remu

                    _ => return None,
                };

                base.set(ins.rd as usize, value);
            }

            _ => return None,
        }

        Some(())
    }
}
