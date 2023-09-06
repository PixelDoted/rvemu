use rvcore::{
    ins::{TypeOp, OPCODE_MASK, OPCODE_OP},
    Base, Extension,
};

pub struct RvM;

impl Extension<i32, ()> for RvM {
    fn execute<B: Base<i32>>(&mut self, ins: u32, base: &mut B) -> Option<()> {
        match ins & OPCODE_MASK {
            OPCODE_OP => {
                let data = TypeOp::decode(ins);
                let rs1 = base.get(data.rs1 as usize);
                let rs2 = base.get(data.rs2 as usize);
                let value = match (data.funct7, data.funct3) {
                    (1, 0) => rs1.saturating_mul(rs2),                      // mul
                    (1, 1) => (((rs1 as i64) * (rs2 as i64)) >> 32) as i32, // mulh
                    (1, 2) | (1, 3) => (((rs1 as u64) * (rs2 as u64)) >> 32) as i32, // mulhu/mulhsu
                    (1, 4) => rs1 / rs2,                                    // div
                    (1, 5) => ((rs1 as u32) / (rs2 as u32)) as i32,         // divu
                    (1, 6) => rs1 % rs2,                                    // rem
                    (1, 7) => ((rs1 as u32) % (rs2 as u32)) as i32,         // remu

                    _ => return None,
                };

                base.set(data.rd as usize, value);
            }

            _ => return None,
        }

        Some(())
    }
}
