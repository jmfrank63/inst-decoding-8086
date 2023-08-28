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

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn test_fmt() {
        assert_eq!(
            format!("{}", X86InstructionError::InvalidInstruction),
            "InvalidInstruction"
        );
        assert_eq!(
            format!("{}", X86InstructionError::InvalidRegister),
            "InvalidRegister"
        );
    }
}
