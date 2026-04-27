use std::path::PathBuf;
use std::time::Instant;

use anyhow::Context as _;
use proof_compression::serialization::PlonkSnarkVerifierCircuitDeviceSetupWrapper;
use shivini::cs::GpuSetup;
use shivini::{ProverContext, ProverContextConfig};

use crate::circuits::{BinaryCommitment, RiscWrapperWitness};
use crate::{
    BoojumWorker, CompressionProof, CompressionTreeHasher, CompressionVK, RiscWrapperProof,
    RiscWrapperTreeHasher, RiscWrapperVK, SnarkWrapperProof, SnarkWrapperVK,
};

pub(crate) struct BackendState {
    risc_wrapper: Option<RiscWrapperSetupCache>,
    compression: Option<CompressionSetupCache>,
    snark: Option<SnarkSetupCache>,
    stark_context: Option<ProverContext>,
}

struct RiscWrapperSetupCache {
    finalization_hint: boojum::cs::implementations::setup::FinalizationHintsForProver,
    gpu_setup: GpuSetup<RiscWrapperTreeHasher>,
    vk: RiscWrapperVK,
}

struct CompressionSetupCache {
    finalization_hint: boojum::cs::implementations::setup::FinalizationHintsForProver,
    gpu_setup: GpuSetup<CompressionTreeHasher>,
    vk: CompressionVK,
}

struct SnarkSetupCache {
    crs: crate::gpu::snark::CompactRawCrs,
    setup: Option<SnarkGeneratedSetupCache>,
}

struct SnarkGeneratedSetupCache {
    precomputation: PlonkSnarkVerifierCircuitDeviceSetupWrapper,
    vk: SnarkWrapperVK,
}

impl BackendState {
    pub(crate) fn new() -> Self {
        Self {
            risc_wrapper: None,
            compression: None,
            snark: None,
            stark_context: None,
        }
    }

    pub(crate) fn cached_risc_wrapper_vk(&self) -> Option<&RiscWrapperVK> {
        self.risc_wrapper.as_ref().map(|cache| &cache.vk)
    }

    pub(crate) fn cached_compression_vk(&self) -> Option<&CompressionVK> {
        self.compression.as_ref().map(|cache| &cache.vk)
    }

    pub(crate) fn cached_snark_vk(&self) -> Option<&SnarkWrapperVK> {
        self.snark
            .as_ref()
            .and_then(|cache| cache.setup.as_ref())
            .map(|setup| &setup.vk)
    }

    pub(crate) fn prove_risc_wrapper(
        &mut self,
        witness: RiscWrapperWitness,
        binary_commitment: BinaryCommitment,
        worker: &BoojumWorker,
    ) -> anyhow::Result<RiscWrapperProof> {
        let start = Instant::now();
        self.ensure_stark_context()
            .context("while attempting to prepare the STARK GPU context")?;
        let cache = self.ensure_risc_wrapper_setup(binary_commitment, worker)?;
        tracing::info!(
            "Phase 1 GPU setup ready in {:.3}s",
            start.elapsed().as_secs_f64()
        );

        let start = Instant::now();
        let proof = crate::gpu::risc_wrapper::prove_risc_wrapper(
            witness,
            &cache.finalization_hint,
            &cache.gpu_setup,
            &cache.vk,
            worker,
            binary_commitment,
        );
        tracing::info!(
            "Phase 1 GPU proving took {:.3}s",
            start.elapsed().as_secs_f64()
        );

        Ok(proof)
    }

    pub(crate) fn risc_wrapper_vk(
        &mut self,
        binary_commitment: BinaryCommitment,
        worker: &BoojumWorker,
    ) -> anyhow::Result<&RiscWrapperVK> {
        let start = Instant::now();
        let cache = self.ensure_risc_wrapper_setup(binary_commitment, worker)?;
        tracing::info!(
            "Phase 1 GPU setup ready in {:.3}s",
            start.elapsed().as_secs_f64()
        );

        Ok(&cache.vk)
    }

    pub(crate) fn prove_compression(
        &mut self,
        risc_wrapper_proof: RiscWrapperProof,
        risc_wrapper_vk: RiscWrapperVK,
        worker: &BoojumWorker,
    ) -> anyhow::Result<CompressionProof> {
        let start = Instant::now();
        self.ensure_stark_context()
            .context("while attempting to prepare the STARK GPU context")?;
        let cache = self.ensure_compression_setup(risc_wrapper_vk.clone(), worker)?;
        tracing::info!(
            "Phase 2 GPU setup ready in {:.3}s",
            start.elapsed().as_secs_f64()
        );

        let start = Instant::now();
        let proof = crate::gpu::compression::prove_compression(
            risc_wrapper_proof,
            risc_wrapper_vk,
            &cache.finalization_hint,
            &cache.gpu_setup,
            &cache.vk,
            worker,
        );
        tracing::info!(
            "Phase 2 GPU proving took {:.3}s",
            start.elapsed().as_secs_f64()
        );

        Ok(proof)
    }

    pub(crate) fn compression_vk(
        &mut self,
        risc_wrapper_vk: RiscWrapperVK,
        worker: &BoojumWorker,
    ) -> anyhow::Result<&CompressionVK> {
        let start = Instant::now();
        let cache = self.ensure_compression_setup(risc_wrapper_vk, worker)?;
        tracing::info!(
            "Phase 2 GPU setup ready in {:.3}s",
            start.elapsed().as_secs_f64()
        );

        Ok(&cache.vk)
    }

    pub(crate) fn prove_snark(
        &mut self,
        compression_proof: CompressionProof,
        compression_vk: CompressionVK,
        trusted_setup: &Option<PathBuf>,
        provided_snark_vk: Option<&SnarkWrapperVK>,
        use_zk: bool,
    ) -> anyhow::Result<SnarkWrapperProof> {
        self.release_stark_context();

        let start = Instant::now();
        let mut device_manager = {
            let crs = self.ensure_snark_crs(trusted_setup)?;
            crate::gpu::snark::gpu_create_snark_device_manager(crs)
                .context("while attempting to initialize the phase 3 GPU device manager")?
        };
        let cache = self.ensure_snark_setup(
            compression_vk.clone(),
            provided_snark_vk,
            &mut device_manager,
        )?;
        tracing::info!(
            "Phase 3 GPU setup ready in {:.3}s",
            start.elapsed().as_secs_f64()
        );

        let start = Instant::now();
        let proof = crate::gpu::snark::gpu_snark_prove_with_manager(
            &cache.precomputation,
            &cache.vk,
            compression_proof,
            compression_vk,
            &mut device_manager,
            use_zk,
        )?;
        tracing::info!(
            "Phase 3 GPU proving took {:.3}s",
            start.elapsed().as_secs_f64()
        );

        Ok(proof)
    }

    pub(crate) fn snark_vk(
        &mut self,
        trusted_setup: &Option<PathBuf>,
        compression_vk: CompressionVK,
    ) -> anyhow::Result<&SnarkWrapperVK> {
        self.release_stark_context();
        let start = Instant::now();
        let cache = self.create_snark_setup(trusted_setup, compression_vk, None)?;
        tracing::info!(
            "Phase 3 GPU setup ready in {:.3}s",
            start.elapsed().as_secs_f64()
        );

        Ok(&cache.vk)
    }

    fn ensure_risc_wrapper_setup(
        &mut self,
        binary_commitment: BinaryCommitment,
        worker: &BoojumWorker,
    ) -> anyhow::Result<&RiscWrapperSetupCache> {
        if self.risc_wrapper.is_none() {
            self.ensure_stark_context()
                .context("while attempting to initialize the phase 1 GPU prover context")?;
            let (gpu_setup, vk, finalization_hint) =
                crate::gpu::risc_wrapper::get_risc_wrapper_setup(worker, binary_commitment);

            self.risc_wrapper = Some(RiscWrapperSetupCache {
                finalization_hint,
                gpu_setup,
                vk,
            });
        }

        Ok(self.risc_wrapper.as_ref().expect("phase 1 cache exists"))
    }

    fn ensure_compression_setup(
        &mut self,
        risc_wrapper_vk: RiscWrapperVK,
        worker: &BoojumWorker,
    ) -> anyhow::Result<&CompressionSetupCache> {
        if self.compression.is_none() {
            self.ensure_stark_context()
                .context("while attempting to initialize the phase 2 GPU prover context")?;
            let (gpu_setup, vk, finalization_hint) =
                crate::gpu::compression::get_compression_setup(worker, risc_wrapper_vk);

            self.compression = Some(CompressionSetupCache {
                finalization_hint,
                gpu_setup,
                vk,
            });
        }

        Ok(self.compression.as_ref().expect("phase 2 cache exists"))
    }

    fn ensure_snark_crs(
        &mut self,
        trusted_setup: &Option<PathBuf>,
    ) -> anyhow::Result<&crate::gpu::snark::CompactRawCrs> {
        if self.snark.is_none() {
            self.release_stark_context();
            let crs_file_path = trusted_setup
                .as_ref()
                .context("while attempting to prepare the phase 3 GPU setup")?;
            let crs_file = crs_file_path.to_string_lossy().to_string();
            let crs = crate::gpu::snark::gpu_load_compact_raw_crs(&crs_file)
                .context("while attempting to load the phase 3 compact CRS")?;

            self.snark = Some(SnarkSetupCache { crs, setup: None });
        }

        Ok(&self.snark.as_ref().expect("phase 3 cache exists").crs)
    }

    fn ensure_snark_setup(
        &mut self,
        compression_vk: CompressionVK,
        provided_snark_vk: Option<&SnarkWrapperVK>,
        device_manager: &mut crate::gpu::snark::PlonkDeviceManager,
    ) -> anyhow::Result<&SnarkGeneratedSetupCache> {
        let cache = self.snark.as_mut().expect("phase 3 CRS is loaded");
        if cache.setup.is_none() {
            let (precomputation, vk) = crate::gpu::snark::gpu_create_snark_setup_data_with_manager(
                &compression_vk,
                device_manager,
                provided_snark_vk,
            )
            .context("while attempting to create the phase 3 SNARK setup")?;

            cache.setup = Some(SnarkGeneratedSetupCache { precomputation, vk });
        }

        Ok(cache.setup.as_ref().expect("phase 3 setup exists"))
    }

    fn create_snark_setup(
        &mut self,
        trusted_setup: &Option<PathBuf>,
        compression_vk: CompressionVK,
        provided_snark_vk: Option<&SnarkWrapperVK>,
    ) -> anyhow::Result<&SnarkGeneratedSetupCache> {
        if self
            .snark
            .as_ref()
            .and_then(|cache| cache.setup.as_ref())
            .is_none()
        {
            let mut device_manager = {
                let crs = self.ensure_snark_crs(trusted_setup)?;
                crate::gpu::snark::gpu_create_snark_device_manager(crs)
                    .context("while attempting to initialize the phase 3 GPU device manager")?
            };

            self.ensure_snark_setup(compression_vk, provided_snark_vk, &mut device_manager)?;
        }

        Ok(self
            .snark
            .as_ref()
            .and_then(|cache| cache.setup.as_ref())
            .expect("phase 3 setup exists"))
    }

    fn ensure_stark_context(&mut self) -> anyhow::Result<()> {
        if self.stark_context.is_none() {
            // Both STARK phases use shivini's process-global context. Use the
            // smallest domain required by phase 2 so one context can serve phase
            // 1 and phase 2 for a single FRI -> SNARK pipeline iteration.
            let config =
                ProverContextConfig::default().with_smallest_supported_domain_size(1 << 15);
            self.stark_context = Some(
                ProverContext::create_with_config(config)
                    .context("while attempting to initialize the STARK GPU prover context")?,
            );
        }

        Ok(())
    }

    fn release_stark_context(&mut self) {
        self.stark_context = None;
    }
}
