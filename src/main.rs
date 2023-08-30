mod instruction_set;

use std::env;
use std::fs::{self, File};
use std::io::{Result, Write};
use std::path::Path;

use crate::instruction_set::X86Instruction;

fn main() -> Result<()> {
    let args: Vec<String> = env::args().collect();

    // Expecting the input file as the first argument and output file as the second
    // If output file is not provided, using the input file name with .asm extension
    let input_filename = args
        .get(1)
        .ok_or("Input file name is required")
        .map_err(|e| std::io::Error::new(std::io::ErrorKind::InvalidInput, e))?;
    let output_filename = if let Some(output_arg) = args.get(2) {
        output_arg.clone()
    } else {
        let mut output = input_filename.clone();
        output.push_str(".asm");
        output
    };
    let input_path = Path::new(input_filename);
    let listing = disassemble(input_path)?;
    println!("{}", listing);
    // Writing to a file
    let mut file = File::create(output_filename)?;
    file.write_all(listing.as_bytes())?;

    Ok(())
}

fn disassemble(file: &Path) -> Result<String> {
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
        assert_eq!(
            normalized_fun_name,
            normalized_stripped_asm_content
        );
    }

    #[test]
    fn test_many_register() {
        let asm_path = Path::new(MANY_ASM);
        let asm_content = fs::read_to_string(asm_path).unwrap();
        let normalized_stripped_asm_content = preprocess_listing(&asm_content);
        let path = Path::new(MANY);
        let normalized_fun_name = disassemble(path).unwrap();
        assert_eq!(
            normalized_fun_name,
            normalized_stripped_asm_content
        );
    }
}
