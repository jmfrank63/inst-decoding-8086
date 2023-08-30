mod instruction_set;
pub mod utils;

use std::fs;
use std::io::Result;
use std::path::Path;

use instruction_set::X86InstructionError;

use crate::instruction_set::X86Instruction;

/// Dissassembles a file into a string.
///
/// # Examples
/// ```
/// use inst_decoding_8086::disassemble;
/// use std::path::Path;
/// use std::fs::File;
/// use std::io::Write;
/// use tempfile::NamedTempFile;
///
/// // Create a temporary file and write 0x89 and 0xD9 bytes to it
/// let mut temp_file = NamedTempFile::new().unwrap();
/// temp_file.write_all(&[0x89, 0xD9]).unwrap();
/// temp_file.flush().unwrap();
///
/// // Get the path of the temporary file and run disassemble
/// let path = temp_file.path();
/// let listing = disassemble(&path).unwrap();
///
/// assert_eq!(listing, "bits 16\nmov cx, bx");
/// ```
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
            _ => return Err(X86InstructionError::InvalidInstruction.into()),
        }
    }
    Ok(listing.trim().into())
}

#[cfg(test)]
mod tests {

    use crate::utils::preprocess_listing;
    use std::io::ErrorKind;
    use std::io::Write;
    use tempfile::NamedTempFile;

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

    #[test]
    fn test_disassemble_file_not_found() {
        let non_existent_path = Path::new("some_non_existent_file");
        let result = disassemble(non_existent_path);
        assert_eq!(result.unwrap_err().kind(), ErrorKind::NotFound);
    }

    #[test]
    fn test_invalid_instruction() {
        // Create a temporary file and write some bytes representing an invalid instruction
        let mut temp_file = NamedTempFile::new().unwrap();
        temp_file.write_all(&[0xFF, 0xFF]).unwrap(); // Assuming 0xFFFF is an invalid instruction
        temp_file.flush().unwrap();

        // Attempt to disassemble the file
        let result = disassemble(temp_file.path()).unwrap_err();

        // Check that it's of type InvalidInstruction
        assert!(matches!(result.kind(), ErrorKind::Other));
        let inner_err = result.get_ref().unwrap();
        assert!(matches!(
            inner_err.downcast_ref(),
            Some(&X86InstructionError::InvalidInstruction)
        ));
    }
}
