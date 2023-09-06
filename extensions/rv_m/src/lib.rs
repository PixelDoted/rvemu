use rvcore::{Base, Extension, Instruction};

pub struct RvM;

impl Extension<i32, ()> for RvM {
    fn execute<B: Base<i32>>(&mut self, ins: &Instruction, base: &mut B) -> Option<()> {
        match ins {
            Instruction::OpImm(_) => return None,
            Instruction::Lui(_) => return None,
            Instruction::AuiPc(_) => return None,
            Instruction::Op(data) => {
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
            Instruction::Jal(_) => return None,
            Instruction::JalR(_) => return None,
            Instruction::Branch(_) => return None,
            Instruction::Load(_) => return None,
            Instruction::Store(_) => return None,
        }

        Some(())
    }
}
