#![expect(warnings)]
#![feature(array_chunks)]
#![feature(slice_from_ptr_range)]
#![feature(allocator_api)]

mod everywhere_except_last;
mod everywhere_except_last_two;
mod first_or_last_rows;
mod memory_accumulators;
mod quotient_generator;

mod end_params_generator;

mod utils;

use clap::Parser;
use end_params_generator::*;
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
    #[arg(long, default_value = "wrapper/src/wrapper_inner_verifier/imports")]
    output_dir: String,
}

fn main() {
    let cli = Cli::parse();
    let output_dir = cli.output_dir;

    let dummy_bytecode = vec![0u32; setups::final_reduced_risc_v_machine::MAX_ROM_SIZE / 4];
    let compiled_circuit = setups::final_reduced_risc_v_machine::get_machine(
        &dummy_bytecode,
        setups::final_reduced_risc_v_machine::ALLOWED_DELEGATION_CSRS,
    );

    let (verifier, inline_verifier) = generate_verifier_files(&compiled_circuit);
    std::fs::write(Path::new(&output_dir).join("circuit_layout.rs"), verifier)
        .expect(&format!("Failed to write to {}", output_dir));
    std::fs::write(
        Path::new(&output_dir).join("circuit_quotient.rs"),
        inline_verifier,
    )
    .expect(&format!("Failed to write to {}", output_dir));

    let binaries = get_binaries();

    let end_params_constants = format_rust_code(
        &generate_constants(
            &binaries[0],
            &binaries[1],
            &binaries[2],
            &binaries[3],
            &binaries[4],
        )
        .to_string(),
    )
    .unwrap();

    std::fs::write(
        Path::new(&output_dir).join("final_state_constants.rs"),
        end_params_constants,
    )
    .expect(&format!("Failed to write to {}", output_dir));
}

fn get_binaries() -> [&'static [u8]; 5] {
    [
        execution_utils::BASE_PROGRAM,
        // Let's use universal verifiers, as it makes it easier to re-generate tests (as this is what CLI supports).
        execution_utils::UNIVERSAL_CIRCUIT_VERIFIER,
        execution_utils::UNIVERSAL_CIRCUIT_VERIFIER,
        execution_utils::UNIVERSAL_CIRCUIT_NO_DELEGATION_VERIFIER,
        execution_utils::UNIVERSAL_CIRCUIT_NO_DELEGATION_VERIFIER,
    ]
}
