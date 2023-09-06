pub mod types;
pub mod util;

pub type QUADWORD = i128;
pub type DOUBLEWORD = i64;
pub type WORD = i32;
pub type HALFWORLD = i16;

pub trait Base<I, D> {
    /// Fetches the current `program counter`
    fn fetch(&mut self) -> I;

    /// Attempts to execute an instruction  
    /// Returns None if the instruction isn't supported
    #[must_use]
    fn execute(&mut self, ins: u32, data: &mut D) -> Option<()>;

    /// Sets the value in register `i`
    fn set(&mut self, i: usize, value: I);

    /// Gets the value in register `i`
    fn get(&self, i: usize) -> I;
}

pub trait Extension {
    /// Attempts to execute an instruction
    /// Returns None if the instruction isn't supported
    #[must_use]
    fn execute<D>(&mut self, ins: u32, data: &mut D) -> Option<()>;

    /// Sets the value in register `i`  
    /// if this extension has registers
    fn set<T>(&mut self, i: usize, value: T) {}

    /// Gets the value is register `i`  
    /// if this extension has registers
    fn get<T>(&self, i: usize, value: T) {}
}
