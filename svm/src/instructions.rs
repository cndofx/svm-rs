// I/O
pub const IN: i32 = -1;
pub const OUT: i32 = -2;

// Arithmetic
pub const ADD: i32 = -3;
pub const SUB: i32 = -4;
pub const MUL: i32 = -5;
pub const DIV: i32 = -6;
pub const MOD: i32 = -7;
pub const NEG: i32 = -8;
pub const INC: i32 = -9;
pub const DEC: i32 = -10;

// Bitwise operations
pub const AND: i32 = -11;
pub const OR: i32 = -12;
pub const NOT: i32 = -13;
pub const XOR: i32 = -14;
pub const SHL: i32 = -15;
pub const SHR: i32 = -16;

// Stack
pub const POP: i32 = -17;
pub const DUP: i32 = -18;
pub const SWP: i32 = -19;
pub const OVR: i32 = -20;

// Memory
pub const LOAD: i32 = -21;
pub const STOR: i32 = -22;

// Jumps
pub const JMP: i32 = -23;
pub const JE: i32 = -24;
pub const JNE: i32 = -25;
pub const JG: i32 = -26;
pub const JGE: i32 = -27;
pub const JL: i32 = -28;
pub const JLE: i32 = -29;

pub const NOP: i32 = -30;
pub const HALT: i32 = -31;

// Flags
pub const RF: i32 = -101;
pub const CRF: i32 = -102;
