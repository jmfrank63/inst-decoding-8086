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
    fn test_error_conversion() {
        let custom_error = X86InstructionError::InvalidInstruction; // Replace with an actual error variant from your X86InstructionError enum
        let io_error: io::Error = custom_error.into();
        assert_eq!(io_error.kind(), io::ErrorKind::Other);
        assert_eq!(
            io_error
                .get_ref()
                .unwrap()
                .downcast_ref::<X86InstructionError>()
                .unwrap(),
            &X86InstructionError::InvalidInstruction
        );
    }

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
