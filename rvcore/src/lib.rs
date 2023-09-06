pub mod ins;
pub mod util;

pub type QUADWORD = i128;
pub type DOUBLEWORD = i64;
pub type WORD = i32;
pub type HALFWORLD = i16;

pub trait Base<I> {
    type DATA;

    /// Fetches the current `program counter`
    fn fetch(&mut self) -> I;

    /// Attempts to execute an instruction  
    /// Returns None if the instruction isn't supported
    #[must_use]
    fn execute(&mut self, ins: u32, data: &mut Self::DATA) -> Option<()>;

    /// Sets the value in register `i`
    fn set(&mut self, i: usize, value: I);

    /// Gets the value in register `i`
    fn get(&self, i: usize) -> I;
}

pub trait Extension<T, I> {
    /// Attempts to execute an instruction
    /// Returns None if the instruction isn't supported
    #[must_use]
    fn execute<B: Base<T>>(&mut self, ins: u32, base: &mut B) -> Option<()>;

    /// Sets the value in register `i`  
    /// if this extension has registers
    fn set(&mut self, _i: usize, _value: T) {
        unimplemented!("This extension doesn't have registers");
    }

    /// Gets the value is register `i`  
    /// if this extension has registers
    fn get(&self, _i: usize) -> T {
        unimplemented!("This extension doesn't have registers");
    }
}
