use std::fmt;

#[derive(Debug, PartialEq, Eq)]
pub enum X86Opcode {
    Mov, // 100010 in binary
    InvalidInstruction,
}

impl X86Opcode {
    pub fn from_u8(value: u8) -> Self {
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
