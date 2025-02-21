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
