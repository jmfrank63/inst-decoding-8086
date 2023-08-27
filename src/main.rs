mod instruction_set;

use std::fs::{self};
use std::io::{Result};
use std::path::Path;

use crate::instruction_set::X86Instruction;

const DATA : &str = "data";
const SINGLE: &str = "listing_0037_single_register_mov";
const MANY: &str = "listing_0038_many_register_mov";

fn main() -> Result<()> {
    for filename in [SINGLE, MANY] {
        let listing = disassemble(DATA, filename)?;
        println!("{}", listing);
    }
    Ok(())
}

fn disassemble(directory: &str, filename: &str) -> Result<String> {
    let directory = Path::new(directory);
    let mut listing = String::from("bits 16\n");
    let path = directory.join(filename);
    let buffer: Vec<u8> = fs::read(path)?;
    for chunk in buffer.chunks(2) {
        let instruction = X86Instruction::new([chunk[0], chunk[1]]);
        match instruction.format_instruction() {
            Ok(formatted_instruction) => {
                listing.push_str(&formatted_instruction);
                listing.push('\n');
            }
            _ => {
                println!("Invalid instruction");
            }
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

    fn read_and_strip_asm(asm_path: &Path) -> Result<String> {
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
        let normalized_fun_name = disassemble(DATA, SINGLE).unwrap().replace("\r\n", "\n");
        assert_eq!(normalized_fun_name.trim(), normalized_stripped_asm_content.trim());
    }

    #[test]
    fn test_many_register() {
        let asm_path = Path::new(MANY_ASM);
        let normalized_stripped_asm_content = read_and_strip_asm(asm_path).unwrap().replace("\r\n", "\n");
        let normalized_fun_name = disassemble(DATA, MANY).unwrap().replace("\r\n", "\n");
        assert_eq!(normalized_fun_name.trim(), normalized_stripped_asm_content.trim());
    }
}
