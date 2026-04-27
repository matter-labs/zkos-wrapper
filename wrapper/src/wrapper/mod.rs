use std::path::PathBuf;
use std::time::Instant;

use anyhow::Context as _;
use execution_utils::unrolled::UnrolledProgramProof;

use crate::circuits::{BinaryCommitment, RiscWrapperWitness};
use crate::{
    BoojumWorker, CompressionProof, CompressionVK, RiscWrapperProof, RiscWrapperVK,
    SnarkWrapperProof, SnarkWrapperVK,
};

#[cfg(not(feature = "gpu"))]
mod cpu;
#[cfg(feature = "gpu")]
mod gpu;

#[cfg(not(feature = "gpu"))]
use self::cpu as backend;
#[cfg(feature = "gpu")]
use self::gpu as backend;

// ==============================================================================
// Public Wrapper Configuration
// ==============================================================================
//
// A wrapper instance represents one stable proving environment. We keep the
// configuration intentionally small for now so the public API tracks the
// concrete inputs that determine the reusable setup chain.

#[derive(Clone, Debug, Default)]
pub struct SnarkWrapperConfig {
    pub bin: Option<PathBuf>,
    pub text: Option<PathBuf>,
    pub trusted_setup: Option<PathBuf>,
    pub threads: Option<usize>,
    /// Trusted phase-1 VK to be reused instead of deriving it from the binary.
    pub risc_wrapper_vk: Option<RiscWrapperVK>,
    /// Trusted phase-2 VK to be reused instead of deriving it from the phase-1 VK.
    pub compression_vk: Option<CompressionVK>,
    /// Trusted phase-3 VK to be reused instead of deriving it from the SNARK setup.
    pub snark_vk: Option<SnarkWrapperVK>,
}

// ==============================================================================
// Stateful Wrapper API
// ==============================================================================
//
// The wrapper owns only the stable session-level state that is worth caching:
// the binary commitment and the backend-specific setup chain. Proof artifacts
// stay outside so library callers remain free to serialize or route them as
// they see fit.

pub struct SnarkWrapper {
    config: SnarkWrapperConfig,
    binary_commitment: Option<BinaryCommitment>,
    worker: BoojumWorker,
    backend: backend::BackendState,
}

impl SnarkWrapper {
    pub fn new(config: SnarkWrapperConfig) -> anyhow::Result<Self> {
        if config.bin.is_some() != config.text.is_some() {
            anyhow::bail!("Both --bin and --text must be provided together");
        }
        validate_trusted_setup_file(&config.trusted_setup)?;

        let worker = create_boojum_worker(config.threads);

        Ok(Self {
            config,
            binary_commitment: None,
            worker,
            backend: backend::BackendState::new(),
        })
    }

    /// Proves phase 1 and verifies the generated RISC wrapper proof before returning it.
    pub fn prove_risc_wrapper(
        &mut self,
        program_proof: UnrolledProgramProof,
    ) -> anyhow::Result<RiscWrapperProof> {
        let binary_commitment = self.binary_commitment()?;
        let start = Instant::now();
        let witness = RiscWrapperWitness::from_full_proof(program_proof, &binary_commitment);
        tracing::info!(
            "Phase 1 witness generation took {:.3}s",
            start.elapsed().as_secs_f64()
        );

        let proof = self
            .backend
            .prove_risc_wrapper(witness, binary_commitment, &self.worker)?;
        let start = Instant::now();
        if !self
            .verify_risc_wrapper(&proof)
            .context("while attempting to verify generated RISC wrapper proof")?
        {
            anyhow::bail!("RISC wrapper proof verification failed");
        }
        tracing::info!(
            "Phase 1 verification took {:.3}s",
            start.elapsed().as_secs_f64()
        );

        Ok(proof)
    }

    /// Verifies an existing phase-1 proof against this wrapper's resolved RISC wrapper VK.
    pub fn verify_risc_wrapper(&mut self, proof: &RiscWrapperProof) -> anyhow::Result<bool> {
        Ok(crate::verify_risc_wrapper_proof(
            proof,
            self.risc_wrapper_vk()?,
        ))
    }

    pub fn risc_wrapper_vk(&mut self) -> anyhow::Result<&RiscWrapperVK> {
        if self.config.risc_wrapper_vk.is_some() {
            return Ok(self
                .config
                .risc_wrapper_vk
                .as_ref()
                .expect("phase 1 config VK exists"));
        }

        if self.backend.cached_risc_wrapper_vk().is_some() {
            return Ok(self
                .backend
                .cached_risc_wrapper_vk()
                .expect("phase 1 cache VK exists"));
        }

        let binary_commitment = self.binary_commitment()?;

        self.backend
            .risc_wrapper_vk(binary_commitment, &self.worker)
    }

    /// Proves phase 2 and verifies the generated compression proof before returning it.
    pub fn prove_compression(
        &mut self,
        risc_wrapper_proof: RiscWrapperProof,
    ) -> anyhow::Result<CompressionProof> {
        let risc_wrapper_vk = self.risc_wrapper_vk()?.clone();

        let proof =
            self.backend
                .prove_compression(risc_wrapper_proof, risc_wrapper_vk, &self.worker)?;
        let start = Instant::now();
        if !self
            .verify_compression(&proof)
            .context("while attempting to verify generated compression proof")?
        {
            anyhow::bail!("Compression proof verification failed");
        }
        tracing::info!(
            "Phase 2 verification took {:.3}s",
            start.elapsed().as_secs_f64()
        );

        Ok(proof)
    }

    /// Verifies an existing phase-2 proof against this wrapper's resolved compression VK.
    pub fn verify_compression(&mut self, proof: &CompressionProof) -> anyhow::Result<bool> {
        Ok(crate::verify_compression_proof(
            proof,
            self.compression_vk()?,
        ))
    }

    pub fn compression_vk(&mut self) -> anyhow::Result<&CompressionVK> {
        if self.config.compression_vk.is_some() {
            return Ok(self
                .config
                .compression_vk
                .as_ref()
                .expect("phase 2 config VK exists"));
        }

        if self.backend.cached_compression_vk().is_some() {
            return Ok(self
                .backend
                .cached_compression_vk()
                .expect("phase 2 cache VK exists"));
        }

        let risc_wrapper_vk = self.risc_wrapper_vk()?.clone();

        self.backend.compression_vk(risc_wrapper_vk, &self.worker)
    }

    /// Proves phase 3 and verifies the generated SNARK proof before returning it.
    pub fn prove_snark(
        &mut self,
        compression_proof: CompressionProof,
        use_zk: bool,
    ) -> anyhow::Result<SnarkWrapperProof> {
        let compression_vk = self.compression_vk()?.clone();

        let proof = self.backend.prove_snark(
            compression_proof,
            compression_vk,
            &self.config.trusted_setup,
            self.config.snark_vk.as_ref(),
            use_zk,
        )?;
        let start = Instant::now();
        if !self
            .verify_snark(&proof)
            .context("while attempting to verify generated SNARK proof")?
        {
            anyhow::bail!("SNARK wrapper proof verification failed");
        }
        tracing::info!(
            "Phase 3 verification took {:.3}s",
            start.elapsed().as_secs_f64()
        );

        Ok(proof)
    }

    /// Runs the full FRI-to-SNARK pipeline and returns the verified SNARK proof.
    pub fn prove_all(
        &mut self,
        program_proof: UnrolledProgramProof,
        use_zk: bool,
    ) -> anyhow::Result<SnarkWrapperProof> {
        let start = Instant::now();
        let risc_wrapper_proof = self.prove_risc_wrapper(program_proof)?;
        let compression_proof = self.prove_compression(risc_wrapper_proof)?;
        let snark_proof = self.prove_snark(compression_proof, use_zk)?;
        tracing::info!(
            "Full FRI-to-SNARK pipeline took {:.3}s",
            start.elapsed().as_secs_f64()
        );

        Ok(snark_proof)
    }

    /// Verifies an existing phase-3 proof against this wrapper's resolved SNARK VK.
    pub fn verify_snark(&mut self, proof: &SnarkWrapperProof) -> anyhow::Result<bool> {
        Ok(crate::verify_snark_wrapper_proof(proof, self.snark_vk()?))
    }

    pub fn snark_vk(&mut self) -> anyhow::Result<&SnarkWrapperVK> {
        if self.config.snark_vk.is_some() {
            return Ok(self
                .config
                .snark_vk
                .as_ref()
                .expect("phase 3 config VK exists"));
        }

        if self.backend.cached_snark_vk().is_some() {
            return Ok(self
                .backend
                .cached_snark_vk()
                .expect("phase 3 cache VK exists"));
        }

        let compression_vk = self.compression_vk()?.clone();

        self.backend
            .snark_vk(&self.config.trusted_setup, compression_vk)
    }

    fn binary_commitment(&mut self) -> anyhow::Result<BinaryCommitment> {
        if let Some(binary_commitment) = self.binary_commitment {
            return Ok(binary_commitment);
        }

        let start = Instant::now();
        let binary_commitment = load_binary_commitment(&self.config.bin, &self.config.text)?;
        tracing::info!(
            "Binary commitment loading took {:.3}s",
            start.elapsed().as_secs_f64()
        );
        self.binary_commitment = Some(binary_commitment);

        Ok(binary_commitment)
    }
}

fn create_boojum_worker(threads: Option<usize>) -> BoojumWorker {
    match threads {
        Some(num_threads) => BoojumWorker::new_with_num_threads(num_threads),
        None => BoojumWorker::new(),
    }
}

fn validate_trusted_setup_file(trusted_setup: &Option<PathBuf>) -> anyhow::Result<()> {
    let Some(path) = trusted_setup else {
        return Ok(());
    };

    let metadata = std::fs::metadata(path).with_context(|| {
        format!(
            "while attempting to read trusted setup metadata at {}",
            path.display()
        )
    })?;
    if !metadata.is_file() {
        anyhow::bail!(
            "Trusted setup path must point to a file: {}",
            path.display()
        );
    }

    Ok(())
}

fn load_binary_commitment(
    bin: &Option<PathBuf>,
    text: &Option<PathBuf>,
) -> anyhow::Result<BinaryCommitment> {
    match (bin, text) {
        (Some(bin_path), Some(text_path)) => {
            let binary = std::fs::read(bin_path).with_context(|| {
                format!(
                    "while attempting to read the binary at {}",
                    bin_path.display()
                )
            })?;
            let text_data = std::fs::read(text_path).with_context(|| {
                format!(
                    "while attempting to read the text section at {}",
                    text_path.display()
                )
            })?;

            Ok(BinaryCommitment::from_binary(&binary, &text_data))
        }
        (None, None) => Ok(BinaryCommitment::from_default_binary()),
        _ => anyhow::bail!("Both --bin and --text must be provided together"),
    }
}
