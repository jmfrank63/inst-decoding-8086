use std::fs;
use std::io::Write;
use std::process::Command;
use tempfile::{Builder, NamedTempFile};

use crate::utils::preprocess_listing;
#[path = "../src/utils.rs"]
mod utils;

#[test]
fn test_command_line_tool() {
    let mut input_file = NamedTempFile::new().unwrap();
    let output_file = NamedTempFile::new().unwrap();

    input_file.write_all(&[0x89, 0xD9]).unwrap();

    let input_path = input_file.path().to_str().unwrap();
    let output_path = output_file.path().to_str().unwrap();

    let mut cmd = Command::new("target/debug/inst-decoding-8086");
    cmd.arg(input_path);
    cmd.arg(output_path);

    let output = cmd.output().unwrap();

    assert!(output.status.success());

    let expected_output = "bits 16\nmov cx, bx";
    let real_output = fs::read_to_string(output_path).unwrap();

    assert_eq!(expected_output, real_output);
}

#[test]
fn test_command_line_tool_without_output_filename() {
    let mut input_file = NamedTempFile::new().unwrap();
    input_file.write_all(&[0x89, 0xD9]).unwrap();

    let input_path = input_file.path().to_str().unwrap();
    let output_path = format!("{}.asm", input_path);

    let mut cmd = Command::new("target/debug/inst-decoding-8086");
    cmd.arg(input_path);

    let output = cmd.output().unwrap();

    assert!(output.status.success());

    let expected_output = "bits 16\nmov cx, bx";
    let real_output = fs::read_to_string(output_path).unwrap();

    assert_eq!(expected_output, real_output);
}

#[test]
fn test_functional_coverage() {
    for filename in [
        "listing_0037_single_register_mov",
        "listing_0038_many_register_mov",
    ] {
        let input_path = format!("tests/test_data/{}", filename);
        let named_tempfile = Builder::new()
            .prefix(filename)
            .suffix(".asm")
            .rand_bytes(5)
            .tempfile()
            .unwrap();
        let output_path = named_tempfile.path().to_str().unwrap();
        let expected_output_path = format!("tests/test_data/{}.asm", filename);

        let mut cmd = Command::new("target/debug/inst-decoding-8086");
        cmd.arg(input_path).arg(output_path);

        let output = cmd.output().unwrap();
        assert!(output.status.success());

        let expected_output = fs::read_to_string(&expected_output_path).unwrap();
        let normalised_expected_output = preprocess_listing(&expected_output);
        let real_output = fs::read_to_string(output_path).unwrap();

        assert_eq!(normalised_expected_output, real_output);
    }
}
