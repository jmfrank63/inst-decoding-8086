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
    let mut file = File::create(output_filename)?;
    file.write_all(listing.as_bytes())?;
    Ok(())
}
