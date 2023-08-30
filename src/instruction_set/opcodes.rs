use std::fmt;

#[repr(u8)]
#[derive(Debug, PartialEq, Eq)]
pub(crate) enum X86Opcode {
    Mov = 0b100010,
    InvalidInstruction,
}

impl X86Opcode {
    pub(crate) fn from_u8(value: u8) -> Self {
        match value {
            0b100010 => X86Opcode::Mov,
            _ => X86Opcode::InvalidInstruction,
        }
    }
}

impl fmt::Display for X86Opcode {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        let value = match self {
            X86Opcode::Mov => "mov",
            X86Opcode::InvalidInstruction => "invalid instruction",
        };
        write!(f, "{}", value)
    }
}

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn test_from_u8() {
        assert_eq!(X86Opcode::from_u8(0b100010), X86Opcode::Mov);
        assert_eq!(X86Opcode::from_u8(0b100011), X86Opcode::InvalidInstruction);
    }

    #[test]
    fn test_fmt() {
        assert_eq!(format!("{}", X86Opcode::Mov), "mov");
        assert_eq!(
            format!("{}", X86Opcode::InvalidInstruction),
            "invalid instruction"
        );
    }

    #[test]
    fn test_debug_trait() {
        // Create instances
        let mov = X86Opcode::Mov;
        let invalid_instruction = X86Opcode::InvalidInstruction;

        // Format instances using debug formatting
        let mov_debug_string = format!("{:?}", mov);
        let invalid_instruction_debug_string = format!("{:?}", invalid_instruction);

        // Assertions
        assert_eq!(mov_debug_string, "Mov"); // Compare against expected output
        assert_eq!(invalid_instruction_debug_string, "InvalidInstruction"); // Compare against expected output
    }

    #[test]
    fn test_opcode_from_u8() {
        // Test known opcode 100010 in binary
        let opcode = X86Opcode::from_u8(0b100010);
        assert_eq!(opcode, X86Opcode::Mov);

        // Test unknown opcode
        let invalid_opcode = X86Opcode::from_u8(0b111111);
        assert_eq!(invalid_opcode, X86Opcode::InvalidInstruction);
    }
}
