#![feature(allocator_api)]
use std::alloc::Global;
/// Tool that takes the riscv proof from boojum 2.0, together with the final value of the
/// registers - and returns the SNARK proof.
// Inside, it runs 3 submodules:
// - wrapping boojum 2.0 proof into boojum
// - doing the compression
// - wrapping the proof into SNARK.
use std::path::Path;

use clap::{Parser, Subcommand};

use bellman::kate_commitment::{Crs, CrsForMonomialForm};
use bellman::worker::Worker as BellmanWorker;

use execution_utils::{
    final_recursion_layer_verifier_vk, recursion_layer_no_delegation_verifier_vk,
    recursion_layer_verifier_vk, universal_circuit_no_delegation_verifier_vk,
    universal_circuit_verifier_vk,
};
use risc_verifier::prover::worker::Worker;
use zkos_wrapper::circuits::BinaryCommitment;
use zkos_wrapper::{Bn256, L1_VERIFIER_DOMAIN_SIZE_LOG, calculate_verification_key_hash};
use zkos_wrapper::{
    circuits::RiscWrapperWitness, get_compression_setup, get_risc_wrapper_setup,
    get_snark_wrapper_setup, prove_compression, prove_risc_wrapper, prove_snark_wrapper,
    verify_compression_proof, verify_risc_wrapper_proof, verify_snark_wrapper_proof,
};

#[derive(Parser)]
#[command(version, about, long_about = None)]
struct Cli {
    #[command(subcommand)]
    command: Commands,
}

#[derive(Subcommand)]
enum Commands {
    /// Take the riscV final proof, and create a SNARK proof.
    Prove {
        #[arg(short, long)]
        input: String,

        // Binary used to generate the proof.
        // If not specified, take the default binary (fibonacci hasher).
        #[arg(long)]
        input_binary: Option<String>,

        #[arg(short, long)]
        output_dir: String,

        /// File with the trusted setup.
        /// If missing - will use the 'fake' trusted setup.
        #[arg(long)]
        trusted_setup_file: Option<String>,
    },
    /// Generate verification key for the SNARK proof.
    GenerateVk {
        // Binary used to generate the proof.
        // If not specified, take the default binary (fibonacci hasher).
        #[arg(long)]
        input_binary: String,

        #[arg(short, long)]
        output_dir: String,

        /// File with the trusted setup.
        /// If missing - will use the 'fake' trusted setup.
        #[arg(long)]
        trusted_setup_file: String,
    },
}

fn serialize_to_file<T: serde::ser::Serialize>(content: &T, filename: &Path) {
    let src = std::fs::File::create(filename).unwrap();
    serde_json::to_writer_pretty(src, content).unwrap();
}

fn deserialize_from_file<T: serde::de::DeserializeOwned>(filename: &str) -> T {
    let src = std::fs::File::open(filename).unwrap();
    serde_json::from_reader(src).unwrap()
}

/// Uploads trusted setup file to the RAM
pub fn get_trusted_setup(crs_file_str: &String) -> Crs<Bn256, CrsForMonomialForm> {
    let crs_file_path = std::path::Path::new(crs_file_str);
    let crs_file = std::fs::File::open(&crs_file_path)
        .expect(format!("Trying to open CRS FILE: {:?}", crs_file_path).as_str());
    Crs::read(&crs_file).expect(format!("Trying to read CRS FILE: {:?}", crs_file_path).as_str())
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

    match cli.command {
        Commands::Prove {
            input,
            input_binary,
            output_dir,
            trusted_setup_file,
        } => {
            println!("=== Phase 0: Proving");
            prove(input, input_binary, output_dir, trusted_setup_file)?;
        }
        Commands::GenerateVk {
            input_binary,
            output_dir,
            trusted_setup_file,
        } => {
            println!("=== Phase 0: Generating the verification key");
            generate_vk(input_binary, output_dir, trusted_setup_file)?;
        }
    }
    Ok(())
}

fn generate_vk(
    input_binary: String,
    output_dir: String,
    trusted_setup_file: String,
) -> Result<(), Box<dyn std::error::Error>> {
    let worker = BellmanWorker::new_with_cpus(4);
    let boojum_worker = boojum::worker::Worker::new_with_num_threads(4);

    // FIXME.
    let trusted_setup_file = None;

    let crs_mons = match trusted_setup_file {
        Some(ref crs_file_str) => get_trusted_setup(crs_file_str),
        None => Crs::<Bn256, CrsForMonomialForm>::crs_42(
            1 << L1_VERIFIER_DOMAIN_SIZE_LOG,
            &BellmanWorker::new(),
        ),
    };
    // FIXME: add universal vs non-universal.
    let binary_commitment = create_binary_commitment(
        input_binary,
        &universal_circuit_no_delegation_verifier_vk().params,
    );

    let (_, _, _, risc_wrapper_vk, _, _, _) =
        get_risc_wrapper_setup(&boojum_worker, binary_commitment.clone());

    let (_, _, _, compression_vk, _, _, _) =
        get_compression_setup(risc_wrapper_vk.clone(), &boojum_worker);

    let (_, snark_wrapper_vk) = get_snark_wrapper_setup(compression_vk.clone(), &crs_mons, &worker);

    serialize_to_file(
        &snark_wrapper_vk,
        &Path::new(&output_dir.clone()).join("snark_vk_expected.json"),
    );

    println!(
        "VK key hash: {:?}",
        calculate_verification_key_hash(snark_wrapper_vk)
    );

    Ok(())
}

fn prove(
    input: String,
    input_binary: Option<String>,
    output_dir: String,
    trusted_setup_file: Option<String>,
) -> Result<(), Box<dyn std::error::Error>> {
    let crs_mons = match trusted_setup_file {
        Some(ref crs_file_str) => get_trusted_setup(crs_file_str),
        None => Crs::<Bn256, CrsForMonomialForm>::crs_42(
            1 << L1_VERIFIER_DOMAIN_SIZE_LOG,
            &BellmanWorker::new(),
        ),
    };

    println!("=== Phase 1: Creating the Risc wrapper proof");

    let worker = boojum::worker::Worker::new_with_num_threads(4);

    let program_proof: zkos_wrapper::ProgramProof = deserialize_from_file(&input);
    let binary_commitment = match input_binary {
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
        &Path::new(&output_dir.clone()).join("risc_proof.json"),
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
        &Path::new(&output_dir.clone()).join("compression_proof.json"),
    );

    println!("=== Phase 3: Creating SNARK proof");

    {
        let worker = BellmanWorker::new_with_cpus(4);

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
            &Path::new(&output_dir.clone()).join("snark_proof.json"),
        );
        serialize_to_file(
            &snark_wrapper_vk,
            &Path::new(&output_dir.clone()).join("snark_vk.json"),
        );
    }

    Ok(())
}
