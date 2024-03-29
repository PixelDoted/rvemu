mod types;

pub use types::*;

pub const OPCODE_MASK: u32 = 0b1111111;

pub const OPCODE_OPIMM: u32 = 0b0010011;
pub const OPCODE_LUI: u32 = 0b0110111;
pub const OPCODE_AUIPC: u32 = 0b0010111;
pub const OPCODE_OP: u32 = 0b0110011;
pub const OPCODE_JAL: u32 = 0b1101111;
pub const OPCODE_JALR: u32 = 0b1100111;
pub const OPCODE_BRANCH: u32 = 0b1100011;
pub const OPCODE_LOAD: u32 = 0b0000011;
pub const OPCODE_STORE: u32 = 0b0100011;
pub const OPCODE_SYSTEM: u32 = 0b1110011;
pub const OPCODE_MISCMEM: u32 = 0b0000110;

pub type TypeOpImm = IType;
pub type TypeLui = UType;
pub type TypeAuiPc = UType;
pub type TypeOp = RType;
pub type TypeJal = JType;
pub type TypeJalR = IType;
pub type TypeBranch = BType;
pub type TypeLoad = IType;
pub type TypeStore = SType;
pub type TypeSystem = IType;
pub type TypeMiscMem = IType;
