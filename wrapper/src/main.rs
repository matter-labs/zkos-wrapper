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
    ProveFull {
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
    /// Take the riscV final proof, and create a RiscWrapper proof.
    ProveRiscWrapper {
        #[arg(short, long)]
        input: String,

        // Binary used to generate the proof.
        // If not specified, take the default binary (fibonacci hasher).
        #[arg(long)]
        input_binary: Option<String>,

        #[arg(short, long)]
        output_dir: String,
    },
    /// Generate verification key for the SNARK proof.
    GenerateSnarkVk {
        // Binary used to generate the proof.
        // If not specified, take the default binary (fibonacci hasher).
        #[arg(long)]
        input_binary: String,

        #[arg(short, long)]
        output_dir: String,

        /// File with the trusted setup.
        /// If missing - will use the 'fake' trusted setup.
        #[arg(long)]
        trusted_setup_file: Option<String>,

        /// If true, then create VK for universal verifier program.
        /// If false then for the separate verifiers.
        #[arg(long)]
        universal_verifier: bool,
    },
    /// Generate verification key for the RiscWrapper proof.
    GenerateRiscWrapperVk {
        // Binary used to generate the proof.
        // If not specified, take the default binary (fibonacci hasher).
        #[arg(long)]
        input_binary: String,

        #[arg(short, long)]
        output_dir: String,

        /// If true, then create VK for universal verifier program.
        /// If false then for the separate verifiers.
        #[arg(long)]
        universal_verifier: bool,
    },
}

fn main() -> Result<(), Box<dyn std::error::Error>> {
    let cli = Cli::parse();

    match cli.command {
        Commands::ProveFull {
            input,
            input_binary,
            output_dir,
            trusted_setup_file,
        } => {
            println!("=== Phase 0: Proving");
            zkos_wrapper::prove(input, input_binary, output_dir, trusted_setup_file, false)?;
        }
        Commands::ProveRiscWrapper {
            input,
            input_binary,
            output_dir,
        } => {
            println!("=== Phase 0: Proving RiscWrapper");
            zkos_wrapper::prove(input, input_binary, output_dir, None, true)?;
        }
        Commands::GenerateSnarkVk {
            input_binary,
            output_dir,
            trusted_setup_file,
            universal_verifier,
        } => {
            println!("=== Phase 0: Generating the verification key");
            generate_vk(
                input_binary,
                output_dir,
                trusted_setup_file,
                universal_verifier,
                false,
            )?;
        },
        Commands::GenerateRiscWrapperVk {
            input_binary,
            output_dir,
            universal_verifier,
        } => {
            println!("=== Phase 0: Generating the RiscWrapper verification key");
            generate_vk(
                input_binary,
                output_dir,
                None,
                universal_verifier,
                true,
            )?;
        }
    }
    Ok(())
}

fn generate_vk(
    input_binary: String,
    output_dir: String,
    trusted_setup_file: Option<String>,
    universal_verifier: bool,
    risc_wrapper_only: bool,
) -> Result<(), Box<dyn std::error::Error>> {
    let worker = BellmanWorker::new();
    let boojum_worker = boojum::worker::Worker::new();

    println!("=== Phase 1: Creating the Risc wrapper key");

    let verifier_params = if universal_verifier {
        universal_circuit_no_delegation_verifier_vk().params
    } else {
        final_recursion_layer_verifier_vk().params
    };

    let binary_commitment = zkos_wrapper::create_binary_commitment(input_binary, &verifier_params);

    let (_, _, _, risc_wrapper_vk, _, _, _) =
        get_risc_wrapper_setup(&boojum_worker, binary_commitment.clone());

    if risc_wrapper_only {
        zkos_wrapper::serialize_to_file(
            &risc_wrapper_vk,
            &Path::new(&output_dir.clone()).join("risc_wrapper_vk_expected.json"),
        );
        return Ok(());
    }

    println!("=== Phase 2: Creating the Compression key");
    let (_, _, _, compression_vk, _, _, _) =
        get_compression_setup(risc_wrapper_vk.clone(), &boojum_worker);

    println!("=== Phase 3: Creating the SNARK key");

    let crs_mons = match trusted_setup_file {
        Some(ref crs_file_str) => zkos_wrapper::get_trusted_setup(crs_file_str),
        None => Crs::<Bn256, CrsForMonomialForm>::crs_42(
            1 << L1_VERIFIER_DOMAIN_SIZE_LOG,
            &BellmanWorker::new(),
        ),
    };

    let (_, snark_wrapper_vk) = get_snark_wrapper_setup(compression_vk.clone(), &crs_mons, &worker);

    zkos_wrapper::serialize_to_file(
        &snark_wrapper_vk,
        &Path::new(&output_dir.clone()).join("snark_vk_expected.json"),
    );

    println!(
        "VK key hash: {:?}",
        calculate_verification_key_hash(snark_wrapper_vk)
    );

    Ok(())
}
