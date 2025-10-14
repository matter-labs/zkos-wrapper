use proof_compression::{
    PlonkSnarkWrapper, ProofSystemDefinition, SnarkWrapperProofSystem,
    hardcoded_canonical_g2_bases,
    serialization::{GenericWrapper, PlonkSnarkVerifierCircuitDeviceSetupWrapper},
};
use zksync_gpu_prover::{
    AsyncSetup,
    bellman::{
        bn256::Bn256,
        kate_commitment::{Crs, CrsForMonomialForm},
        plonk::better_better_cs::{
            cs::{
                Assembly, Circuit, PlonkCsWidth4WithNextStepAndCustomGatesParams,
                SynthesisModeGenerateSetup, SynthesisModeProve,
            },
            gates::selector_optimized_with_d_next::SelectorOptimizedWidth4MainGateWithDNext,
        },
    },
};

use crate::{
    CompressionProof, CompressionVK, L1_VERIFIER_DOMAIN_SIZE_LOG, SnarkWrapperCircuit,
    SnarkWrapperFunction, SnarkWrapperProof, SnarkWrapperVK,
};

// The code below is based off the zkos-compressor code.
// Unfortunately we were not able to use zkos-compressor directly (for example PlonkSnarkWrapper) - as the Compression circuit here is different.

/// Creates setup data (precomputations and verification key) for a given circuit.
/// crs_file must point at **compact** CRS.
pub fn gpu_create_snark_setup_data(
    compression_vk: &CompressionVK,
    crs_file: &str,
) -> (PlonkSnarkVerifierCircuitDeviceSetupWrapper, SnarkWrapperVK) {
    let reader = std::fs::File::open(crs_file).unwrap();

    let crs_mons =
        <PlonkSnarkWrapper as SnarkWrapperProofSystem>::load_compact_raw_crs(reader).unwrap();

    type PlonkAssembly<CSConfig> = Assembly<
        Bn256,
        PlonkCsWidth4WithNextStepAndCustomGatesParams,
        SelectorOptimizedWidth4MainGateWithDNext,
        CSConfig,
        zksync_gpu_prover::cuda_bindings::CudaAllocator,
    >;

    // reimplementing stuff in precompute_plonk_wrapper_circuit (as we have different compression circuit).
    let fixed_parameters = compression_vk.fixed_parameters.clone();
    let wrapper_function = SnarkWrapperFunction;
    let wrapper_circuit = SnarkWrapperCircuit {
        witness: None,
        vk: compression_vk.clone(),
        fixed_parameters,
        transcript_params: (),
        wrapper_function,
    };

    let mut setup_assembly = PlonkAssembly::<SynthesisModeGenerateSetup>::new();

    wrapper_circuit
        .synthesize(&mut setup_assembly)
        .expect("must work");

    let hardcoded_finalization_hint = L1_VERIFIER_DOMAIN_SIZE_LOG;

    // It used finalization hint instead (fine - it is 24 for plonk, and 23 for fflonk).
    setup_assembly.finalize_to_size_log_2(hardcoded_finalization_hint);
    assert!(setup_assembly.is_satisfied());

    // now gpu part.
    let mut ctx = PlonkSnarkWrapper::init_context(&crs_mons)
        .unwrap()
        .into_inner();

    let worker = zksync_gpu_prover::bellman::worker::Worker::new();
    let mut precomputation = zksync_gpu_prover::AsyncSetup::<
        <PlonkSnarkWrapper as ProofSystemDefinition>::Allocator,
    >::allocate(1 << hardcoded_finalization_hint);
    precomputation
        .generate_from_assembly(&worker, &setup_assembly, &mut ctx)
        .unwrap();

    let hardcoded_g2_bases = hardcoded_canonical_g2_bases();
    let mut dummy_crs = Crs::<bellman::bn256::Bn256, CrsForMonomialForm>::dummy_crs(1);
    dummy_crs.g2_monomial_bases = std::sync::Arc::new(hardcoded_g2_bases.to_vec());
    let vk = zksync_gpu_prover::compute_vk_from_assembly::<
        _,
        _,
        PlonkCsWidth4WithNextStepAndCustomGatesParams,
        SynthesisModeGenerateSetup,
    >(&mut ctx, &setup_assembly, &dummy_crs)
    .unwrap();

    ctx.free_all_slots();

    (
        PlonkSnarkVerifierCircuitDeviceSetupWrapper::from_inner(precomputation),
        vk,
    )
}

/// Computes the SnarkProof for a given compression proof.
pub fn gpu_snark_prove(
    precomputation: &PlonkSnarkVerifierCircuitDeviceSetupWrapper,
    snark_wrapper_vk: &SnarkWrapperVK,
    compression_proof: CompressionProof,
    compression_vk: CompressionVK,
    crs_file: &str,
) -> SnarkWrapperProof {
    let reader = std::fs::File::open(crs_file).unwrap();
    let finalization_hint: usize = 1 << 24;

    let crs_mons =
        <PlonkSnarkWrapper as SnarkWrapperProofSystem>::load_compact_raw_crs(reader).unwrap();

    let input_proof = compression_proof;
    // Recreate stuff from prove_plonk_snark_wrapper_step

    let input_vk = compression_vk.clone();
    let mut ctx = PlonkSnarkWrapper::init_context(&crs_mons)
        .unwrap()
        .into_inner();
    let fixed_parameters = input_vk.fixed_parameters.clone();

    let wrapper_function = SnarkWrapperFunction;
    let circuit = SnarkWrapperCircuit {
        witness: Some(input_proof),
        vk: compression_vk.clone(),
        fixed_parameters,
        transcript_params: (),
        wrapper_function,
    };
    type PlonkAssembly<CSConfig> = Assembly<
        Bn256,
        PlonkCsWidth4WithNextStepAndCustomGatesParams,
        SelectorOptimizedWidth4MainGateWithDNext,
        CSConfig,
        zksync_gpu_prover::cuda_bindings::CudaAllocator,
    >;

    let mut proving_assembly = PlonkAssembly::<SynthesisModeProve>::new();

    circuit
        .synthesize(&mut proving_assembly)
        .expect("must work");

    let precomputation: &AsyncSetup = precomputation.into_inner_ref();

    assert!(proving_assembly.is_satisfied());
    assert!(finalization_hint.is_power_of_two());
    proving_assembly.finalize_to_size_log_2(finalization_hint.trailing_zeros() as usize);
    let domain_size = proving_assembly.n() + 1;
    assert!(domain_size.is_power_of_two());
    assert!(domain_size == finalization_hint.clone());

    let worker = zksync_gpu_prover::bellman::worker::Worker::new();
    let start = std::time::Instant::now();
    let proof = zksync_gpu_prover::create_proof::<
        _,
        _,
        <PlonkSnarkWrapper as ProofSystemDefinition>::Transcript,
        _,
    >(&proving_assembly, &mut ctx, &worker, precomputation, None)
    .unwrap();

    println!("plonk proving takes {} s", start.elapsed().as_secs());
    ctx.free_all_slots();

    let result = zksync_gpu_prover::bellman::plonk::better_better_cs::verifier::verify::<
        _,
        _,
        <PlonkSnarkWrapper as ProofSystemDefinition>::Transcript,
    >(snark_wrapper_vk, &proof, None)
    .unwrap();

    if !result {
        panic!("*** WARNING - SNARK FAILED TO VERIFY ****");
    }
    proof
}
