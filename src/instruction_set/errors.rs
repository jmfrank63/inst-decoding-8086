use std::error::Error;
use std::fmt;
use std::io;

#[derive(Debug, PartialEq)]
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

    #[test]
    fn test_debug_trait() {
        let err = X86InstructionError::InvalidInstruction;
        assert_eq!(format!("{:?}", err), "InvalidInstruction");

        let err = X86InstructionError::InvalidRegister;
        assert_eq!(format!("{:?}", err), "InvalidRegister");
    }

    #[test]
    fn test_partial_eq_trait() {
        assert_eq!(
            X86InstructionError::InvalidInstruction,
            X86InstructionError::InvalidInstruction
        );
        assert_eq!(
            X86InstructionError::InvalidRegister,
            X86InstructionError::InvalidRegister
        );
        assert_ne!(
            X86InstructionError::InvalidInstruction,
            X86InstructionError::InvalidRegister
        );
    }

    #[test]
    fn test_from_trait_implementation() {
        let x86_error = X86InstructionError::InvalidInstruction;
        let io_error = io::Error::from(x86_error);
        assert_eq!(io_error.kind(), io::ErrorKind::Other);
        let inner = io_error.get_ref().unwrap();
        let inner_downcasted = inner.downcast_ref::<X86InstructionError>().unwrap();
        assert_eq!(inner_downcasted, &X86InstructionError::InvalidInstruction);
    }

    #[test]
    fn test_into_trait_implementation() {
        let x86_error = X86InstructionError::InvalidRegister;
        let io_error: io::Error = x86_error.into();
        assert_eq!(io_error.kind(), io::ErrorKind::Other);
        let inner = io_error.get_ref().unwrap();
        let inner_downcasted = inner.downcast_ref::<X86InstructionError>().unwrap();
        assert_eq!(inner_downcasted, &X86InstructionError::InvalidRegister);
    }
}
