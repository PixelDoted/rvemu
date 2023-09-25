pub mod ins;
mod memory;
pub mod util;

pub type QUADWORD = i128;
pub type DOUBLEWORD = i64;
pub type WORD = i32;
pub type HALFWORLD = i16;

pub use memory::Memory;

pub trait Base<T>: Volatile<T> {
    /// Fetches the current `program counter`
    fn fetch(&mut self) -> T;

    /// Attempts to execute an instruction  
    /// Returns None if the instruction isn't supported
    #[must_use]
    fn execute(&mut self, ins: u32, memory: &mut Memory) -> Option<()>;
}

pub trait Extension<D> {
    /// Attempts to execute an instruction
    /// Returns None if the instruction isn't supported
    #[must_use]
    fn execute(&mut self, ins: u32, data: &mut D) -> Option<()>;
}

pub trait Volatile<T> {
    fn set(&mut self, index: usize, value: T);
    fn get(&self, index: usize) -> T;
}
