use std::fs::File;

use clap::Parser;

use zkos_wrapper::{
    get_zkos_wrapper_proof, get_zkos_wrapper_setup, verify_zkos_wrapper_proof,
    zkos_utils::read_and_verify_proof,
};

#[derive(Parser)]
#[command(version, about, long_about = None)]
struct Cli {
    #[arg(short, long)]
    input: String,
    #[arg(short, long)]
    output: String,
}

fn main() -> Result<(), Box<dyn std::error::Error>> {
    let cli = Cli::parse();

    let worker = boojum::worker::Worker::new_with_num_threads(4);
    let (proof, _, _) = read_and_verify_proof(&cli.input);

    let (finalization_hint, setup_base, setup, vk, setup_tree, vars_hint, witness_hints) =
        get_zkos_wrapper_setup(&worker);
    let proof = get_zkos_wrapper_proof(
        proof,
        &finalization_hint,
        &setup_base,
        &setup,
        &vk,
        &setup_tree,
        &vars_hint,
        &witness_hints,
        &worker,
    );

    let is_valid = verify_zkos_wrapper_proof(&proof, &vk);

    assert!(is_valid);

    let proof_file = File::create(cli.output)?;
    serde_json::to_writer(proof_file, &proof)?;

    Ok(())
}
