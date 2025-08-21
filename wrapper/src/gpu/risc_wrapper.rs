use std::alloc::Global;

use boojum::{
    algebraic_props::{round_function::AbsorptionModeOverwrite, sponge::GoldilocksPoseidon2Sponge},
    config::{ProvingCSConfig, SetupCSConfig},
    cs::{
        cs_builder::new_builder,
        cs_builder_reference::CsReferenceImplementationBuilder,
        implementations::{
            pow::NoPow, prover::ProofConfig, setup::FinalizationHintsForProver,
            transcript::GoldilocksPoisedon2Transcript,
        },
        traits::circuit::CircuitBuilder,
    },
    field::goldilocks::GoldilocksField,
    worker::Worker,
};
use shivini::{
    ProverContext, ProverContextConfig,
    cs::{GpuSetup, gpu_setup_and_vk_from_base_setup_vk_params_and_hints},
    gpu_proof_config::GpuProofConfig,
    gpu_prove_from_external_witness_data,
};

use crate::{
    BinaryCommitment, RiscWrapper, RiscWrapperProof, RiscWrapperTreeHasher, RiscWrapperVK,
    RiscWrapperWitness,
};

type GL = GoldilocksField;

pub fn get_risc_wrapper_setup(
    worker: &Worker,
    binary_commitment: BinaryCommitment,
) -> (
    GpuSetup<RiscWrapperTreeHasher>,
    RiscWrapperVK,
    FinalizationHintsForProver,
) {
    let start = std::time::Instant::now();

    let config = ProverContextConfig::default().with_smallest_supported_domain_size(1 << 15);
    let _prover_context = ProverContext::create_with_config(config).unwrap();

    let verify_inner_proof: bool = false;
    let circuit = RiscWrapper::new(None, verify_inner_proof, binary_commitment);

    let geometry = RiscWrapper::geometry();
    let (max_trace_len, num_vars) = circuit.size_hint();

    let builder_impl = CsReferenceImplementationBuilder::<GL, GL, SetupCSConfig>::new(
        geometry,
        max_trace_len.unwrap(),
    );
    let builder = new_builder::<_, GL>(builder_impl);

    let builder = RiscWrapper::configure_builder(builder);
    let mut cs = builder.build(num_vars.unwrap());
    circuit.add_tables(&mut cs);
    circuit.synthesize_into_cs(&mut cs);
    let (_, finalization_hint) = cs.pad_and_shrink();

    let ProofConfig {
        fri_lde_factor,
        merkle_tree_cap_size,
        ..
    } = RiscWrapper::get_proof_config();
    let cs = cs.into_assembly::<std::alloc::Global>();

    // TODO: for gpu we don't need full setup (light setup is enough)

    let (setup_base, vk_params, vars_hint, witness_hints) =
        cs.get_light_setup(worker, fri_lde_factor, merkle_tree_cap_size);

    println!("======== STARTING GPU ===========");

    /*let verifier_builder = RiscWrapperCircuitBuilder::dyn_verifier_builder();
    let verifier = verifier_builder.create_verifier();

    let gpu_proof_config = GpuProofConfig::from_verifier(&verifier);*/

    //let gpu_proof_config = GpuProofConfig::from_assembly(&cs);

    let (gpu_setup, gpu_vk) =
        gpu_setup_and_vk_from_base_setup_vk_params_and_hints::<RiscWrapperTreeHasher, _>(
            setup_base.clone(),
            vk_params,
            vars_hint.clone(),
            witness_hints.clone(),
            &worker,
        )
        .unwrap();

    println!(
        "risc wrapper setup takes {} ms",
        start.elapsed().as_millis()
    );

    (gpu_setup, gpu_vk, finalization_hint)
}

pub fn prove_risc_wrapper(
    risc_wrapper_witness: RiscWrapperWitness,
    finalization_hint: &FinalizationHintsForProver,
    gpu_setup: &GpuSetup<RiscWrapperTreeHasher>,
    gpu_vk: &RiscWrapperVK,
    worker: &Worker,
    binary_commitment: BinaryCommitment,
) -> RiscWrapperProof {
    let start = std::time::Instant::now();

    // TODO: this should be somehow done on higher level.
    let config = ProverContextConfig::default().with_smallest_supported_domain_size(1 << 15);
    let _prover_context = ProverContext::create_with_config(config).unwrap();

    let verify_inner_proof = true;
    let circuit = RiscWrapper::new(
        Some(risc_wrapper_witness),
        verify_inner_proof,
        binary_commitment,
    );

    let geometry = RiscWrapper::geometry();
    let (max_trace_len, num_vars) = circuit.size_hint();

    let builder_impl = CsReferenceImplementationBuilder::<GL, GL, ProvingCSConfig>::new(
        geometry,
        max_trace_len.unwrap(),
    );
    let builder = new_builder::<_, GL>(builder_impl);

    let builder = RiscWrapper::configure_builder(builder);
    let mut cs = builder.build(num_vars.unwrap());
    circuit.add_tables(&mut cs);
    circuit.synthesize_into_cs(&mut cs);
    cs.pad_and_shrink_using_hint(finalization_hint);
    let cs = cs.into_assembly::<std::alloc::Global>();

    let gpu_proof_config = GpuProofConfig::from_assembly(&cs);

    let external_witness_data = cs.witness.unwrap();

    // WitnessVectorGeneratorPayload is the input - it has circuit wrapper + finaliation hint.
    // to get this witness data, we do 'synthesize_vector'.
    // this calls circuit -> sytnehsis - > synthesis inner.
    // and this does stuff + into assembly.
    // we return WitnessVectorGeneratorExecutionOutput - circuit + tiwness vec.

    let proof_config = RiscWrapper::get_proof_config();

    type Transcript = GoldilocksPoisedon2Transcript;
    type Hasher = GoldilocksPoseidon2Sponge<AbsorptionModeOverwrite>;

    let proof = gpu_prove_from_external_witness_data::<Transcript, Hasher, NoPow, Global>(
        &gpu_proof_config,      // normally taken from 'verifier' - but I don't have any.
        &external_witness_data, // witness vector (I have it as struct, not bytes)
        proof_config,           // LDE factors and other stuff.
        &gpu_setup,
        &gpu_vk, // vk should be fine
        (),      // empty shoudl be ok
        worker,  // ok
    )
    .unwrap();

    println!(
        "risc wrapper proving takes {} ms",
        start.elapsed().as_millis()
    );

    proof.into()
}
