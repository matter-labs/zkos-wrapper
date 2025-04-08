/// Tool that takes the riscv proof from boojum 2.0, together with the final value of the
/// registers - and returns the SNARK proof.
// Inside, it runs 3 submodules:
// - wrapping boojum 2.0 proof into boojum
// - doing the compression
// - wrapping the proof into SNARK.
use std::path::Path;

use clap::Parser;

use bellman::kate_commitment::{Crs, CrsForMonomialForm};
use bellman::worker::Worker as BellmanWorker;

use zkos_wrapper::{
    get_compression_setup, get_risc_wrapper_setup, get_snark_wrapper_setup, prove_compression,
    prove_risc_wrapper, prove_snark_wrapper, verify_compression_proof, verify_risc_wrapper_proof,
    verify_snark_wrapper_proof, zkos_utils::read_and_verify_proof,
};
use zkos_wrapper::{Bn256, L1_VERIFIER_DOMAIN_SIZE_LOG};

#[derive(Parser)]
#[command(version, about, long_about = None)]
struct Cli {
    #[arg(short, long)]
    input: String,
    #[arg(short, long)]
    /// File where the values of the registers are stored.
    registers_input: String,
    #[arg(short, long)]
    output_dir: String,
}

fn serialize_to_file<T: serde::ser::Serialize>(content: &T, filename: &Path) {
    let src = std::fs::File::create(filename).unwrap();
    serde_json::to_writer_pretty(src, content).unwrap();
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

    serialize_to_file(
        &risc_wrapper_proof,
        &Path::new(&cli.output_dir.clone()).join("risc_proof.json"),
    );

    let (
        finalization_hint,
        setup_base,
        setup,
        compression_vk,
        setup_tree,
        vars_hint,
        witness_hints,
    ) = get_compression_setup(risc_wrapper_vk.clone(), &worker);
    let compression_proof = prove_compression(
        risc_wrapper_proof,
        risc_wrapper_vk,
        &finalization_hint,
        &setup_base,
        &setup,
        &compression_vk,
        &setup_tree,
        &vars_hint,
        &witness_hints,
        &worker,
    );
    let is_valid = verify_compression_proof(&compression_proof, &compression_vk);

    assert!(is_valid);

    serialize_to_file(
        &compression_proof,
        &Path::new(&cli.output_dir.clone()).join("compression_proof.json"),
    );

    {
        let worker = BellmanWorker::new_with_cpus(4);

        let crs_mons =
            Crs::<Bn256, CrsForMonomialForm>::crs_42(1 << L1_VERIFIER_DOMAIN_SIZE_LOG, &worker);

        let (snark_setup, snark_wrapper_vk) =
            get_snark_wrapper_setup(compression_vk.clone(), &crs_mons, &worker);

        let snark_wrapper_proof = prove_snark_wrapper(
            compression_proof,
            compression_vk,
            &snark_setup,
            &crs_mons,
            &worker,
        );

        let is_valid = verify_snark_wrapper_proof(&snark_wrapper_proof, &snark_wrapper_vk);

        assert!(is_valid);

        serialize_to_file(
            &snark_wrapper_proof,
            &Path::new(&cli.output_dir.clone()).join("snark_proof.json"),
        );
        serialize_to_file(
            &snark_wrapper_vk,
            &Path::new(&cli.output_dir.clone()).join("snark_vk.json"),
        );
    }

    Ok(())
}
