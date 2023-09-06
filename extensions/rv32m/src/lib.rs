use rvcore::Extension;

pub struct RV32M {}

impl Extension for RV32M {
    fn execute<D>(&mut self, ins: u32, data: &mut D) -> Option<()> {
        todo!();
        Some(())
    }
}
