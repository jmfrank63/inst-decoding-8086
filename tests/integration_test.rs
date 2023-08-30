use std::fs;
use std::io::Write;
use std::process::Command;
use tempfile::{Builder, NamedTempFile};

use crate::utils::preprocess_listing;
#[path = "../src/utils.rs"]
mod utils;

#[test]
fn test_command_line_tool() {
    let mut input_file = NamedTempFile::new().expect("Failed to create temporary input file.");
    let output_file = NamedTempFile::new().expect("Failed to create temporary output file.");

    writeln!(input_file, "Test data").expect("Failed to write to temporary input file.");

    let input_path = input_file
        .path()
        .to_str()
        .expect("Failed to convert input path to str.");
    let output_path = output_file
        .path()
        .to_str()
        .expect("Failed to convert output path to str.");

    let mut cmd = Command::new("target/debug/inst-decoding-8086");
    cmd.arg(input_path);
    cmd.arg(output_path);

    let output = cmd.output().expect("Failed to execute command");

    assert!(output.status.success(), "Command did not run successfully.");

    let expected_output = "bits 16";
    let real_output = fs::read_to_string(output_path).expect("Failed to read output file.");

    assert_eq!(expected_output, real_output);
}

#[test]
fn test_command_line_tool_without_output_filename() {
    let mut input_file = NamedTempFile::new().expect("Failed to create temporary input file.");

    writeln!(input_file, "Test data").expect("Failed to write to temporary input file.");

    let input_path = input_file
        .path()
        .to_str()
        .expect("Failed to convert input path to str.");
    let output_path = format!("{}.asm", input_path); // This is where your else branch should write the output

    let mut cmd = Command::new("target/debug/inst-decoding-8086");
    cmd.arg(input_path);

    let output = cmd.output().expect("Failed to execute command");

    assert!(output.status.success());

    let expected_output = "bits 16";
    let real_output = fs::read_to_string(&output_path).expect("Failed to read output file.");

    assert_eq!(expected_output, real_output);

    // Cleanup
    fs::remove_file(output_path).expect("Failed to remove output file");
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

        let output = cmd.output().expect("Failed to execute command");
        assert!(
            output.status.success(),
            "Command did not run successfully for {}",
            filename
        );

        // Read the expected output from the .asm file
        let expected_output = fs::read_to_string(&expected_output_path)
            .expect("Failed to read expected output file.");
        let expected_output = preprocess_listing(&expected_output);
        // Read the actual output
        let real_output = fs::read_to_string(output_path).expect("Failed to read output file.");

        assert_eq!(
            expected_output, real_output,
            "Output did not match expected output for {}",
            filename
        );

        // Cleanup
        // fs::remove_file(&output_path).expect("Failed to remove output file");
    }
}
