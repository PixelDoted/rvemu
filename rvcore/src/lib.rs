pub mod bus;
mod dram;
pub mod ins;
pub mod util;

pub type QUADWORD = i128;
pub type DOUBLEWORD = i64;
pub type WORD = i32;
pub type HALFWORLD = i16;

pub use dram::DRam;

// ---- Base ----

pub trait Base<T>: Volatile<T> {
    /// Fetches the current `program counter`
    fn fetch(&mut self) -> T;

    /// Attempts to execute an instruction  
    /// Returns None if the instruction isn't supported
    #[must_use]
    fn execute(&mut self, ins: u32) -> EResult;
}

// ---- Extension ----
pub trait Extension<B> {
    /// Attempts to execute an instruction
    /// Returns None if the instruction isn't supported
    #[must_use]
    fn execute(&mut self, ins: u32, base: &mut B) -> EResult;
}

// ---- Volatile ----

pub trait Volatile<T> {
    fn set(&mut self, index: usize, value: T);
    fn get(&self, index: usize) -> T;
}

// ---- Result ----

/// Execution Result
pub enum EResult {
    NotFound,
    Found,
    ECall,
    EBreak,
}
