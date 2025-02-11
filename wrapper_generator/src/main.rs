#![feature(array_chunks)]

mod everywhere_except_last;
mod everywhere_except_last_two;
mod first_or_last_rows;
mod memory_accumulators;
mod quotient_generator;

mod utils;

use clap::Parser;
use prover::{cs::one_row_compiler::CompiledCircuitArtifact, field::Mersenne31Field};
use quotient_generator::generate_inlined;
use std::io::Write;
use std::path::Path;
use std::process::Command;
use std::process::Stdio;
use zkos_verifier_generator::generate_from_parts;

/// Returns formatted rust code with verifier and inline verifier files.
fn generate_verifier_files(circuit: &CompiledCircuitArtifact<Mersenne31Field>) -> (String, String) {
    let verifier = format_rust_code(&generate_from_parts(&circuit).to_string()).unwrap();

    let inlined_verifier =
        format_rust_code(&generate_inlined(circuit.clone()).to_string()).unwrap();

    (verifier, inlined_verifier)
}

/// Runs rustfmt to format the code.
fn format_rust_code(code: &str) -> Result<String, String> {
    // Spawn the `rustfmt` process
    let mut rustfmt = Command::new("rustfmt")
        .stdin(Stdio::piped())
        .stdout(Stdio::piped())
        .spawn()
        .map_err(|e| format!("Failed to spawn rustfmt: {}", e))?;

    // Write the Rust code to `rustfmt`'s stdin
    if let Some(mut stdin) = rustfmt.stdin.take() {
        stdin
            .write_all(code.as_bytes())
            .map_err(|e| format!("Failed to write to rustfmt stdin: {}", e))?;
    }

    // Wait for `rustfmt` to complete and collect the formatted code
    let output = rustfmt
        .wait_with_output()
        .map_err(|e| format!("Failed to read rustfmt output: {}", e))?;

    if !output.status.success() {
        return Err(format!(
            "rustfmt failed with status {}: {}",
            output.status,
            String::from_utf8_lossy(&output.stderr)
        ));
    }

    // Convert the output to a String
    String::from_utf8(output.stdout).map_err(|e| format!("Invalid UTF-8 in rustfmt output: {}", e))
}

#[derive(Parser)]
#[command(version, about, long_about = None)]
struct Cli {
    #[arg(long, default_value = "../src/delegation_verifier/imports")]
    output_dir: String,
    #[arg(long, default_value = "delegation_layout")]
    input_layout_file: String,
    #[arg(long, default_value = "../src/blake2s_delegator_verifier/imports")]
    blake2s_output_dir: String,
    #[arg(long, default_value = "blake2s_delegator_layout")]
    blake2s_input_layout_file: String,
}

fn deserialize_from_file<T: serde::de::DeserializeOwned>(filename: &str) -> T {
    let src = std::fs::File::open(filename).unwrap();
    serde_json::from_reader(src).unwrap()
}

fn main() {
    let cli = Cli::parse();

    let output_dir = cli.output_dir;
    let input_layout_file = cli.input_layout_file;

    let compiled_circuit: CompiledCircuitArtifact<Mersenne31Field> =
        deserialize_from_file(&input_layout_file);
    let (verifier, inline_verifier) = generate_verifier_files(&compiled_circuit);
    std::fs::write(Path::new(&output_dir).join("circuit_layout.rs"), verifier)
        .expect(&format!("Failed to write to {}", output_dir));
    std::fs::write(
        Path::new(&output_dir).join("circuit_quotient.rs"),
        inline_verifier,
    )
    .expect(&format!("Failed to write to {}", output_dir));

    // let blake2s_input_layout_file = cli.blake2s_input_layout_file;

    // let compiled_circuit: CompiledCircuitArtifact<Mersenne31Field> =
    //     deserialize_from_file(&blake2s_input_layout_file);
    // let (blake2s_verifier, blake2s_inline_verifier) = generate_verifier_files(&compiled_circuit);
    // std::fs::write(Path::new(&output_dir).join("circuit_layout.rs"), blake2s_verifier)
    //     .expect(&format!("Failed to write to {}", output_dir));
    // std::fs::write(
    //     Path::new(&output_dir).join("circuit_quotient.rs"),
    //     blake2s_inline_verifier,
    // )
    // .expect(&format!("Failed to write to {}", output_dir));
}
