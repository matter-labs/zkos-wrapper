use boojum::worker::Worker as BoojumWorker;
use std::path::{Path, PathBuf};
use std::time::Instant;

#[cfg(not(feature = "gpu"))]
use bellman::kate_commitment::{Crs, CrsForMonomialForm};
#[cfg(not(feature = "gpu"))]
use bellman::worker::Worker as BellmanWorker;

use crate::circuits::BinaryCommitment;
#[cfg(not(feature = "gpu"))]
use crate::{Bn256, L1_VERIFIER_DOMAIN_SIZE_LOG, get_trusted_setup};

// ==============================================================================
// Local Execution Helpers
// ==============================================================================
//
// The interface commands mostly orchestrate already-existing proving APIs.
// These helpers keep the orchestration code focused on the pipeline itself:
// worker selection, filesystem setup, shared timing logs, and loading optional
// assets that are reused across multiple commands.

pub(super) fn create_boojum_worker(threads: Option<usize>) -> BoojumWorker {
    match threads {
        Some(n) => {
            tracing::info!("Using {n} worker threads");
            BoojumWorker::new_with_num_threads(n)
        }
        None => BoojumWorker::new(),
    }
}

/// Emits a consistent timing line after a named step completes.
pub(super) fn print_elapsed(label: &str, start: Instant) {
    tracing::info!(
        "=== {label}: completed in {:.1}s",
        start.elapsed().as_secs_f64()
    );
}

/// Loads the recursion verifier binary when the caller overrides the defaults.
///
/// The pipeline always needs a `BinaryCommitment`, but CLI callers are allowed
/// to either provide both binary artifacts or rely on the built-in verifier.
pub(super) fn load_binary_commitment(
    bin: &Option<PathBuf>,
    text: &Option<PathBuf>,
) -> Result<BinaryCommitment, Box<dyn std::error::Error>> {
    match (bin, text) {
        (Some(bin_path), Some(text_path)) => {
            tracing::info!("Loading binary from {}", bin_path.display());
            let binary = std::fs::read(bin_path)
                .map_err(|e| format!("Failed to read .bin file {}: {e}", bin_path.display()))?;
            let text_data = std::fs::read(text_path)
                .map_err(|e| format!("Failed to read .text file {}: {e}", text_path.display()))?;
            Ok(BinaryCommitment::from_binary(&binary, &text_data))
        }
        _ => {
            tracing::info!("Using default recursion verifier binary");
            Ok(BinaryCommitment::from_default_binary())
        }
    }
}

pub(super) fn ensure_output_dir(path: &Path) -> Result<(), Box<dyn std::error::Error>> {
    std::fs::create_dir_all(path)
        .map_err(|e| format!("Failed to create output directory {}: {e}", path.display()))?;
    Ok(())
}

#[cfg(not(feature = "gpu"))]
pub(super) fn load_crs(trusted_setup: &Option<PathBuf>) -> Crs<Bn256, CrsForMonomialForm> {
    match trusted_setup {
        Some(path) => {
            tracing::info!("Loading trusted setup from {}", path.display());
            get_trusted_setup(&path.to_string_lossy().to_string())
        }
        None => {
            tracing::info!(
                "WARNING: Using fake crs_42 trusted setup (testing only, NOT for production!)"
            );
            Crs::<Bn256, CrsForMonomialForm>::crs_42(
                1 << L1_VERIFIER_DOMAIN_SIZE_LOG,
                &BellmanWorker::new(),
            )
        }
    }
}

pub(super) fn output_path(dir: &Path, filename: &str) -> String {
    dir.join(filename).to_string_lossy().to_string()
}
