use super::{bit::Bit, errors::X86InstructionError, opcodes::X86Opcode, registers::X86Register};

#[derive(Debug, Clone, Copy)]
pub struct X86Instruction {
    raw: u16,
}

impl X86Instruction {
    /// Create a new X86Instruction from a 4-byte array
    pub fn new(buffer: [u8; 2]) -> Self {
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
    pub fn reg_field(&self) -> u8 {
        ((self.raw & 0b00000000_00111000) >> 3) as u8
    }

    /// Get the 3-bit R/M field
    pub fn rm_field(&self) -> u8 {
        (self.raw & 0b00000000_00000111) as u8
    }

    pub fn format_instruction(&self) -> Result<String, X86InstructionError> {
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
    fn test_instruction_mov_ax_ax_reverse() {
        let inst = X86Instruction::new([0b10001011, 0b11000000]);
        assert_eq!(inst.format_instruction().unwrap(), "mov ax, ax");
    }

    #[test]
    fn test_instruction_mov_ax_ax() {
        let inst = X86Instruction::new([0b10001001, 0b11000000]);
        assert_eq!(inst.format_instruction().unwrap(), "mov ax, ax");
    }

    #[test]
    fn test_instruction_mov_ax_bx() {
        let inst = X86Instruction::new([0b10001011, 0b11000011]);
        assert_eq!(inst.format_instruction().unwrap(), "mov ax, bx");
    }

    #[test]
    fn test_instruction_mov_bx_ax() {
        // The d bit is 0 here (9th bit from the right)
        let inst = X86Instruction::new([0b10001001, 0b11000011]);
        assert_eq!(inst.format_instruction().unwrap(), "mov bx, ax");
    }

    #[test]
    fn test_instruction_mov_ax_cx() {
        let inst = X86Instruction::new([0b10001011, 0b11000001]);
        assert_eq!(inst.format_instruction().unwrap(), "mov ax, cx");
    }

    #[test]
    fn test_instruction_mov_cx_ax() {
        // The d bit is 0 here (9th bit from the right)
        let inst = X86Instruction::new([0b10001001, 0b11000001]);
        assert_eq!(inst.format_instruction().unwrap(), "mov cx, ax");
    }

    #[test]
    fn test_instruction_mov_ax_dx() {
        let inst = X86Instruction::new([0b10001011, 0b11000010]);
        assert_eq!(inst.format_instruction().unwrap(), "mov ax, dx");
    }

    #[test]
    fn test_instruction_mov_dx_ax() {
        // The d bit is 0 here (9th bit from the right)
        let inst = X86Instruction::new([0b10001001, 0b11000010]);
        assert_eq!(inst.format_instruction().unwrap(), "mov dx, ax");
    }

    #[test]
fn test_instruction_mov_al_al_reverse() {
    let inst = X86Instruction::new([0b10001010, 0b11000000]);
    assert_eq!(inst.format_instruction().unwrap(), "mov al, al");
}

#[test]
fn test_instruction_mov_al_al() {
    let inst = X86Instruction::new([0b10001000, 0b11000000]);
    assert_eq!(inst.format_instruction().unwrap(), "mov al, al");
}

#[test]
fn test_instruction_mov_al_bl() {
    let inst = X86Instruction::new([0b10001010, 0b11000011]);
    assert_eq!(inst.format_instruction().unwrap(), "mov al, bl");
}

#[test]
fn test_instruction_mov_bl_al() {
    // The d bit is 0 here (9th bit from the right)
    let inst = X86Instruction::new([0b10001000, 0b11000011]);
    assert_eq!(inst.format_instruction().unwrap(), "mov bl, al");
}

#[test]
fn test_instruction_mov_al_cl() {
    let inst = X86Instruction::new([0b10001010, 0b11000001]);
    assert_eq!(inst.format_instruction().unwrap(), "mov al, cl");
}

#[test]
fn test_instruction_mov_cl_al() {
    // The d bit is 0 here (9th bit from the right)
    let inst = X86Instruction::new([0b10001000, 0b11000001]);
    assert_eq!(inst.format_instruction().unwrap(), "mov cl, al");
}

#[test]
fn test_instruction_mov_al_dl() {
    let inst = X86Instruction::new([0b10001010, 0b11000010]);
    assert_eq!(inst.format_instruction().unwrap(), "mov al, dl");
}

#[test]
fn test_instruction_mov_dl_al() {
    // The d bit is 0 here (9th bit from the right)
    let inst = X86Instruction::new([0b10001000, 0b11000010]);
    assert_eq!(inst.format_instruction().unwrap(), "mov dl, al");
}

// For bx to bx
#[test]
fn test_instruction_mov_bx_bx_reverse() {
    let inst = X86Instruction::new([0b10001011, 0b11011011]);
    assert_eq!(inst.format_instruction().unwrap(), "mov bx, bx");
}

#[test]
fn test_instruction_mov_bx_bx() {
    let inst = X86Instruction::new([0b10001001, 0b11011011]);
    assert_eq!(inst.format_instruction().unwrap(), "mov bx, bx");
}

// For bx to cx
#[test]
fn test_instruction_mov_bx_cx() {
    let inst = X86Instruction::new([0b10001011, 0b11011001]);
    assert_eq!(inst.format_instruction().unwrap(), "mov bx, cx");
}

#[test]
fn test_instruction_mov_cx_bx() {
    let inst = X86Instruction::new([0b10001001, 0b11011001]);
    assert_eq!(inst.format_instruction().unwrap(), "mov cx, bx");
}

// For bx to dx
#[test]
fn test_instruction_mov_bx_dx() {
    let inst = X86Instruction::new([0b10001011, 0b11011010]);
    assert_eq!(inst.format_instruction().unwrap(), "mov bx, dx");
}

#[test]
fn test_instruction_mov_dx_bx() {
    let inst = X86Instruction::new([0b10001001, 0b11011010]);
    assert_eq!(inst.format_instruction().unwrap(), "mov dx, bx");
}

// For low 8-bits
#[test]
fn test_instruction_mov_bl_cl() {
    let inst = X86Instruction::new([0b10001010, 0b11011001]);
    assert_eq!(inst.format_instruction().unwrap(), "mov bl, cl");
}

#[test]
fn test_instruction_mov_cl_bl() {
    let inst = X86Instruction::new([0b10001000, 0b11011001]);
    assert_eq!(inst.format_instruction().unwrap(), "mov cl, bl");
}

#[test]
fn test_instruction_mov_bl_dl() {
    let inst = X86Instruction::new([0b10001010, 0b11011010]);
    assert_eq!(inst.format_instruction().unwrap(), "mov bl, dl");
}

#[test]
fn test_instruction_mov_dl_bl() {
    let inst = X86Instruction::new([0b10001000, 0b11011010]);
    assert_eq!(inst.format_instruction().unwrap(), "mov dl, bl");
}

// For cx to cx
#[test]
fn test_instruction_mov_cx_cx_reverse() {
    let inst = X86Instruction::new([0b10001011, 0b11001001]);
    assert_eq!(inst.format_instruction().unwrap(), "mov cx, cx");
}

#[test]
fn test_instruction_mov_cx_cx() {
    let inst = X86Instruction::new([0b10001001, 0b11001001]);
    assert_eq!(inst.format_instruction().unwrap(), "mov cx, cx");
}

// For low 8-bits
#[test]
fn test_instruction_mov_cl_cl() {
    let inst = X86Instruction::new([0b10001010, 0b11001001]);
    assert_eq!(inst.format_instruction().unwrap(), "mov cl, cl");
}

#[test]
fn test_instruction_mov_cl_cl_reverse() {
    let inst = X86Instruction::new([0b10001000, 0b11001001]);
    assert_eq!(inst.format_instruction().unwrap(), "mov cl, cl");
}

// For dx to dx
#[test]
fn test_instruction_mov_dx_dx_reverse() {
    let inst = X86Instruction::new([0b10001011, 0b11010010]);
    assert_eq!(inst.format_instruction().unwrap(), "mov dx, dx");
}

#[test]
fn test_instruction_mov_dx_dx() {
    let inst = X86Instruction::new([0b10001001, 0b11010010]);
    assert_eq!(inst.format_instruction().unwrap(), "mov dx, dx");
}

// For low 8-bit dl to dl
#[test]
fn test_instruction_mov_dl_dl_reverse() {
    let inst = X86Instruction::new([0b10001000, 0b11010010]);
    assert_eq!(inst.format_instruction().unwrap(), "mov dl, dl");
}

#[test]
fn test_instruction_mov_dl_dl() {
    let inst = X86Instruction::new([0b10001010, 0b11010010]);
    assert_eq!(inst.format_instruction().unwrap(), "mov dl, dl");
}

// For AH to AH
#[test]
fn test_instruction_mov_ah_ah() {
    let inst = X86Instruction::new([0b10001000, 0b11100100]);
    assert_eq!(inst.format_instruction().unwrap(), "mov ah, ah");
}

#[test]
fn test_instruction_mov_ah_ah_reverse() {
    let inst = X86Instruction::new([0b10001010, 0b11100100]);
    assert_eq!(inst.format_instruction().unwrap(), "mov ah, ah");
}

// For CH to CH
#[test]
fn test_instruction_mov_ch_ch() {
    let inst = X86Instruction::new([0b10001000, 0b11101101]);
    assert_eq!(inst.format_instruction().unwrap(), "mov ch, ch");
}

#[test]
fn test_instruction_mov_ch_ch_reverse() {
    let inst = X86Instruction::new([0b10001010, 0b11101101]);
    assert_eq!(inst.format_instruction().unwrap(), "mov ch, ch");
}

// For DH to DH
#[test]
fn test_instruction_mov_dh_dh() {
    let inst = X86Instruction::new([0b10001000, 0b11110110]);
    assert_eq!(inst.format_instruction().unwrap(), "mov dh, dh");
}

#[test]
fn test_instruction_mov_dh_dh_reverse() {
    let inst = X86Instruction::new([0b10001010, 0b11110110]);
    assert_eq!(inst.format_instruction().unwrap(), "mov dh, dh");
}

// For BH to BH
#[test]
fn test_instruction_mov_bh_bh() {
    let inst = X86Instruction::new([0b10001000, 0b11111111]);
    assert_eq!(inst.format_instruction().unwrap(), "mov bh, bh");
}

#[test]
fn test_instruction_mov_bh_bh_reverse() {
    let inst = X86Instruction::new([0b10001010, 0b11111111]);
    assert_eq!(inst.format_instruction().unwrap(), "mov bh, bh");
}

// For AH to CH
#[test]
fn test_instruction_mov_ah_ch() {
    let inst = X86Instruction::new([0b10001010, 0b11100101]);
    assert_eq!(inst.format_instruction().unwrap(), "mov ah, ch");
}

#[test]
fn test_instruction_mov_ch_ah() {
    let inst = X86Instruction::new([0b10001000, 0b11100101]);
    assert_eq!(inst.format_instruction().unwrap(), "mov ch, ah");
}

// For AH to DH
#[test]
fn test_instruction_mov_ah_dh() {
    let inst = X86Instruction::new([0b10001010, 0b11100110]);
    assert_eq!(inst.format_instruction().unwrap(), "mov ah, dh");
}

#[test]
fn test_instruction_mov_dh_ah() {
    let inst = X86Instruction::new([0b10001000, 0b11100110]);
    assert_eq!(inst.format_instruction().unwrap(), "mov dh, ah");
}

// For AH to BH
#[test]
fn test_instruction_mov_ah_bh() {
    let inst = X86Instruction::new([0b10001010, 0b11100111]);
    assert_eq!(inst.format_instruction().unwrap(), "mov ah, bh");
}

#[test]
fn test_instruction_mov_bh_ah() {
    let inst = X86Instruction::new([0b10001000, 0b11100111]);
    assert_eq!(inst.format_instruction().unwrap(), "mov bh, ah");
}

// For AH to AL
#[test]
fn test_instruction_mov_ah_al() {
    let inst = X86Instruction::new([0b10001010, 0b11100000]);
    assert_eq!(inst.format_instruction().unwrap(), "mov ah, al");
}

#[test]
fn test_instruction_mov_al_ah() {
    let inst = X86Instruction::new([0b10001000, 0b11100000]);
    assert_eq!(inst.format_instruction().unwrap(), "mov al, ah");
}

// For AH to BL
#[test]
fn test_instruction_mov_ah_bl() {
    let inst = X86Instruction::new([0b10001010, 0b11100011]);
    assert_eq!(inst.format_instruction().unwrap(), "mov ah, bl");
}

#[test]
fn test_instruction_mov_bl_ah() {
    let inst = X86Instruction::new([0b10001000, 0b11100011]);
    assert_eq!(inst.format_instruction().unwrap(), "mov bl, ah");
}

// For AH to CL
#[test]
fn test_instruction_mov_ah_cl() {
    let inst = X86Instruction::new([0b10001010, 0b11100001]);
    assert_eq!(inst.format_instruction().unwrap(), "mov ah, cl");
}

#[test]
fn test_instruction_mov_cl_ah() {
    let inst = X86Instruction::new([0b10001000, 0b11100001]);
    assert_eq!(inst.format_instruction().unwrap(), "mov cl, ah");
}

// For AH to DL
#[test]
fn test_instruction_mov_ah_dl() {
    let inst = X86Instruction::new([0b10001010, 0b11100010]);
    assert_eq!(inst.format_instruction().unwrap(), "mov ah, dl");
}

#[test]
fn test_instruction_mov_dl_ah() {
    let inst = X86Instruction::new([0b10001000, 0b11100010]);
    assert_eq!(inst.format_instruction().unwrap(), "mov dl, ah");
}

}
