#![feature(allocator_api)]
#![feature(array_chunks)]
#![feature(generic_arg_infer)]

use std::mem::MaybeUninit;

use boojum::cs::gates::ConstantAllocatableCS;
use boojum::cs::gates::PublicInputGate;
use boojum::field::goldilocks::GoldilocksField;
use boojum::{
    cs::traits::cs::ConstraintSystem, field::SmallField,
    gadgets::traits::allocatable::CSAllocatable,
};
use serde::Deserialize;
use verifier::prover_structs::{
    ProofSkeletonInstance, WrappedProofOutput, WrappedProofPublicInputs, WrappedProofSkeleton,
    WrappedProofSkeletonInstance, WrappedQueryValues, WrappedQueryValuesInstance,
};
use zkos_verifier::ProofPublicInputs;
use zkos_verifier::{
    concrete::skeleton_instance::QueryValuesInstance, verifier_common::ProofOutput,
};
pub mod verifier;

#[derive(Clone, Debug, serde::Serialize, serde::Deserialize)]
pub struct ZKOSWrapperInstanceWitness {
    #[serde(skip)]
    pub skeleton: Option<ProofSkeletonInstance>,
    #[serde(skip)]
    pub queries: Option<[QueryValuesInstance; 53]>, // NUM_QUERIES
}

use zkos_verifier::concrete::size_constants::{
    NUM_AUX_BOUNDARY_VALUES, NUM_COSETS, NUM_DELEGATION_CHALLENGES, NUM_OPENINGS_AT_Z, NUM_QUERIES,
    NUM_STATE_ELEMENTS, TREE_CAP_SIZE,
};

pub fn zkoswrapper_function<F: SmallField, CS: ConstraintSystem<F>>(
    cs: &mut CS,
    mut witness: ZKOSWrapperInstanceWitness,
) {
    let skeleton = witness.skeleton.unwrap();
    let queries = witness.queries.unwrap();

    let skeleton = WrappedProofSkeleton::allocate(cs, skeleton);
    let queries = queries.map(|query| WrappedQueryValues::allocate(cs, query));

    // allocate empty
    let proof_state_dst = unsafe {
        MaybeUninit::<
            ProofOutput<
                TREE_CAP_SIZE,
                NUM_COSETS,
                NUM_DELEGATION_CHALLENGES,
                NUM_AUX_BOUNDARY_VALUES,
            >,
        >::uninit()
        .assume_init()
    };
    let proof_input_dst =
        unsafe { MaybeUninit::<ProofPublicInputs<NUM_STATE_ELEMENTS>>::uninit().assume_init() };

    let mut proof_state_dst: WrappedProofOutput<F, _, 2, _, 1> =
        WrappedProofOutput::allocate(cs, proof_state_dst);
    let mut proof_input_dst = WrappedProofPublicInputs::allocate(cs, proof_input_dst);
    crate::verifier::verify(
        cs,
        &mut proof_state_dst,
        &mut proof_input_dst,
        skeleton,
        queries,
    );
    for i in 1u64..5u64 {
        let aa = cs.allocate_constant(F::ONE);

        let gate = PublicInputGate::new(aa);
        gate.add_to_cs(cs);
    }
}

#[cfg(test)]
mod tests;

use boojum::algebraic_props::round_function::AbsorptionModeOverwrite;
use boojum::algebraic_props::sponge::GoldilocksPoseidon2Sponge;
use boojum::config::{ProvingCSConfig, SetupCSConfig};
use boojum::cs::cs_builder::new_builder;
use boojum::cs::cs_builder_reference::CsReferenceImplementationBuilder;
use boojum::cs::implementations::hints::DenseVariablesCopyHint;
use boojum::cs::implementations::hints::DenseWitnessCopyHint;
use boojum::cs::implementations::polynomial_storage::SetupBaseStorage;
use boojum::cs::implementations::polynomial_storage::SetupStorage;
use boojum::cs::implementations::pow::NoPow;
use boojum::cs::implementations::proof::Proof;
use boojum::cs::implementations::prover::ProofConfig;
use boojum::cs::implementations::setup::FinalizationHintsForProver;
use boojum::cs::implementations::verifier::VerificationKey;
use boojum::cs::oracle::merkle_tree::MerkleTreeWithCap;
use boojum::cs::traits::circuit::CircuitBuilder;
use boojum::cs::traits::circuit::CircuitBuilderProxy;
use boojum::worker::*;
use verifier::verifier_traits::CircuitBlake2sForEverythingVerifier;
use verifier_circuit::*;
use zkos_verifier::prover::prover_stages::Proof as ZkosProof;

type F = boojum::field::goldilocks::GoldilocksField;
type EXT = boojum::field::goldilocks::GoldilocksExt2;
type V = CircuitBlake2sForEverythingVerifier<F>;
type C = ZKOSWrapperCircuit<F, V>;
type TR = boojum::cs::implementations::transcript::GoldilocksPoisedon2Transcript;
type H = GoldilocksPoseidon2Sponge<AbsorptionModeOverwrite>;

type CB = CircuitBuilderProxy<F, C>;

pub fn get_zkos_wrapper_setup(
    worker: &Worker,
) -> (
    FinalizationHintsForProver,
    SetupBaseStorage<F>,
    SetupStorage<F>,
    VerificationKey<F, H>,
    MerkleTreeWithCap<F, H>,
    DenseVariablesCopyHint,
    DenseWitnessCopyHint,
) {
    let verify_inner_proof: bool = false;
    let circuit = C::new(None, verify_inner_proof);

    let geometry = C::geometry();
    let (max_trace_len, num_vars) = circuit.size_hint();

    let builder_impl = CsReferenceImplementationBuilder::<F, F, SetupCSConfig>::new(
        geometry,
        max_trace_len.unwrap(),
    );
    let builder = new_builder::<_, F>(builder_impl);

    let builder = C::configure_builder(builder);
    let mut cs = builder.build(num_vars.unwrap());
    circuit.add_tables(&mut cs);
    circuit.synthesize_into_cs(&mut cs);
    let (_, finalization_hint) = cs.pad_and_shrink();

    let ProofConfig {
        fri_lde_factor,
        merkle_tree_cap_size,
        ..
    } = C::get_proof_config();
    let cs = cs.into_assembly::<std::alloc::Global>();

    let (setup_base, setup, vk, setup_tree, vars_hint, witness_hints) =
        cs.get_full_setup::<H>(worker, fri_lde_factor, merkle_tree_cap_size);

    (
        finalization_hint,
        setup_base,
        setup,
        vk,
        setup_tree,
        vars_hint,
        witness_hints,
    )
}

pub fn get_zkos_wrapper_proof(
    zkos_proof: ZkosProof,
    finalization_hint: &FinalizationHintsForProver,
    setup_base: &SetupBaseStorage<F>,
    setup: &SetupStorage<F>,
    vk: &VerificationKey<F, H>,
    setup_tree: &MerkleTreeWithCap<F, H>,
    vars_hint: &DenseVariablesCopyHint,
    witness_hints: &DenseWitnessCopyHint,
    worker: &Worker,
) -> Proof<F, H, EXT> {
    let verify_inner_proof = true;
    let circuit = C::new(Some(zkos_proof), verify_inner_proof);

    let geometry = C::geometry();
    let (max_trace_len, num_vars) = circuit.size_hint();

    let builder_impl = CsReferenceImplementationBuilder::<F, F, ProvingCSConfig>::new(
        geometry,
        max_trace_len.unwrap(),
    );
    let builder = new_builder::<_, F>(builder_impl);

    let builder = C::configure_builder(builder);
    let mut cs = builder.build(num_vars.unwrap());
    circuit.add_tables(&mut cs);
    circuit.synthesize_into_cs(&mut cs);
    cs.pad_and_shrink_using_hint(finalization_hint);
    let cs = cs.into_assembly::<std::alloc::Global>();

    let proof_config = C::get_proof_config();

    cs.prove_from_precomputations::<EXT, TR, H, NoPow>(
        proof_config,
        &setup_base,
        &setup,
        &setup_tree,
        &vk,
        &vars_hint,
        &witness_hints,
        (),
        worker,
    )
}

pub fn verify_zkos_wrapper_proof(proof: &Proof<F, H, EXT>, vk: &VerificationKey<F, H>) -> bool {
    let verifier_builder = CB::dyn_verifier_builder();
    let verifier = verifier_builder.create_verifier();
    verifier.verify::<H, TR, NoPow>((), vk, proof)
}
