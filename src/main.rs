use std::fs::File;
use std::io::{self, Read};
use std::path::Path;

fn main() -> io::Result<()> {
    if cfg!(target_endian = "little") {
        println!("The system is little-endian!");
    } else if cfg!(target_endian = "big") {
        println!("The system is big-endian!");
    } else {
        println!("The endianness of the system is unknown!");
    }
    let path = Path::new("data/listing_0038_many_register_mov");
    let mut file = File::open(path)?;

    // Create a 2-byte buffer to read in 16 bits at a time
    let mut buffer = [0; 2];

    while file.read(&mut buffer)? > 0 {
        // Interpret the 2 bytes as a 16-bit unsigned integer (little endian)
        let value = u16::from_ne_bytes(buffer);
        println!("Read 16-bit value: {:b}", value);
    }

    Ok(())
}
