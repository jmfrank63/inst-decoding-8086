use super::{bit::Bit, errors::X86InstructionError, opcodes::X86Opcode, registers::X86Register};

#[derive(Debug, Clone, Copy)] // GRCOV_EXCL_LINE
pub struct X86Instruction {
    raw: u16,
}

impl X86Instruction {
    /// Create a new X86Instruction from a 4-byte array
    pub(crate) fn new(buffer: [u8; 2]) -> Self {
        let value = u16::from_be_bytes(buffer);
        X86Instruction { raw: value }
    }

    /// Get the 6-bit opcode
    fn opcode(&self) -> X86Opcode {
        X86Opcode::from_u8(((self.raw & 0b11111100_00000000) >> 10) as u8)
    }

    /// Get the 1-bit D field
    fn d_bit(&self) -> Bit {
        Bit(((self.raw & 0b00000010_00000000) >> 9) == 1)
    }

    /// Get the 1-bit W field
    fn w_bit(&self) -> Bit {
        Bit(((self.raw & 0b00000001_00000000) >> 8) == 1)
    }

    /// Get the 2-bit mod field
    fn mod_field(&self) -> u8 {
        ((self.raw & 0b00000000_11000000) >> 6) as u8
    }

    /// Get the 3-bit reg field
    fn reg_field(&self) -> u8 {
        ((self.raw & 0b00000000_00111000) >> 3) as u8
    }

    /// Get the 3-bit R/M field
    fn rm_field(&self) -> u8 {
        (self.raw & 0b00000000_00000111) as u8
    }

    pub(crate) fn format_instruction(&self) -> Result<String, X86InstructionError> {
        let opcode = self.opcode();

        let op = match opcode {
            X86Opcode::Mov => format!("{}", X86Opcode::Mov),
            X86Opcode::InvalidInstruction => return Err(X86InstructionError::InvalidInstruction),
        };
        let w = self.w_bit();
        let d = self.d_bit();
        let mod_field = self.mod_field();
        let reg_field = self.reg_field();
        let rm_field = self.rm_field();

        let reg = X86Register::from_w_and_field(w, reg_field)?;
        let rm = X86Register::from_w_and_field(w, rm_field)?;
        if mod_field != 0b11 {
            return Err(X86InstructionError::InvalidInstruction);
        }
        Ok(if d == Bit(true) {
            format!("{} {}, {}", op, reg, rm)
        } else {
            format!("{} {}, {}", op, rm, reg)
        })
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_opcode_extraction() {
        let inst = X86Instruction::new([0b10001011, 0b11111111]);
        assert_eq!(inst.opcode(), X86Opcode::Mov);
    }

    #[test]
    fn test_d_bit_extraction() {
        let inst = X86Instruction::new([0b00000010, 0b00000000]);
        assert_eq!(inst.d_bit(), Bit(true));
    }

    #[test]
    fn test_w_bit_extraction() {
        let inst = X86Instruction::new([0b00000001, 0b00000000]);
        assert_eq!(inst.w_bit(), Bit(true));
    }

    #[test]
    fn test_mod_field_extraction() {
        let inst = X86Instruction::new([0b00000000, 0b11000000]);
        assert_eq!(inst.mod_field(), 0b11);
    }

    #[test]
    fn test_reg_field_extraction() {
        let inst = X86Instruction::new([0b00000000, 0b00111000]);
        assert_eq!(inst.reg_field(), 0b111);
    }

    #[test]
    fn test_rm_field_extraction() {
        let inst = X86Instruction::new([0b00000000, 0b00000111]);
        assert_eq!(inst.rm_field(), 0b111);
    }

    #[test]
    fn test_invalid_instruction() {
        let inst = X86Instruction::new([0b10000000, 0b00000000]);
        assert_eq!(
            inst.format_instruction(),
            Err(X86InstructionError::InvalidInstruction)
        );
    }

    #[test]
    fn test_format_instruction_invalid_mod_field() {
        // Construct an X86Instruction with a mod_field other than 0b11
        let src = 0b000;
        let dest = 0b000;
        let mod_bits = 0b01;
        let opcode = 0b100010;
        let first_byte = opcode << 2;
        let second_byte = (mod_bits << 6) | (src << 3) | dest;
        let instruction = X86Instruction::new([first_byte, second_byte]);
        let result = instruction.format_instruction();
        assert_eq!(result, Err(X86InstructionError::InvalidInstruction));
    }

    #[test]
    fn test_instruction_mov_all_combinations() {
        let mut count = 0;
        for src in 0b000..=0b111 {
            for dest in 0b000..=0b111 {
                for d in 0..=1u8 {
                    for w_msb in 0..=1 {
                        count += 1;
                        let w = Bit(w_msb == 1);
                        let mod_bits = 0b11;
                        let opcode = 0b100010;

                        let first_byte = (opcode << 2) | (d << 1) | w.0 as u8;
                        let second_byte = (mod_bits << 6) | (src << 3) | dest;

                        let inst = X86Instruction::new([first_byte, second_byte]);

                        // Extract the string representation for the src and dest based on the W bit.
                        let src_register = X86Register::from_w_and_field(w, src).unwrap();
                        let dest_register = X86Register::from_w_and_field(w, dest).unwrap();
                        let mnemonic = inst.format_instruction().unwrap();
                        if d == 0 {
                            assert_eq!(
                                mnemonic,
                                format!("mov {}, {}", dest_register, src_register)
                            );
                        } else {
                            assert_eq!(
                                mnemonic,
                                format!("mov {}, {}", src_register, dest_register)
                            );
                        }
                    }
                }
            }
        }
        assert_eq!(count, 8 * 8 * 2 * 2); // all registers with combined with each other 8 * 8 , 2 for d bit, 2 for w bit
    }
}
