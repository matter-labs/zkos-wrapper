#![feature(allocator_api)]

use clap::{Parser, Subcommand, ValueEnum};
use std::error::Error;
use std::path::PathBuf;
use tracing::level_filters::LevelFilter;
use tracing_subscriber::EnvFilter;

use zkos_wrapper::interface;

#[derive(Parser)]
#[command(
    name = "wrapper",
    version,
    about = "Wrap FRI proofs from zksync-airbender into SNARKs verifiable on Ethereum"
)]
struct Cli {
    /// Number of worker threads (defaults to all available cores)
    #[arg(long, global = true)]
    threads: Option<usize>,

    #[command(subcommand)]
    command: Commands,
}

#[derive(Subcommand)]
enum Commands {
    /// Full pipeline: FRI proof -> SNARK proof (phases 1+2+3)
    ProveAll {
        /// Path to the FRI proof JSON (UnrolledProgramProof from airbender)
        #[arg(long)]
        proof: PathBuf,

        /// Path to the RISC-V .bin file (omit to use default recursion verifier)
        #[arg(long, requires = "text")]
        bin: Option<PathBuf>,

        /// Path to the RISC-V .text file (omit to use default recursion verifier)
        #[arg(long, requires = "bin")]
        text: Option<PathBuf>,

        /// Output directory for proof and VK files
        #[arg(short, long)]
        output_dir: PathBuf,

        /// Path to trusted setup (CRS) file. If omitted, uses fake crs_42 for testing.
        #[arg(long)]
        trusted_setup: Option<PathBuf>,

        /// Enable zero-knowledge padding in SNARK proving phase
        #[arg(long)]
        use_zk: bool,

        /// Save intermediate proofs (risc wrapper, compression) alongside final SNARK
        #[arg(long)]
        save_intermediates: bool,
    },

    /// Phase 1: Wrap FRI proof into a boojum STARK proof
    ProveRiscWrapper {
        /// Path to the FRI proof JSON (UnrolledProgramProof)
        #[arg(long)]
        proof: PathBuf,

        /// Path to the RISC-V .bin file (omit to use default recursion verifier)
        #[arg(long, requires = "text")]
        bin: Option<PathBuf>,

        /// Path to the RISC-V .text file (omit to use default recursion verifier)
        #[arg(long, requires = "bin")]
        text: Option<PathBuf>,

        /// Output directory
        #[arg(short, long)]
        output_dir: PathBuf,
    },

    /// Phase 2: Compress a STARK wrapper proof (Poseidon2-based re-hashing)
    ProveCompression {
        /// Path to risc_wrapper_proof.json
        #[arg(long)]
        risc_wrapper_proof: PathBuf,

        /// Path to risc_wrapper_vk.json
        #[arg(long)]
        risc_wrapper_vk: PathBuf,

        /// Output directory
        #[arg(short, long)]
        output_dir: PathBuf,
    },

    /// Phase 3: Wrap compressed STARK proof into a BN256 SNARK
    ProveSnark {
        /// Path to compression_proof.json
        #[arg(long)]
        compression_proof: PathBuf,

        /// Path to compression_vk.json
        #[arg(long)]
        compression_vk: PathBuf,

        /// Output directory
        #[arg(short, long)]
        output_dir: PathBuf,

        /// Path to trusted setup (CRS) file. If omitted, uses fake crs_42 for testing.
        #[arg(long)]
        trusted_setup: Option<PathBuf>,

        /// Enable zero-knowledge padding in SNARK proving
        #[arg(long)]
        use_zk: bool,
    },

    /// Generate verification keys without a proof (for deployment preparation)
    GenerateVk {
        /// Output directory for VK files
        #[arg(short, long)]
        output_dir: PathBuf,

        /// Path to RISC-V .bin file (omit to use default recursion verifier)
        #[arg(long, requires = "text")]
        bin: Option<PathBuf>,

        /// Path to RISC-V .text file (omit to use default recursion verifier)
        #[arg(long, requires = "bin")]
        text: Option<PathBuf>,

        /// Path to trusted setup (CRS) file. If omitted, uses fake crs_42.
        #[arg(long)]
        trusted_setup: Option<PathBuf>,
    },

    /// Compute the Keccak256 hash of a SNARK verification key
    VkHash {
        /// Path to snark_vk.json
        #[arg(long)]
        vk: PathBuf,
    },

    /// Verify a proof at any pipeline stage
    Verify {
        /// Pipeline stage of the proof
        #[arg(long, value_enum)]
        stage: VerifyStage,

        /// Path to the proof JSON file
        #[arg(long)]
        proof: PathBuf,

        /// Path to the verification key JSON file
        #[arg(long)]
        vk: PathBuf,
    },
}

#[derive(Clone, ValueEnum)]
enum VerifyStage {
    RiscWrapper,
    Compression,
    Snark,
}

impl From<VerifyStage> for zkos_wrapper::interface::VerifyStage {
    fn from(this: VerifyStage) -> Self {
        match this {
            VerifyStage::RiscWrapper => Self::RiscWrapper,
            VerifyStage::Compression => Self::Compression,
            VerifyStage::Snark => Self::Snark,
        }
    }
}

// `INFO` logs are enabled by default, but `RUST_LOG` overrides are supported as well.
fn init_tracing() -> Result<(), Box<dyn Error>> {
    let env_filter = EnvFilter::builder()
        .with_default_directive(LevelFilter::INFO.into())
        .from_env_lossy();

    tracing_subscriber::fmt()
        .with_env_filter(env_filter)
        .try_init()
        .map_err(|error| format!("Failed to initialize tracing subscriber: {error}"))?;

    Ok(())
}

fn main() -> Result<(), Box<dyn Error>> {
    init_tracing()?;

    let cli = Cli::parse();

    match cli.command {
        Commands::ProveAll {
            proof,
            bin,
            text,
            output_dir,
            trusted_setup,
            use_zk,
            save_intermediates,
        } => interface::cmd_prove_all(
            proof,
            bin,
            text,
            output_dir,
            trusted_setup,
            use_zk,
            save_intermediates,
            cli.threads,
        ),

        Commands::ProveRiscWrapper {
            proof,
            bin,
            text,
            output_dir,
        } => interface::cmd_prove_risc_wrapper(proof, bin, text, output_dir, cli.threads),

        Commands::ProveCompression {
            risc_wrapper_proof,
            risc_wrapper_vk,
            output_dir,
        } => interface::cmd_prove_compression(
            risc_wrapper_proof,
            risc_wrapper_vk,
            output_dir,
            cli.threads,
        ),

        Commands::ProveSnark {
            compression_proof,
            compression_vk,
            output_dir,
            trusted_setup,
            use_zk,
        } => interface::cmd_prove_snark(
            compression_proof,
            compression_vk,
            output_dir,
            trusted_setup,
            use_zk,
        ),

        Commands::GenerateVk {
            output_dir,
            bin,
            text,
            trusted_setup,
        } => interface::cmd_generate_vk(output_dir, bin, text, trusted_setup, cli.threads),

        Commands::VkHash { vk } => interface::cmd_vk_hash(vk),

        Commands::Verify { stage, proof, vk } => interface::cmd_verify(stage.into(), proof, vk),
    }
}
