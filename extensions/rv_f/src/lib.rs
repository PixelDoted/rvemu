mod rv32;

pub use rv32::{RV32FData, RV32F};
use rvcore::ins::{IType, RType, SType};

const OPCODE_LOADF: u32 = 0b0000111;
const OPCODE_STOREF: u32 = 0b0100111;
const OPCODE_OPFP: u32 = 0b1010011;

type TypeLoadF = IType;
type TypeStoreF = SType;
type TypeOpFp = RType;
