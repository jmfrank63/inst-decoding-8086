mod instruction_set;

use std::fs;
use std::io::Result;
use std::path::Path;

use crate::instruction_set::X86Instruction;

pub fn disassemble(file: &Path) -> Result<String> {
    let mut listing = String::from("bits 16\n");

    let buffer: Vec<u8> = fs::read(file)?;
    for chunk in buffer.chunks(2) {
        let instruction = X86Instruction::new([chunk[0], chunk[1]]);
        match instruction.format_instruction() {
            Ok(formatted_instruction) => {
                listing.push_str(&formatted_instruction);
                listing.push('\n');
            }
            _ => println!("Invalid instruction"), // GRCOV_EXCL_LINE
        }
    }
    Ok(listing.trim().into())
}

#[cfg(test)]
mod utils;
#[cfg(test)]
mod tests {

    use crate::utils::preprocess_listing;

    use super::*;
    use std::fs;
    const SINGLE: &str = "tests/test_data/listing_0037_single_register_mov";
    const MANY: &str = "tests/test_data/listing_0038_many_register_mov";
    const SINGLE_ASM: &str = "tests/test_data/listing_0037_single_register_mov.asm";
    const MANY_ASM: &str = "tests/test_data/listing_0038_many_register_mov.asm";

    #[test]
    fn test_single_register() {
        let asm_path = Path::new(SINGLE_ASM);
        let asm_content = fs::read_to_string(asm_path).unwrap();
        let normalized_stripped_asm_content = preprocess_listing(&asm_content);
        let path = Path::new(SINGLE);
        let normalized_fun_name = disassemble(path).unwrap();
        assert_eq!(normalized_fun_name, normalized_stripped_asm_content);
    }

    #[test]
    fn test_many_register() {
        let asm_path = Path::new(MANY_ASM);
        let asm_content = fs::read_to_string(asm_path).unwrap();
        let normalized_stripped_asm_content = preprocess_listing(&asm_content);
        let path = Path::new(MANY);
        let normalized_fun_name = disassemble(path).unwrap();
        assert_eq!(normalized_fun_name, normalized_stripped_asm_content);
    }
}
