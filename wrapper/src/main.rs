#![feature(allocator_api)]
/// Tool that takes the riscv proof from boojum 2.0, together with the final value of the
/// registers - and returns the SNARK proof.
// Inside, it runs 3 submodules:
// - wrapping boojum 2.0 proof into boojum
// - doing the compression
// - wrapping the proof into SNARK.
use clap::{Parser, Subcommand};

use zkos_wrapper::{generate_and_save_risc_wrapper_vk, generate_vk};

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
            )?;
        }
        Commands::GenerateRiscWrapperVk {
            input_binary,
            output_dir,
            universal_verifier,
        } => {
            println!("=== Phase 0: Generating the RiscWrapper verification key");
            generate_and_save_risc_wrapper_vk(input_binary, output_dir, universal_verifier)?;
        }
    }
    Ok(())
}
