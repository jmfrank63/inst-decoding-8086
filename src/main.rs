use std::error::Error;
use std::fmt;
use std::fs::File;
use std::io::{self, Read};
use std::path::Path;

#[derive(Debug, PartialEq, Eq)]
enum X86Register {
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

#[derive(Debug)]
enum X86InstructionError {
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

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct Bit(bool);

impl fmt::Binary for Bit {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        let value = if self.0 { 1 } else { 0 };
        write!(f, "{}", value)
    }
}

impl fmt::Display for Bit {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        fmt::Binary::fmt(self, f)
    }
}

#[derive(Debug, Clone, Copy)]
pub struct X86Instruction {
    raw: u16,
}

impl X86Instruction {
    /// Create a new X86Instruction from a 16-bit integer
    pub fn from_u16(value: u16) -> Self {
        X86Instruction { raw: value }
    }

    /// Create a new X86Instruction from two 8-bit bytes
    pub fn from_bytes(byte1: u8, byte2: u8) -> Self {
        // Combine the two 8-bit bytes into a single 16-bit integer
        let combined = ((byte2 as u16) << 8) | (byte1 as u16);
        X86Instruction { raw: combined }
    }

    /// Get the 6-bit opcode
    pub fn opcode(&self) -> X86Opcode {
        X86Opcode::from_u8(((self.raw & 0b11111100_00000000) >> 10) as u8)
    }

    /// Get the 1-bit D field
    pub fn d_bit(&self) -> Bit {
        Bit(((self.raw & 0b00000010_00000000) >> 9) == 1)
    }

    /// Get the 1-bit W field
    pub fn w_bit(&self) -> Bit {
        Bit(((self.raw & 0b00000001_00000000) >> 8) == 1)
    }

    /// Get the 2-bit mod field
    pub fn mod_field(&self) -> u8 {
        ((self.raw & 0b00000000_11000000) >> 6) as u8
    }

    /// Get the 3-bit reg field
    pub fn reg_field(&self) -> u8 {
        ((self.raw & 0b00000000_00111000) >> 3) as u8
    }

    /// Get the 3-bit R/M field
    pub fn rm_field(&self) -> u8 {
        (self.raw & 0b00000000_00000111) as u8
    }

    fn format_instruction(&self) -> Result<String, X86InstructionError> {
        let opcode = self.opcode();

        let op = match opcode {
            X86Opcode::Mov => format!("{}", X86Opcode::Mov),
            X86Opcode::InvalidInstruction => return Err(X86InstructionError::InvalidInstruction),
        };
        let w = self.w_bit();
        let d = self.d_bit();
        let reg_field = self.reg_field();
        let rm_field = self.rm_field();

        let reg = X86Register::from_w_and_field(w, reg_field)?;
        let rm = X86Register::from_w_and_field(w, rm_field)?;

        Ok(if d == Bit(true) {
            format!("{} {}, {}", op, reg, rm)
        } else {
            format!("{} {}, {}", op, rm, reg)
        })
    }
}

const SINGLE: &str = "data/listing_0037_single_register_mov";
const MANY: &str = "data/listing_0038_many_register_mov";

fn main() -> io::Result<()> {
    let listing = fun_name(SINGLE)?;
    println!("{}", listing);
    println!();
    let listing = fun_name(MANY)?;
    println!("{}", listing);
    Ok(())
}

fn fun_name(binary: &str) -> Result<String, io::Error> {
    let path = Path::new(binary);
    let mut file = File::open(path)?;
    let mut buffer = [0; 2];
    let mut listing = String::new();
    listing.push_str("bits 16\n");

    while file.read(&mut buffer)? > 0 {
        let value = u16::from_be_bytes(buffer);
        let instruction = X86Instruction::from_u16(value);
        match instruction.format_instruction() {
            Ok(formatted_instruction) => {
                listing.push_str(&formatted_instruction);
                listing.push('\n');
            }
            Err(e) => return Err(io::Error::from(e)),
        }
    }
    Ok(listing)
}

#[cfg(test)]
mod tests {
    use super::*;
    use std::fs;
    const SINGLE_ASM: &str = "data/listing_0037_single_register_mov.asm";
    const MANY_ASM: &str = "data/listing_0038_many_register_mov.asm";

    fn read_and_strip_asm(asm_path: &Path) -> io::Result<String> {
        let asm_content = fs::read_to_string(asm_path)?;
        Ok(asm_content
            .lines()
            .filter(|line| {
                !line.starts_with(';') && !line.is_empty() && !line.starts_with("bit 16")
            })
            .collect::<Vec<_>>()
            .join("\n"))
    }
    #[test]
    fn test_single_register() {
        let asm_path = Path::new(SINGLE_ASM);
        let normalized_stripped_asm_content = read_and_strip_asm(asm_path).unwrap().replace("\r\n", "\n");
        let normalized_fun_name = fun_name(SINGLE).unwrap().replace("\r\n", "\n");
        assert_eq!(normalized_fun_name.trim(), normalized_stripped_asm_content.trim());
    }

    #[test]
    fn test_many_register() {
        let asm_path = Path::new(MANY_ASM);
        let normalized_stripped_asm_content = read_and_strip_asm(asm_path).unwrap().replace("\r\n", "\n");
        let normalized_fun_name = fun_name(MANY).unwrap().replace("\r\n", "\n");
        assert_eq!(normalized_fun_name.trim(), normalized_stripped_asm_content.trim());
    }
}
