#![feature(allocator_api)]
use std::alloc::Global;
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

use execution_utils::{
    final_recursion_layer_verifier_vk, recursion_layer_no_delegation_verifier_vk,
    recursion_layer_verifier_vk, universal_circuit_no_delegation_verifier_vk,
    universal_circuit_verifier_vk,
};
use risc_verifier::prover::worker::Worker;
use zkos_wrapper::circuits::BinaryCommitment;
use zkos_wrapper::{Bn256, L1_VERIFIER_DOMAIN_SIZE_LOG};
use zkos_wrapper::{
    circuits::RiscWrapperWitness, get_compression_setup, get_risc_wrapper_setup,
    get_snark_wrapper_setup, prove_compression, prove_risc_wrapper, prove_snark_wrapper,
    verify_compression_proof, verify_risc_wrapper_proof, verify_snark_wrapper_proof,
};

#[derive(Parser)]
#[command(version, about, long_about = None)]
struct Cli {
    #[arg(short, long)]
    input: String,

    // Binary used to generate the proof.
    // If not specified, take the default binary (fibonacci hasher).
    #[arg(long)]
    input_binary: Option<String>,

    #[arg(short, long)]
    output_dir: String,
}

fn serialize_to_file<T: serde::ser::Serialize>(content: &T, filename: &Path) {
    let src = std::fs::File::create(filename).unwrap();
    serde_json::to_writer_pretty(src, content).unwrap();
}

fn deserialize_from_file<T: serde::de::DeserializeOwned>(filename: &str) -> T {
    let src = std::fs::File::open(filename).unwrap();
    serde_json::from_reader(src).unwrap()
}

pub fn create_binary_commitment(
    binary_path: String,
    expected_end_params: &[u32; 8],
) -> BinaryCommitment {
    let bin = std::fs::read(binary_path).unwrap();

    let worker = Worker::new_with_num_threads(8);

    let expected_final_pc = execution_utils::find_binary_exit_point(&bin);
    let binary: Vec<u32> = execution_utils::get_padded_binary(&bin);

    let base_params = execution_utils::compute_end_parameters(
        expected_final_pc,
        &setups::get_main_riscv_circuit_setup::<Global>(&binary, &worker),
    );

    // Check which verifier was used.
    if universal_circuit_no_delegation_verifier_vk().params == *expected_end_params {
        let layers = vec![
            [0u32; 8],
            base_params,
            universal_circuit_verifier_vk().params,
            universal_circuit_no_delegation_verifier_vk().params,
        ];
        BinaryCommitment {
            end_params: universal_circuit_no_delegation_verifier_vk().params,
            aux_params: execution_utils::compute_chain_encoding(layers),
        }
    } else if final_recursion_layer_verifier_vk().params == *expected_end_params {
        let layers = vec![
            [0u32; 8],
            base_params,
            recursion_layer_verifier_vk().params,
            recursion_layer_no_delegation_verifier_vk().params,
            final_recursion_layer_verifier_vk().params,
        ];
        BinaryCommitment {
            end_params: final_recursion_layer_verifier_vk().params,
            aux_params: execution_utils::compute_chain_encoding(layers),
        }
    } else {
        panic!(
            "Cannot find a verifier for the proof end parameters: {:?}",
            expected_end_params
        );
    }
}

fn main() -> Result<(), Box<dyn std::error::Error>> {
    let cli = Cli::parse();

    println!("=== Phase 1: Creating the Risc wrapper proof");

    let worker = boojum::worker::Worker::new_with_num_threads(4);

    let program_proof: zkos_wrapper::ProgramProof = deserialize_from_file(&cli.input);
    let binary_commitment = match cli.input_binary {
        Some(binary_path) => create_binary_commitment(binary_path, &program_proof.end_params),
        None => BinaryCommitment::from_default_binary(),
    };
    let risc_wrapper_witness =
        RiscWrapperWitness::from_full_proof(program_proof, &binary_commitment);

    let (
        finalization_hint,
        setup_base,
        setup,
        risc_wrapper_vk,
        setup_tree,
        vars_hint,
        witness_hints,
    ) = get_risc_wrapper_setup(&worker, binary_commitment.clone());

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
        binary_commitment,
    );
    let is_valid = verify_risc_wrapper_proof(&risc_wrapper_proof, &risc_wrapper_vk);

    assert!(is_valid);

    serialize_to_file(
        &risc_wrapper_proof,
        &Path::new(&cli.output_dir.clone()).join("risc_proof.json"),
    );

    println!("=== Phase 2: Creating compression proof");

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

    println!("=== Phase 3: Creating SNARK proof");

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
