#![feature(allocator_api)]

use boojum::worker::Worker as BoojumWorker;
use clap::{Parser, Subcommand, ValueEnum};
use std::path::{Path, PathBuf};
use std::time::Instant;

#[cfg(not(feature = "gpu"))]
use bellman::kate_commitment::{Crs, CrsForMonomialForm};
#[cfg(not(feature = "gpu"))]
use bellman::worker::Worker as BellmanWorker;

use zkos_wrapper::{
    calculate_verification_key_hash, deserialize_from_file, serialize_to_file,
    circuits::{BinaryCommitment, RiscWrapperWitness},
};
#[cfg(not(feature = "gpu"))]
use zkos_wrapper::{Bn256, L1_VERIFIER_DOMAIN_SIZE_LOG, get_trusted_setup};

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

// ---------------------------------------------------------------------------
// Helpers
// ---------------------------------------------------------------------------

fn create_boojum_worker(threads: Option<usize>) -> BoojumWorker {
    match threads {
        Some(n) => {
            println!("Using {n} worker threads");
            BoojumWorker::new_with_num_threads(n)
        }
        None => BoojumWorker::new(),
    }
}

/// Prints a timing message for a named phase. Call with the label and start instant
/// after the work is done.
fn print_elapsed(label: &str, start: Instant) {
    println!("=== {label}: completed in {:.1}s", start.elapsed().as_secs_f64());
}

fn load_binary_commitment(
    bin: &Option<PathBuf>,
    text: &Option<PathBuf>,
) -> Result<BinaryCommitment, Box<dyn std::error::Error>> {
    match (bin, text) {
        (Some(bin_path), Some(text_path)) => {
            println!("Loading binary from {}", bin_path.display());
            let binary = std::fs::read(bin_path)
                .map_err(|e| format!("Failed to read .bin file {}: {e}", bin_path.display()))?;
            let text_data = std::fs::read(text_path)
                .map_err(|e| format!("Failed to read .text file {}: {e}", text_path.display()))?;
            Ok(BinaryCommitment::from_binary(&binary, &text_data))
        }
        _ => {
            println!("Using default recursion verifier binary");
            Ok(BinaryCommitment::from_default_binary())
        }
    }
}

fn ensure_output_dir(path: &Path) -> Result<(), Box<dyn std::error::Error>> {
    std::fs::create_dir_all(path)
        .map_err(|e| format!("Failed to create output directory {}: {e}", path.display()))?;
    Ok(())
}

#[cfg(not(feature = "gpu"))]
fn load_crs(trusted_setup: &Option<PathBuf>) -> Crs<Bn256, CrsForMonomialForm> {
    match trusted_setup {
        Some(path) => {
            println!("Loading trusted setup from {}", path.display());
            get_trusted_setup(&path.to_string_lossy().to_string())
        }
        None => {
            println!(
                "WARNING: Using fake crs_42 trusted setup (testing only, NOT for production!)"
            );
            Crs::<Bn256, CrsForMonomialForm>::crs_42(
                1 << L1_VERIFIER_DOMAIN_SIZE_LOG,
                &BellmanWorker::new(),
            )
        }
    }
}

fn output_path(dir: &Path, filename: &str) -> String {
    dir.join(filename).to_string_lossy().to_string()
}

// ---------------------------------------------------------------------------
// Phase implementations
// ---------------------------------------------------------------------------

fn run_phase1_risc_wrapper(
    proof_path: &Path,
    bin: &Option<PathBuf>,
    text: &Option<PathBuf>,
    worker: &BoojumWorker,
) -> Result<
    (zkos_wrapper::RiscWrapperProof, zkos_wrapper::RiscWrapperVK),
    Box<dyn std::error::Error>,
> {
    println!("=== Phase 1 - binary commitment: starting...");
    let start = Instant::now();
    let binary_commitment = load_binary_commitment(bin, text)?;
    print_elapsed("Phase 1 - binary commitment", start);

    println!("Loading FRI proof from {}", proof_path.display());
    let program_proof: execution_utils::unrolled::UnrolledProgramProof =
        deserialize_from_file(proof_path.to_str().unwrap());

    println!("=== Phase 1 - witness generation: starting...");
    let start = Instant::now();
    let risc_wrapper_witness =
        RiscWrapperWitness::from_full_proof(program_proof, &binary_commitment);
    print_elapsed("Phase 1 - witness generation", start);

    #[cfg(not(feature = "gpu"))]
    let (risc_wrapper_proof, risc_wrapper_vk) = {
        println!("=== Phase 1 - setup (CPU): starting...");
        let start = Instant::now();
        let (finalization_hint, setup_base, setup, risc_wrapper_vk, setup_tree, vars_hint, witness_hints) =
            zkos_wrapper::get_risc_wrapper_setup(worker, binary_commitment.clone());
        print_elapsed("Phase 1 - setup (CPU)", start);

        println!("=== Phase 1 - prove (CPU): starting...");
        let start = Instant::now();
        let risc_wrapper_proof = zkos_wrapper::prove_risc_wrapper(
            risc_wrapper_witness,
            &finalization_hint,
            &setup_base,
            &setup,
            &risc_wrapper_vk,
            &setup_tree,
            &vars_hint,
            &witness_hints,
            worker,
            binary_commitment,
        );
        print_elapsed("Phase 1 - prove (CPU)", start);

        (risc_wrapper_proof, risc_wrapper_vk)
    };

    #[cfg(feature = "gpu")]
    let (risc_wrapper_proof, risc_wrapper_vk) = {
        println!("=== Phase 1 - setup (GPU): starting...");
        let start = Instant::now();
        let (gpu_setup, gpu_vk, finalization_hint) =
            zkos_wrapper::gpu::risc_wrapper::get_risc_wrapper_setup(worker, binary_commitment.clone());
        print_elapsed("Phase 1 - setup (GPU)", start);

        println!("=== Phase 1 - prove (GPU): starting...");
        let start = Instant::now();
        let risc_wrapper_proof = zkos_wrapper::gpu::risc_wrapper::prove_risc_wrapper(
            risc_wrapper_witness,
            &finalization_hint,
            &gpu_setup,
            &gpu_vk,
            worker,
            binary_commitment,
        );
        print_elapsed("Phase 1 - prove (GPU)", start);

        (risc_wrapper_proof, gpu_vk)
    };

    println!("=== Phase 1 - verify: starting...");
    let start = Instant::now();
    let is_valid = zkos_wrapper::verify_risc_wrapper_proof(&risc_wrapper_proof, &risc_wrapper_vk);
    print_elapsed("Phase 1 - verify", start);
    if !is_valid {
        return Err("RISC wrapper proof verification failed".into());
    }
    println!("Phase 1 proof verified successfully");

    Ok((risc_wrapper_proof, risc_wrapper_vk))
}

fn run_phase2_compression(
    risc_wrapper_proof: zkos_wrapper::RiscWrapperProof,
    risc_wrapper_vk: zkos_wrapper::RiscWrapperVK,
    worker: &BoojumWorker,
) -> Result<
    (zkos_wrapper::CompressionProof, zkos_wrapper::CompressionVK),
    Box<dyn std::error::Error>,
> {
    #[cfg(not(feature = "gpu"))]
    let (compression_proof, compression_vk) = {
        println!("=== Phase 2 - setup (CPU): starting...");
        let start = Instant::now();
        let (finalization_hint, setup_base, setup, compression_vk, setup_tree, vars_hint, witness_hints) =
            zkos_wrapper::get_compression_setup(risc_wrapper_vk.clone(), worker);
        print_elapsed("Phase 2 - setup (CPU)", start);

        println!("=== Phase 2 - prove (CPU): starting...");
        let start = Instant::now();
        let compression_proof = zkos_wrapper::prove_compression(
            risc_wrapper_proof,
            risc_wrapper_vk,
            &finalization_hint,
            &setup_base,
            &setup,
            &compression_vk,
            &setup_tree,
            &vars_hint,
            &witness_hints,
            worker,
        );
        print_elapsed("Phase 2 - prove (CPU)", start);

        (compression_proof, compression_vk)
    };

    #[cfg(feature = "gpu")]
    let (compression_proof, compression_vk) = {
        let config = shivini::ProverContextConfig::default().with_smallest_supported_domain_size(1 << 15);
        let _prover_context = shivini::ProverContext::create_with_config(config).unwrap();

        println!("=== Phase 2 - setup (GPU): starting...");
        let start = Instant::now();
        let (gpu_setup, gpu_vk, finalization_hint) =
            zkos_wrapper::gpu::compression::get_compression_setup(worker, risc_wrapper_vk.clone());
        print_elapsed("Phase 2 - setup (GPU)", start);

        println!("=== Phase 2 - prove (GPU): starting...");
        let start = Instant::now();
        let compression_proof = zkos_wrapper::gpu::compression::prove_compression(
            risc_wrapper_proof,
            risc_wrapper_vk,
            &finalization_hint,
            &gpu_setup,
            &gpu_vk,
            worker,
        );
        print_elapsed("Phase 2 - prove (GPU)", start);

        (compression_proof, gpu_vk)
    };

    println!("=== Phase 2 - verify: starting...");
    let start = Instant::now();
    let is_valid = zkos_wrapper::verify_compression_proof(&compression_proof, &compression_vk);
    print_elapsed("Phase 2 - verify", start);
    if !is_valid {
        return Err("Compression proof verification failed".into());
    }
    println!("Phase 2 proof verified successfully");

    Ok((compression_proof, compression_vk))
}

fn run_phase3_snark(
    compression_proof: zkos_wrapper::CompressionProof,
    compression_vk: zkos_wrapper::CompressionVK,
    trusted_setup: &Option<PathBuf>,
    use_zk: bool,
) -> Result<
    (
        zkos_wrapper::SnarkWrapperProof,
        zkos_wrapper::SnarkWrapperVK,
    ),
    Box<dyn std::error::Error>,
> {
    #[cfg(not(feature = "gpu"))]
    let (snark_proof, snark_vk) = {
        println!("=== Phase 3 - load CRS: starting...");
        let start = Instant::now();
        let crs_mons = load_crs(trusted_setup);
        print_elapsed("Phase 3 - load CRS", start);

        let bellman_worker = BellmanWorker::new();

        println!("=== Phase 3 - setup (CPU): starting...");
        let start = Instant::now();
        let (snark_setup, snark_vk) =
            zkos_wrapper::get_snark_wrapper_setup(compression_vk.clone(), &crs_mons, &bellman_worker);
        print_elapsed("Phase 3 - setup (CPU)", start);

        println!("=== Phase 3 - prove (CPU): starting...");
        let start = Instant::now();
        let snark_proof = zkos_wrapper::prove_snark_wrapper(
            compression_proof,
            compression_vk,
            &snark_setup,
            &crs_mons,
            &bellman_worker,
            use_zk,
        );
        print_elapsed("Phase 3 - prove (CPU)", start);

        (snark_proof, snark_vk)
    };

    #[cfg(feature = "gpu")]
    let (snark_proof, snark_vk) = {
        let crs_file = trusted_setup
            .as_ref()
            .expect("GPU SNARK proving requires a trusted setup file path (--trusted-setup)")
            .to_string_lossy()
            .to_string();

        println!("=== Phase 3 - setup (GPU): starting...");
        let start = Instant::now();
        let (precomputation, snark_vk) =
            zkos_wrapper::gpu::snark::gpu_create_snark_setup_data(&compression_vk, &crs_file);
        print_elapsed("Phase 3 - setup (GPU)", start);

        println!("=== Phase 3 - prove (GPU): starting...");
        let start = Instant::now();
        let snark_proof = zkos_wrapper::gpu::snark::gpu_snark_prove(
            &precomputation,
            &snark_vk,
            compression_proof,
            compression_vk,
            &crs_file,
            use_zk,
        );
        print_elapsed("Phase 3 - prove (GPU)", start);

        (snark_proof, snark_vk)
    };

    println!("=== Phase 3 - verify: starting...");
    let start = Instant::now();
    let is_valid = zkos_wrapper::verify_snark_wrapper_proof(&snark_proof, &snark_vk);
    print_elapsed("Phase 3 - verify", start);
    if !is_valid {
        return Err("SNARK wrapper proof verification failed".into());
    }
    println!("Phase 3 proof verified successfully");

    Ok((snark_proof, snark_vk))
}

// ---------------------------------------------------------------------------
// Subcommand handlers
// ---------------------------------------------------------------------------

fn cmd_prove_all(
    proof: PathBuf,
    bin: Option<PathBuf>,
    text: Option<PathBuf>,
    output_dir: PathBuf,
    trusted_setup: Option<PathBuf>,
    use_zk: bool,
    save_intermediates: bool,
    threads: Option<usize>,
) -> Result<(), Box<dyn std::error::Error>> {
    ensure_output_dir(&output_dir)?;
    let total_start = Instant::now();
    let worker = create_boojum_worker(threads);

    // Phase 1
    let (risc_wrapper_proof, risc_wrapper_vk) =
        run_phase1_risc_wrapper(&proof, &bin, &text, &worker)?;

    if save_intermediates {
        serialize_to_file(
            &risc_wrapper_proof,
            &output_path(&output_dir, "risc_wrapper_proof.json"),
        );
        serialize_to_file(
            &risc_wrapper_vk,
            &output_path(&output_dir, "risc_wrapper_vk.json"),
        );
        println!("Saved intermediate Phase 1 outputs");
    }

    // Phase 2
    let (compression_proof, compression_vk) =
        run_phase2_compression(risc_wrapper_proof, risc_wrapper_vk, &worker)?;

    if save_intermediates {
        serialize_to_file(
            &compression_proof,
            &output_path(&output_dir, "compression_proof.json"),
        );
        serialize_to_file(
            &compression_vk,
            &output_path(&output_dir, "compression_vk.json"),
        );
        println!("Saved intermediate Phase 2 outputs");
    }

    // Phase 3
    let (snark_proof, snark_vk) =
        run_phase3_snark(compression_proof, compression_vk, &trusted_setup, use_zk)?;

    serialize_to_file(&snark_proof, &output_path(&output_dir, "snark_proof.json"));
    serialize_to_file(&snark_vk, &output_path(&output_dir, "snark_vk.json"));

    let vk_hash = calculate_verification_key_hash(snark_vk);
    println!("SNARK VK hash: {vk_hash:?}");

    let total_elapsed = total_start.elapsed();
    println!(
        "=== Total pipeline time: {:.1}s",
        total_elapsed.as_secs_f64()
    );

    Ok(())
}

fn cmd_prove_risc_wrapper(
    proof: PathBuf,
    bin: Option<PathBuf>,
    text: Option<PathBuf>,
    output_dir: PathBuf,
    threads: Option<usize>,
) -> Result<(), Box<dyn std::error::Error>> {
    ensure_output_dir(&output_dir)?;
    let worker = create_boojum_worker(threads);

    let (risc_wrapper_proof, risc_wrapper_vk) =
        run_phase1_risc_wrapper(&proof, &bin, &text, &worker)?;

    serialize_to_file(
        &risc_wrapper_proof,
        &output_path(&output_dir, "risc_wrapper_proof.json"),
    );
    serialize_to_file(
        &risc_wrapper_vk,
        &output_path(&output_dir, "risc_wrapper_vk.json"),
    );

    Ok(())
}

fn cmd_prove_compression(
    risc_wrapper_proof_path: PathBuf,
    risc_wrapper_vk_path: PathBuf,
    output_dir: PathBuf,
    threads: Option<usize>,
) -> Result<(), Box<dyn std::error::Error>> {
    ensure_output_dir(&output_dir)?;
    let worker = create_boojum_worker(threads);

    println!(
        "Loading RISC wrapper proof from {}",
        risc_wrapper_proof_path.display()
    );
    let risc_wrapper_proof = deserialize_from_file(risc_wrapper_proof_path.to_str().unwrap());
    println!(
        "Loading RISC wrapper VK from {}",
        risc_wrapper_vk_path.display()
    );
    let risc_wrapper_vk = deserialize_from_file(risc_wrapper_vk_path.to_str().unwrap());

    let (compression_proof, compression_vk) =
        run_phase2_compression(risc_wrapper_proof, risc_wrapper_vk, &worker)?;

    serialize_to_file(
        &compression_proof,
        &output_path(&output_dir, "compression_proof.json"),
    );
    serialize_to_file(
        &compression_vk,
        &output_path(&output_dir, "compression_vk.json"),
    );

    Ok(())
}

fn cmd_prove_snark(
    compression_proof_path: PathBuf,
    compression_vk_path: PathBuf,
    output_dir: PathBuf,
    trusted_setup: Option<PathBuf>,
    use_zk: bool,
) -> Result<(), Box<dyn std::error::Error>> {
    ensure_output_dir(&output_dir)?;

    println!(
        "Loading compression proof from {}",
        compression_proof_path.display()
    );
    let compression_proof = deserialize_from_file(compression_proof_path.to_str().unwrap());
    println!(
        "Loading compression VK from {}",
        compression_vk_path.display()
    );
    let compression_vk = deserialize_from_file(compression_vk_path.to_str().unwrap());

    let (snark_proof, snark_vk) =
        run_phase3_snark(compression_proof, compression_vk, &trusted_setup, use_zk)?;

    serialize_to_file(&snark_proof, &output_path(&output_dir, "snark_proof.json"));
    serialize_to_file(&snark_vk, &output_path(&output_dir, "snark_vk.json"));

    let vk_hash = calculate_verification_key_hash(snark_vk);
    println!("SNARK VK hash: {vk_hash:?}");

    Ok(())
}

fn cmd_generate_vk(
    output_dir: PathBuf,
    bin: Option<PathBuf>,
    text: Option<PathBuf>,
    trusted_setup: Option<PathBuf>,
    threads: Option<usize>,
) -> Result<(), Box<dyn std::error::Error>> {
    ensure_output_dir(&output_dir)?;
    let worker = create_boojum_worker(threads);

    // Phase 1: RISC wrapper VK
    println!("=== VK generation - binary commitment: starting...");
    let start = Instant::now();
    let binary_commitment = load_binary_commitment(&bin, &text)?;
    print_elapsed("VK generation - binary commitment", start);

    println!("=== VK generation - Phase 1 (RISC wrapper): starting...");
    let start = Instant::now();
    #[cfg(not(feature = "gpu"))]
    let risc_wrapper_vk = {
        let (_, _, _, risc_wrapper_vk, _, _, _) =
            zkos_wrapper::get_risc_wrapper_setup(&worker, binary_commitment);
        risc_wrapper_vk
    };
    #[cfg(feature = "gpu")]
    let risc_wrapper_vk = {
        let (_, gpu_vk, _) =
            zkos_wrapper::gpu::risc_wrapper::get_risc_wrapper_setup(&worker, binary_commitment);
        gpu_vk
    };
    print_elapsed("VK generation - Phase 1 (RISC wrapper)", start);
    serialize_to_file(&risc_wrapper_vk, &output_path(&output_dir, "risc_wrapper_vk.json"));
    println!("Saved risc_wrapper_vk.json");

    // Phase 2: Compression VK
    println!("=== VK generation - Phase 2 (compression): starting...");
    let start = Instant::now();
    #[cfg(not(feature = "gpu"))]
    let compression_vk = {
        let (_, _, _, compression_vk, _, _, _) =
            zkos_wrapper::get_compression_setup(risc_wrapper_vk, &worker);
        compression_vk
    };
    #[cfg(feature = "gpu")]
    let compression_vk = {
        let config = shivini::ProverContextConfig::default().with_smallest_supported_domain_size(1 << 15);
        let _prover_context = shivini::ProverContext::create_with_config(config).unwrap();

        let (_, gpu_vk, _) =
            zkos_wrapper::gpu::compression::get_compression_setup(&worker, risc_wrapper_vk);
        gpu_vk
    };
    print_elapsed("VK generation - Phase 2 (compression)", start);
    serialize_to_file(&compression_vk, &output_path(&output_dir, "compression_vk.json"));
    println!("Saved compression_vk.json");

    // Phase 3: SNARK VK
    println!("=== VK generation - Phase 3 (SNARK): starting...");
    let start = Instant::now();
    #[cfg(not(feature = "gpu"))]
    let snark_vk = {
        let crs_mons = load_crs(&trusted_setup);
        let bellman_worker = BellmanWorker::new();
        let (_, snark_vk) =
            zkos_wrapper::get_snark_wrapper_setup(compression_vk, &crs_mons, &bellman_worker);
        snark_vk
    };
    #[cfg(feature = "gpu")]
    let snark_vk = {
        let crs_file = trusted_setup
            .as_ref()
            .expect("GPU VK generation requires a trusted setup file path (--trusted-setup)")
            .to_string_lossy()
            .to_string();
        let (_, snark_vk) =
            zkos_wrapper::gpu::snark::gpu_create_snark_setup_data(&compression_vk, &crs_file);
        snark_vk
    };
    print_elapsed("VK generation - Phase 3 (SNARK)", start);
    serialize_to_file(&snark_vk, &output_path(&output_dir, "snark_vk.json"));
    println!("Saved snark_vk.json");

    let vk_hash = calculate_verification_key_hash(snark_vk);
    println!("SNARK VK hash: {vk_hash:?}");

    Ok(())
}

fn cmd_vk_hash(vk_path: PathBuf) -> Result<(), Box<dyn std::error::Error>> {
    println!("Loading VK from {}", vk_path.display());
    let vk = deserialize_from_file(vk_path.to_str().unwrap());
    let vk_hash = calculate_verification_key_hash(vk);
    println!("SNARK VK hash: {vk_hash:?}");
    Ok(())
}

fn cmd_verify(
    stage: VerifyStage,
    proof_path: PathBuf,
    vk_path: PathBuf,
) -> Result<(), Box<dyn std::error::Error>> {
    let proof_str = proof_path.to_str().unwrap();
    let vk_str = vk_path.to_str().unwrap();

    let is_valid = match stage {
        VerifyStage::RiscWrapper => {
            println!("Verifying RISC wrapper proof...");
            let proof = deserialize_from_file(proof_str);
            let vk = deserialize_from_file(vk_str);
            zkos_wrapper::verify_risc_wrapper_proof(&proof, &vk)
        }
        VerifyStage::Compression => {
            println!("Verifying compression proof...");
            let proof = deserialize_from_file(proof_str);
            let vk = deserialize_from_file(vk_str);
            zkos_wrapper::verify_compression_proof(&proof, &vk)
        }
        VerifyStage::Snark => {
            println!("Verifying SNARK proof...");
            let proof = deserialize_from_file(proof_str);
            let vk = deserialize_from_file(vk_str);
            zkos_wrapper::verify_snark_wrapper_proof(&proof, &vk)
        }
    };

    if is_valid {
        println!("Proof is VALID");
        Ok(())
    } else {
        Err("Proof verification FAILED".into())
    }
}

// ---------------------------------------------------------------------------
// Main
// ---------------------------------------------------------------------------

fn main() -> Result<(), Box<dyn std::error::Error>> {
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
        } => cmd_prove_all(
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
        } => cmd_prove_risc_wrapper(proof, bin, text, output_dir, cli.threads),

        Commands::ProveCompression {
            risc_wrapper_proof,
            risc_wrapper_vk,
            output_dir,
        } => cmd_prove_compression(risc_wrapper_proof, risc_wrapper_vk, output_dir, cli.threads),

        Commands::ProveSnark {
            compression_proof,
            compression_vk,
            output_dir,
            trusted_setup,
            use_zk,
        } => cmd_prove_snark(
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
        } => cmd_generate_vk(output_dir, bin, text, trusted_setup, cli.threads),

        Commands::VkHash { vk } => cmd_vk_hash(vk),

        Commands::Verify { stage, proof, vk } => cmd_verify(stage, proof, vk),
    }
}
