
use std::fmt;

use super::{bit::Bit, errors::X86InstructionError};

#[derive(Debug, PartialEq, Eq)]
pub enum X86Register {
    AL,
    CL,
    DL,
    BL,
    AH,
    CH,
    DH,
    BH,
    AX,
    CX,
    DX,
    BX,
    SP,
    BP,
    SI,
    DI,
}

impl fmt::Display for X86Register {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        let value = match self {
            X86Register::AL => "al",
            X86Register::CL => "cl",
            X86Register::DL => "dl",
            X86Register::BL => "bl",
            X86Register::AH => "ah",
            X86Register::CH => "ch",
            X86Register::DH => "dh",
            X86Register::BH => "bh",
            X86Register::AX => "ax",
            X86Register::CX => "cx",
            X86Register::DX => "dx",
            X86Register::BX => "bx",
            X86Register::SP => "sp",
            X86Register::BP => "bp",
            X86Register::SI => "si",
            X86Register::DI => "di",
        };
        write!(f, "{}", value)
    }
}

impl X86Register {
    // Constructs an X86Register from the W and reg fields.
    // Here `w` is assumed to be either 0 or 1, and `reg` is assumed to be a value from 0 to 7.
    pub fn from_w_and_field(w: Bit, field: u8) -> Result<Self, X86InstructionError> {
        match (w, field) {
            (Bit(false), 0b000) => Ok(X86Register::AL),
            (Bit(false), 0b001) => Ok(X86Register::CL),
            (Bit(false), 0b010) => Ok(X86Register::DL),
            (Bit(false), 0b011) => Ok(X86Register::BL),
            (Bit(false), 0b100) => Ok(X86Register::AH),
            (Bit(false), 0b101) => Ok(X86Register::CH),
            (Bit(false), 0b110) => Ok(X86Register::DH),
            (Bit(false), 0b111) => Ok(X86Register::BH),
            (Bit(true), 0b000) => Ok(X86Register::AX),
            (Bit(true), 0b001) => Ok(X86Register::CX),
            (Bit(true), 0b010) => Ok(X86Register::DX),
            (Bit(true), 0b011) => Ok(X86Register::BX),
            (Bit(true), 0b100) => Ok(X86Register::SP),
            (Bit(true), 0b101) => Ok(X86Register::BP),
            (Bit(true), 0b110) => Ok(X86Register::SI),
            (Bit(true), 0b111) => Ok(X86Register::DI),
            (_, _) => Err(X86InstructionError::InvalidRegister),
        }
    }
}
