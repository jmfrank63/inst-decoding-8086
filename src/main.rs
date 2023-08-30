use std::env;
use std::fs::File;
use std::io::{Result, Write};
use std::path::Path;

use inst_decoding_8086::disassemble;

fn main() -> Result<()> {
    let args: Vec<String> = env::args().collect();
    let input_filename = args
        .get(1)
        .ok_or("Input file name is required")
        .map_err(|e| std::io::Error::new(std::io::ErrorKind::InvalidInput, e))?;
    let output_filename = get_output_filename(&args, input_filename);
    let input_path = Path::new(&input_filename);
    let listing = disassemble(input_path)?;
    println!("{}", listing);
    let mut file = File::create(output_filename)?;
    file.write_all(listing.as_bytes())?;
    Ok(())
}

fn get_output_filename(args: &[String], default_input: &str) -> String {
    if let Some(output_arg) = args.get(2) {
        output_arg.clone()
    } else {
        let mut output = default_input.to_string();
        output.push_str(".asm");
        output
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_get_output_filename() {
        let args = vec!["inst-decoding-8086".to_string(), "input".to_string()];
        let default_input = "input";
        let expected = "input.asm";
        assert_eq!(get_output_filename(&args, default_input), expected);
    }

    #[test]
    fn test_get_output_filename_with_output_arg() {
        let args = vec![
            "inst-decoding-8086".to_string(),
            "input".to_string(),
            "output".to_string(),
        ];
        let default_input = "input";
        let expected = "output";
        assert_eq!(get_output_filename(&args, default_input), expected);
    }
}
