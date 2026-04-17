use boojum::worker::Worker as BoojumWorker;
use execution_utils::unrolled::UnrolledProgramProof;
use std::path::PathBuf;
use std::time::Instant;

#[cfg(not(feature = "gpu"))]
use super::utils::load_crs;
#[cfg(not(feature = "gpu"))]
use bellman::worker::Worker as BellmanWorker;

use crate::circuits::RiscWrapperWitness;

use super::utils::{load_binary_commitment, print_elapsed};

#[derive(Clone)]
pub enum VerifyStage {
    RiscWrapper,
    Compression,
    Snark,
}

// ==============================================================================
// Pipeline Phase Implementations
// ==============================================================================
//
// Each phase function owns one boundary in the wrapper pipeline. The command
// layer is responsible for CLI-oriented concerns such as paths and persistence,
// while this module focuses on the actual proving, setup, and verification flow.

pub fn run_phase1_risc_wrapper(
    program_proof: UnrolledProgramProof,
    bin: &Option<PathBuf>,
    text: &Option<PathBuf>,
    worker: &BoojumWorker,
) -> Result<(crate::RiscWrapperProof, crate::RiscWrapperVK), Box<dyn std::error::Error>> {
    tracing::info!("=== Phase 1 - binary commitment: starting...");
    let start = Instant::now();
    let binary_commitment = load_binary_commitment(bin, text)?;
    print_elapsed("Phase 1 - binary commitment", start);

    tracing::info!("=== Phase 1 - witness generation: starting...");
    let start = Instant::now();
    let risc_wrapper_witness =
        RiscWrapperWitness::from_full_proof(program_proof, &binary_commitment);
    print_elapsed("Phase 1 - witness generation", start);

    #[cfg(not(feature = "gpu"))]
    let (risc_wrapper_proof, risc_wrapper_vk) = {
        tracing::info!("=== Phase 1 - setup (CPU): starting...");
        let start = Instant::now();
        let (
            finalization_hint,
            setup_base,
            setup,
            risc_wrapper_vk,
            setup_tree,
            vars_hint,
            witness_hints,
        ) = crate::get_risc_wrapper_setup(worker, binary_commitment.clone());
        print_elapsed("Phase 1 - setup (CPU)", start);

        tracing::info!("=== Phase 1 - prove (CPU): starting...");
        let start = Instant::now();
        let risc_wrapper_proof = crate::prove_risc_wrapper(
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
        tracing::info!("=== Phase 1 - setup (GPU): starting...");
        let start = Instant::now();
        let (gpu_setup, gpu_vk, finalization_hint) =
            crate::gpu::risc_wrapper::get_risc_wrapper_setup(worker, binary_commitment.clone());
        print_elapsed("Phase 1 - setup (GPU)", start);

        tracing::info!("=== Phase 1 - prove (GPU): starting...");
        let start = Instant::now();
        let risc_wrapper_proof = crate::gpu::risc_wrapper::prove_risc_wrapper(
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

    tracing::info!("=== Phase 1 - verify: starting...");
    let start = Instant::now();
    let is_valid = crate::verify_risc_wrapper_proof(&risc_wrapper_proof, &risc_wrapper_vk);
    print_elapsed("Phase 1 - verify", start);
    if !is_valid {
        return Err("RISC wrapper proof verification failed".into());
    }
    tracing::info!("Phase 1 proof verified successfully");

    Ok((risc_wrapper_proof, risc_wrapper_vk))
}

pub fn run_phase2_compression(
    risc_wrapper_proof: crate::RiscWrapperProof,
    risc_wrapper_vk: crate::RiscWrapperVK,
    worker: &BoojumWorker,
) -> Result<(crate::CompressionProof, crate::CompressionVK), Box<dyn std::error::Error>> {
    #[cfg(not(feature = "gpu"))]
    let (compression_proof, compression_vk) = {
        tracing::info!("=== Phase 2 - setup (CPU): starting...");
        let start = Instant::now();
        let (
            finalization_hint,
            setup_base,
            setup,
            compression_vk,
            setup_tree,
            vars_hint,
            witness_hints,
        ) = crate::get_compression_setup(risc_wrapper_vk.clone(), worker);
        print_elapsed("Phase 2 - setup (CPU)", start);

        tracing::info!("=== Phase 2 - prove (CPU): starting...");
        let start = Instant::now();
        let compression_proof = crate::prove_compression(
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
        let config =
            shivini::ProverContextConfig::default().with_smallest_supported_domain_size(1 << 15);
        let _prover_context = shivini::ProverContext::create_with_config(config).unwrap();

        tracing::info!("=== Phase 2 - setup (GPU): starting...");
        let start = Instant::now();
        let (gpu_setup, gpu_vk, finalization_hint) =
            crate::gpu::compression::get_compression_setup(worker, risc_wrapper_vk.clone());
        print_elapsed("Phase 2 - setup (GPU)", start);

        tracing::info!("=== Phase 2 - prove (GPU): starting...");
        let start = Instant::now();
        let compression_proof = crate::gpu::compression::prove_compression(
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

    tracing::info!("=== Phase 2 - verify: starting...");
    let start = Instant::now();
    let is_valid = crate::verify_compression_proof(&compression_proof, &compression_vk);
    print_elapsed("Phase 2 - verify", start);
    if !is_valid {
        return Err("Compression proof verification failed".into());
    }
    tracing::info!("Phase 2 proof verified successfully");

    Ok((compression_proof, compression_vk))
}

pub fn run_phase3_snark(
    compression_proof: crate::CompressionProof,
    compression_vk: crate::CompressionVK,
    trusted_setup: &Option<PathBuf>,
    use_zk: bool,
) -> Result<(crate::SnarkWrapperProof, crate::SnarkWrapperVK), Box<dyn std::error::Error>> {
    #[cfg(not(feature = "gpu"))]
    let (snark_proof, snark_vk) = {
        tracing::info!("=== Phase 3 - load CRS: starting...");
        let start = Instant::now();
        let crs_mons = load_crs(trusted_setup);
        print_elapsed("Phase 3 - load CRS", start);

        let bellman_worker = BellmanWorker::new();

        tracing::info!("=== Phase 3 - setup (CPU): starting...");
        let start = Instant::now();
        let (snark_setup, snark_vk) =
            crate::get_snark_wrapper_setup(compression_vk.clone(), &crs_mons, &bellman_worker);
        print_elapsed("Phase 3 - setup (CPU)", start);

        tracing::info!("=== Phase 3 - prove (CPU): starting...");
        let start = Instant::now();
        let snark_proof = crate::prove_snark_wrapper(
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

        tracing::info!("=== Phase 3 - setup (GPU): starting...");
        let start = Instant::now();
        let (precomputation, snark_vk) =
            crate::gpu::snark::gpu_create_snark_setup_data(&compression_vk, &crs_file);
        print_elapsed("Phase 3 - setup (GPU)", start);

        tracing::info!("=== Phase 3 - prove (GPU): starting...");
        let start = Instant::now();
        let snark_proof = crate::gpu::snark::gpu_snark_prove(
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

    tracing::info!("=== Phase 3 - verify: starting...");
    let start = Instant::now();
    let is_valid = crate::verify_snark_wrapper_proof(&snark_proof, &snark_vk);
    print_elapsed("Phase 3 - verify", start);
    if !is_valid {
        return Err("SNARK wrapper proof verification failed".into());
    }
    tracing::info!("Phase 3 proof verified successfully");

    Ok((snark_proof, snark_vk))
}
