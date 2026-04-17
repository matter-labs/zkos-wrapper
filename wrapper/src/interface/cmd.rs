use std::path::PathBuf;
use std::time::Instant;

use anyhow::Context as _;

#[cfg(not(feature = "gpu"))]
use super::utils::load_crs;
#[cfg(not(feature = "gpu"))]
use bellman::worker::Worker as BellmanWorker;

use crate::{calculate_verification_key_hash, deserialize_from_file, serialize_to_file};

use super::phases::{
    VerifyStage, run_phase1_risc_wrapper, run_phase2_compression, run_phase3_snark,
};
use super::utils::{
    create_boojum_worker, ensure_output_dir, load_binary_commitment, load_proof, output_path,
    print_elapsed,
};

// ==============================================================================
// Public Command Entry Points
// ==============================================================================
//
// These functions represent the "commands" that can be done via CLI; you can
// treat them as reusable high-level entrypoints. However, outside of CLI-like
// utilities, it is recommended to use `phases` module directly instead.

pub fn cmd_prove_all(
    proof: PathBuf,
    bin: Option<PathBuf>,
    text: Option<PathBuf>,
    output_dir: PathBuf,
    trusted_setup: Option<PathBuf>,
    use_zk: bool,
    save_intermediates: bool,
    threads: Option<usize>,
) -> anyhow::Result<()> {
    ensure_output_dir(&output_dir)?;
    let total_start = Instant::now();
    let worker = create_boojum_worker(threads);
    let program_proof = load_proof(&proof).context("Can't load the proof")?;

    // Drive the full pipeline in order so callers can optionally persist the
    // boundaries between phases for later debugging or reuse.
    let (risc_wrapper_proof, risc_wrapper_vk) =
        run_phase1_risc_wrapper(program_proof, &bin, &text, &worker)?;

    if save_intermediates {
        serialize_to_file(
            &risc_wrapper_proof,
            &output_path(&output_dir, "risc_wrapper_proof.json"),
        )?;
        serialize_to_file(
            &risc_wrapper_vk,
            &output_path(&output_dir, "risc_wrapper_vk.json"),
        )?;
        tracing::info!("Saved intermediate Phase 1 outputs");
    }

    let (compression_proof, compression_vk) =
        run_phase2_compression(risc_wrapper_proof, risc_wrapper_vk, &worker)?;

    if save_intermediates {
        serialize_to_file(
            &compression_proof,
            &output_path(&output_dir, "compression_proof.json"),
        )?;
        serialize_to_file(
            &compression_vk,
            &output_path(&output_dir, "compression_vk.json"),
        )?;
        tracing::info!("Saved intermediate Phase 2 outputs");
    }

    let (snark_proof, snark_vk) =
        run_phase3_snark(compression_proof, compression_vk, &trusted_setup, use_zk)?;

    serialize_to_file(&snark_proof, &output_path(&output_dir, "snark_proof.json"))?;
    serialize_to_file(&snark_vk, &output_path(&output_dir, "snark_vk.json"))?;

    let vk_hash = calculate_verification_key_hash(snark_vk);
    tracing::info!("SNARK VK hash: {vk_hash:?}");

    let total_elapsed = total_start.elapsed();
    tracing::info!(
        "=== Total pipeline time: {:.1}s",
        total_elapsed.as_secs_f64()
    );

    Ok(())
}

pub fn cmd_prove_risc_wrapper(
    proof: PathBuf,
    bin: Option<PathBuf>,
    text: Option<PathBuf>,
    output_dir: PathBuf,
    threads: Option<usize>,
) -> anyhow::Result<()> {
    ensure_output_dir(&output_dir)?;
    let worker = create_boojum_worker(threads);
    let program_proof = load_proof(&proof).context("Can't load the proof")?;

    let (risc_wrapper_proof, risc_wrapper_vk) =
        run_phase1_risc_wrapper(program_proof, &bin, &text, &worker)?;

    serialize_to_file(
        &risc_wrapper_proof,
        &output_path(&output_dir, "risc_wrapper_proof.json"),
    )?;
    serialize_to_file(
        &risc_wrapper_vk,
        &output_path(&output_dir, "risc_wrapper_vk.json"),
    )?;

    Ok(())
}

pub fn cmd_prove_compression(
    risc_wrapper_proof_path: PathBuf,
    risc_wrapper_vk_path: PathBuf,
    output_dir: PathBuf,
    threads: Option<usize>,
) -> anyhow::Result<()> {
    ensure_output_dir(&output_dir)?;
    let worker = create_boojum_worker(threads);

    tracing::info!(
        "Loading RISC wrapper proof from {}",
        risc_wrapper_proof_path.display()
    );
    let risc_wrapper_proof = deserialize_from_file(risc_wrapper_proof_path.to_str().unwrap())
        .context("risk_wrapper_proof")?;
    tracing::info!(
        "Loading RISC wrapper VK from {}",
        risc_wrapper_vk_path.display()
    );
    let risc_wrapper_vk =
        deserialize_from_file(risc_wrapper_vk_path.to_str().unwrap()).context("risk_wrapper_vk")?;

    let (compression_proof, compression_vk) =
        run_phase2_compression(risc_wrapper_proof, risc_wrapper_vk, &worker)?;

    serialize_to_file(
        &compression_proof,
        &output_path(&output_dir, "compression_proof.json"),
    )?;
    serialize_to_file(
        &compression_vk,
        &output_path(&output_dir, "compression_vk.json"),
    )?;

    Ok(())
}

pub fn cmd_prove_snark(
    compression_proof_path: PathBuf,
    compression_vk_path: PathBuf,
    output_dir: PathBuf,
    trusted_setup: Option<PathBuf>,
    use_zk: bool,
) -> anyhow::Result<()> {
    ensure_output_dir(&output_dir)?;

    tracing::info!(
        "Loading compression proof from {}",
        compression_proof_path.display()
    );
    let compression_proof = deserialize_from_file(compression_proof_path.to_str().unwrap())
        .context("compression proof")?;
    tracing::info!(
        "Loading compression VK from {}",
        compression_vk_path.display()
    );
    let compression_vk =
        deserialize_from_file(compression_vk_path.to_str().unwrap()).context("compression vk")?;

    let (snark_proof, snark_vk) =
        run_phase3_snark(compression_proof, compression_vk, &trusted_setup, use_zk)?;

    serialize_to_file(&snark_proof, &output_path(&output_dir, "snark_proof.json"))?;
    serialize_to_file(&snark_vk, &output_path(&output_dir, "snark_vk.json"))?;

    let vk_hash = calculate_verification_key_hash(snark_vk);
    tracing::info!("SNARK VK hash: {vk_hash:?}");

    Ok(())
}

pub fn cmd_generate_vk(
    output_dir: PathBuf,
    bin: Option<PathBuf>,
    text: Option<PathBuf>,
    trusted_setup: Option<PathBuf>,
    threads: Option<usize>,
) -> anyhow::Result<()> {
    ensure_output_dir(&output_dir)?;
    let worker = create_boojum_worker(threads);

    // Build the verification key chain directly without requiring a proof input.
    tracing::info!("=== VK generation - binary commitment: starting...");
    let start = Instant::now();
    let binary_commitment = load_binary_commitment(&bin, &text).context("binary commitment")?;
    print_elapsed("VK generation - binary commitment", start);

    tracing::info!("=== VK generation - Phase 1 (RISC wrapper): starting...");
    let start = Instant::now();
    #[cfg(not(feature = "gpu"))]
    let risc_wrapper_vk = {
        let (_, _, _, risc_wrapper_vk, _, _, _) =
            crate::get_risc_wrapper_setup(&worker, binary_commitment);
        risc_wrapper_vk
    };
    #[cfg(feature = "gpu")]
    let risc_wrapper_vk = {
        let (_, gpu_vk, _) =
            crate::gpu::risc_wrapper::get_risc_wrapper_setup(&worker, binary_commitment);
        gpu_vk
    };
    print_elapsed("VK generation - Phase 1 (RISC wrapper)", start);
    serialize_to_file(
        &risc_wrapper_vk,
        &output_path(&output_dir, "risc_wrapper_vk.json"),
    )?;
    tracing::info!("Saved risc_wrapper_vk.json");

    tracing::info!("=== VK generation - Phase 2 (compression): starting...");
    let start = Instant::now();
    #[cfg(not(feature = "gpu"))]
    let compression_vk = {
        let (_, _, _, compression_vk, _, _, _) =
            crate::get_compression_setup(risc_wrapper_vk, &worker);
        compression_vk
    };
    #[cfg(feature = "gpu")]
    let compression_vk = {
        let config =
            shivini::ProverContextConfig::default().with_smallest_supported_domain_size(1 << 15);
        let _prover_context = shivini::ProverContext::create_with_config(config).unwrap();

        let (_, gpu_vk, _) =
            crate::gpu::compression::get_compression_setup(&worker, risc_wrapper_vk);
        gpu_vk
    };
    print_elapsed("VK generation - Phase 2 (compression)", start);
    serialize_to_file(
        &compression_vk,
        &output_path(&output_dir, "compression_vk.json"),
    )?;
    tracing::info!("Saved compression_vk.json");

    tracing::info!("=== VK generation - Phase 3 (SNARK): starting...");
    let start = Instant::now();
    #[cfg(not(feature = "gpu"))]
    let snark_vk = {
        let crs_mons = load_crs(&trusted_setup)?;
        let bellman_worker = BellmanWorker::new();
        let (_, snark_vk) =
            crate::get_snark_wrapper_setup(compression_vk, &crs_mons, &bellman_worker);
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
            crate::gpu::snark::gpu_create_snark_setup_data(&compression_vk, &crs_file);
        snark_vk
    };
    print_elapsed("VK generation - Phase 3 (SNARK)", start);
    serialize_to_file(&snark_vk, &output_path(&output_dir, "snark_vk.json"))?;
    tracing::info!("Saved snark_vk.json");

    let vk_hash = calculate_verification_key_hash(snark_vk);
    tracing::info!("SNARK VK hash: {vk_hash:?}");

    Ok(())
}

pub fn cmd_vk_hash(vk_path: PathBuf) -> anyhow::Result<()> {
    tracing::info!("Loading VK from {}", vk_path.display());
    let vk = deserialize_from_file(vk_path.to_str().unwrap())?;
    let vk_hash = calculate_verification_key_hash(vk);
    tracing::info!("SNARK VK hash: {vk_hash:?}");
    Ok(())
}

pub fn cmd_verify(stage: VerifyStage, proof_path: PathBuf, vk_path: PathBuf) -> anyhow::Result<()> {
    let proof_str = proof_path.to_str().unwrap();
    let vk_str = vk_path.to_str().unwrap();

    let is_valid = match stage {
        VerifyStage::RiscWrapper => {
            tracing::info!("Verifying RISC wrapper proof...");
            let proof = deserialize_from_file(proof_str).context("proof")?;
            let vk = deserialize_from_file(vk_str).context("vk")?;
            crate::verify_risc_wrapper_proof(&proof, &vk)
        }
        VerifyStage::Compression => {
            tracing::info!("Verifying compression proof...");
            let proof = deserialize_from_file(proof_str).context("proof")?;
            let vk = deserialize_from_file(vk_str).context("vk")?;
            crate::verify_compression_proof(&proof, &vk)
        }
        VerifyStage::Snark => {
            tracing::info!("Verifying SNARK proof...");
            let proof = deserialize_from_file(proof_str).context("proof")?;
            let vk = deserialize_from_file(vk_str).context("vk")?;
            crate::verify_snark_wrapper_proof(&proof, &vk)
        }
    };

    if is_valid {
        tracing::info!("Proof is VALID");
        Ok(())
    } else {
        Err(anyhow::anyhow!("Proof verification FAILED"))
    }
}
