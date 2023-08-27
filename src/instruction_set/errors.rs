use std::error::Error;
use std::fmt;
use std::io;

#[derive(Debug, PartialEq, Eq)]
pub enum X86InstructionError {
    InvalidInstruction,
    InvalidRegister,
}

impl fmt::Display for X86InstructionError {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "{:?}", self)
    }
}

impl Error for X86InstructionError {}

impl From<X86InstructionError> for io::Error {
    fn from(error: X86InstructionError) -> Self {
        io::Error::new(io::ErrorKind::Other, error)
    }
}
