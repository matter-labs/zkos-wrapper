use std::fs::File;

use clap::Parser;

use zkos_wrapper::{
    get_risc_wrapper_setup, prove_risc_wrapper, verify_risc_wrapper_proof,
    zkos_utils::read_and_verify_proof,
};

#[derive(Parser)]
#[command(version, about, long_about = None)]
struct Cli {
    #[arg(short, long)]
    input: String,
    #[arg(short, long)]
    registers_input: String,
    #[arg(short, long)]
    output: String,
}

fn main() -> Result<(), Box<dyn std::error::Error>> {
    let cli = Cli::parse();

    let worker = boojum::worker::Worker::new_with_num_threads(4);
    let risc_wrapper_witness = read_and_verify_proof(&cli.input, &cli.registers_input);

    let (
        finalization_hint,
        setup_base,
        setup,
        risc_wrapper_vk,
        setup_tree,
        vars_hint,
        witness_hints,
    ) = get_risc_wrapper_setup(&worker);

    let risc_wrapper_proof = prove_risc_wrapper(
        risc_wrapper_witness,
        &finalization_hint,
        &setup_base,
        &setup,
        &risc_wrapper_vk,
        &setup_tree,
        &vars_hint,
        &witness_hints,
        &worker,
    );
    let is_valid = verify_risc_wrapper_proof(&risc_wrapper_proof, &risc_wrapper_vk);

    assert!(is_valid);

    let proof_file = File::create(cli.output)?;
    serde_json::to_writer(proof_file, &risc_wrapper_proof)?;

    Ok(())
}
