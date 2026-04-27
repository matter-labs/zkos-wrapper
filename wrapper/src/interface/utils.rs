use execution_utils::unrolled::UnrolledProgramProof;
use std::path::Path;
use std::time::Instant;

use crate::deserialize_from_file;

/// Emits a consistent timing line after a named step completes.
pub(super) fn print_elapsed(label: &str, start: Instant) {
    tracing::info!(
        "=== {label}: completed in {:.1}s",
        start.elapsed().as_secs_f64()
    );
}

/// Loads and deserializes the proof from file.
pub fn load_proof(proof_path: &Path) -> anyhow::Result<UnrolledProgramProof> {
    tracing::info!("Loading FRI proof from {}", proof_path.display());
    deserialize_from_file(proof_path.to_str().expect("Non-unicode file path"))
}

pub(super) fn ensure_output_dir(path: &Path) -> anyhow::Result<()> {
    std::fs::create_dir_all(path).map_err(|e| {
        anyhow::anyhow!("Failed to create output directory {}: {e}", path.display())
    })?;
    Ok(())
}

pub(super) fn output_path(dir: &Path, filename: &str) -> String {
    dir.join(filename).to_string_lossy().to_string()
}
