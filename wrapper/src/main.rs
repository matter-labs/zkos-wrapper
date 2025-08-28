#![feature(allocator_api)]
/// Tool that takes the riscv proof from boojum 2.0, together with the final value of the
/// registers - and returns the SNARK proof.
// Inside, it runs 3 submodules:
// - wrapping boojum 2.0 proof into boojum
// - doing the compression
// - wrapping the proof into SNARK.
use clap::{Parser, Subcommand};

use execution_utils::RecursionStrategy;
use zkos_wrapper::{generate_and_save_risc_wrapper_vk, generate_vk, verification_hash};

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

        #[arg(short, long)]
        output_dir: String,

        /// File with the trusted setup.
        /// If missing - will use the 'fake' trusted setup.
        #[arg(long)]
        trusted_setup_file: Option<String>,

        #[arg(long)]
        precomputation_dir: Option<String>,
    },
    /// Take the riscV final proof, and create a RiscWrapper proof.
    ProveRiscWrapper {
        #[arg(short, long)]
        input: String,

        #[arg(short, long)]
        output_dir: String,
    },
    /// Generate verification key for the SNARK proof.
    GenerateSnarkVk {
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

        /// If true, then create VK for universal verifier program.
        /// If false then for the separate verifiers.
        #[arg(long, default_value_t = true)]
        universal_verifier: bool,
        #[arg(long, value_enum, default_value = "use-reduced-log23-machine")]
        recursion_mode: RecursionStrategy,
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
        #[arg(long, default_value_t = true)]
        universal_verifier: bool,
        #[arg(long, value_enum, default_value = "use-reduced-log23-machine")]
        recursion_mode: RecursionStrategy,
    },
    /// Get the hash of the verification key.
    GetVkHash {
        /// Path for VK to calculate the hash.
        #[arg(short, long)]
        vk_path: String,
    },
}

fn main() -> Result<(), Box<dyn std::error::Error>> {
    let cli = Cli::parse();

    match cli.command {
        Commands::ProveFull {
            input,
            output_dir,
            trusted_setup_file,
            precomputation_dir,
        } => {
            println!("=== Phase 0: Proving");
            zkos_wrapper::prove(
                input,
                output_dir,
                trusted_setup_file,
                false,
                precomputation_dir,
            )?;
        }
        Commands::ProveRiscWrapper { input, output_dir } => {
            println!("=== Phase 0: Proving RiscWrapper");
            zkos_wrapper::prove(input, output_dir, None, true, None)?;
        }
        Commands::GenerateSnarkVk {
            input_binary,
            output_dir,
            trusted_setup_file,
            universal_verifier,
            recursion_mode,
        } => {
            println!("=== Phase 0: Generating the verification key");
            generate_vk(
                input_binary,
                output_dir,
                trusted_setup_file,
                universal_verifier,
                recursion_mode,
            )?;
        }
        Commands::GenerateRiscWrapperVk {
            input_binary,
            output_dir,
            universal_verifier,
            recursion_mode,
        } => {
            println!("=== Phase 0: Generating the RiscWrapper verification key");
            generate_and_save_risc_wrapper_vk(
                input_binary,
                output_dir,
                universal_verifier,
                recursion_mode,
            )?;
        }
        Commands::GetVkHash { vk_path } => {
            verification_hash(vk_path);
        }
    }
    Ok(())
}
