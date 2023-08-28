# 8086 Instruction Decoder

## Overview

This project aims to provide a simple instruction decoder for the original Intel 8086 microprocessor.
It's designed as an educational tool to help better understand the lower-level operations of this
classic CPU.

The project is based on the course available at [computerenhance.com](https://www.computerenhance.com/),
and it's meant for anyone looking to learn more about assembly language, computer architecture, or
retro computing.

## Features

- Decodes instructions for the 8086 CPU
- Supports MOV instruction (more to be added)
- Enum-based representation of opcodes and registers
- Error handling for invalid instructions and registers

## Usage

```rust
let inst = X86Instruction::new([0b10001000, 0b11000111]);
let decoded_inst = inst.format_instruction().unwrap();
println!("Decoded instruction: {}", decoded_inst);
```

## Tests

The project contains a suite of tests to ensure accurate decoding of 8086 instructions.
Run the tests using:

```bash
cargo test
```

## Future Work

- Add more instruction support
- Implement a disassembler
- Possibly extend the support to 80186, 80286, etc.

## Contributing

Feel free to fork the project and submit pull requests or issues.

---

Feel free to add or remove sections as you see fit for your project!
