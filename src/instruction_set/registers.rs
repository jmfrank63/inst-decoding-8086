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
    InvalidRegister,
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
            X86Register::InvalidRegister => "invalid",
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
            (_, _) => {
                Err(X86Register::InvalidRegister).map_err(|_| X86InstructionError::InvalidRegister)
            }
        }
    }
}

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn test_from_w_and_field_w0() {
        // Test for W bit false (8-bit registers)
        for field in 0b000..=0b111 {
            let w = Bit(false);
            let register = X86Register::from_w_and_field(w, field).unwrap();

            // Determine the expected register enum variant based on the field
            let expected = match field {
                0b000 => X86Register::AL,
                0b001 => X86Register::CL,
                0b010 => X86Register::DL,
                0b011 => X86Register::BL,
                0b100 => X86Register::AH,
                0b101 => X86Register::CH,
                0b110 => X86Register::DH,
                0b111 => X86Register::BH,
                _ => X86Register::InvalidRegister,
            };

            assert_eq!(register, expected);
        }
    }

    #[test]
    fn test_from_w_and_field_w1() {
        // Test for W bit true (16-bit registers)
        for field in 0b000..=0b111 {
            let w = Bit(true);
            let register = X86Register::from_w_and_field(w, field).unwrap();

            // Determine the expected register enum variant based on the field
            let expected = match field {
                0b000 => X86Register::AX,
                0b001 => X86Register::CX,
                0b010 => X86Register::DX,
                0b011 => X86Register::BX,
                0b100 => X86Register::SP,
                0b101 => X86Register::BP,
                0b110 => X86Register::SI,
                0b111 => X86Register::DI,
                _ => panic!("Invalid field value!"), // GRCOV_EXCL_LINE
            };

            assert_eq!(register, expected);
        }
    }

    #[test]
    fn test_invalid_register() {
        let invalid_result = X86Register::from_w_and_field(Bit(false), 0b10101); // value not covered
        assert!(matches!(
            invalid_result,
            Err(X86InstructionError::InvalidRegister)
        ));

        let invalid_result = X86Register::from_w_and_field(Bit(true), 0b11000); // value not covered
        assert!(matches!(
            invalid_result,
            Err(X86InstructionError::InvalidRegister)
        ));
    }

    #[test]
    fn test_invalid_register_display() {
        let register = X86Register::InvalidRegister;
        let display_string = format!("{}", register);
        assert_eq!(display_string, "invalid");
    }
}
