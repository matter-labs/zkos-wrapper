use std::mem::MaybeUninit;

use boojum::cs::traits::cs::ConstraintSystem;
use boojum::field::SmallField;
use boojum::gadgets::boolean::Boolean;
use boojum::gadgets::traits::allocatable::CSAllocatable;
use boojum::gadgets::traits::selectable::Selectable;
use boojum::gadgets::u32::UInt32;
use circuit_mersenne_field::extension_trait::CircuitFieldExpression;
use circuit_mersenne_field::{MersenneComplex, MersenneField, MersenneQuartic};

use blake_verifier::blake2s_u32::*;
use blake_verifier::concrete::size_constants::*;
// use blake_verifier::concrete::skeleton_instance::BASE_CIRCUIT_QUERY_VALUES_NO_PADDING_U32_WORDS;
use blake_verifier::concrete::skeleton_instance::ProofSkeletonInstance;
use blake_verifier::concrete::skeleton_instance::QueryValuesInstance;
use blake_verifier::field::*;
use blake_verifier::prover::cs::definitions::*;
use blake_verifier::skeleton::{ProofSkeleton, QueryValues};
use blake_verifier::verifier_common::non_determinism_source::NonDeterminismSource;
use risc_verifier::verifier_common::{SizedProofPowChallenges, transcript_challenge_array_size};

use crate::wrapper_utils::prover_structs::*;

pub(crate) mod imports;
pub mod skeleton;

pub use crate::transcript::*;
use crate::wrapper_utils::verifier_traits::*;
use skeleton::*;

use crate::risc_verifier;
use crate::set_iterator_from_proof;
use risc_verifier::prover::nd_source_std::ThreadLocalBasedSource;
use risc_verifier::prover::prover_stages::Proof as RiscProof;
pub fn prepare_blake_proof_for_wrapper<
    F: SmallField,
    CS: ConstraintSystem<F>,
    V: CircuitLeafInclusionVerifier<F>,
>(
    cs: &mut CS,
    proof: &RiscProof,
) -> (
    WrappedProofSkeletonInstance<F>,
    [WrappedQueryValuesInstance<F>; NUM_QUERIES],
) {
    let shuffle_ram_inits_and_teardowns_len = imports::VERIFIER_COMPILED_LAYOUT
        .memory_layout
        .shuffle_ram_inits_and_teardowns
        .len();
    set_iterator_from_proof(proof, shuffle_ram_inits_and_teardowns_len);

    let skeleton = unsafe {
        WrappedProofSkeletonInstance::from_non_determinism_source::<_, ThreadLocalBasedSource>(cs)
    };

    let mut leaf_inclusion_verifier = V::new(cs);

    let queries: [_; NUM_QUERIES] = std::array::from_fn(|_idx| unsafe {
        WrappedQueryValuesInstance::from_non_determinism_source::<_, ThreadLocalBasedSource, _>(
            cs,
            &skeleton,
            &mut leaf_inclusion_verifier,
        )
    });

    (skeleton, queries)
}

pub fn placeholders<F: SmallField, CS: ConstraintSystem<F>, V: CircuitLeafInclusionVerifier<F>>(
    cs: &mut CS,
) -> (
    WrappedProofSkeletonInstance<F>,
    [WrappedQueryValuesInstance<F>; NUM_QUERIES],
) {
    // allocate from placeholder
    let skeleton_witness = WrappedProofSkeletonInstance::<F>::placeholder_witness();
    let skeleton = WrappedProofSkeletonInstance::allocate(cs, skeleton_witness);

    let mut leaf_inclusion_verifier = V::new(cs);

    let queries: [_; NUM_QUERIES] = std::array::from_fn(|_idx| unsafe {
        WrappedQueryValuesInstance::from_non_determinism_source::<_, PlaceholderSource, _>(
            cs,
            &skeleton,
            &mut leaf_inclusion_verifier,
        )
    });

    (skeleton, queries)
}

use crate::inner_verifiers::blake_delegation::prover::definitions::LeafInclusionVerifier;
use crate::inner_verifiers::blake_delegation::verifier_common::ProofOutput;
use blake_verifier::*;

#[allow(invalid_value)]
pub(crate) fn verify_blake_proof<V: LeafInclusionVerifier>(
    proof: &RiscProof,
) -> (
    ProofOutput<TREE_CAP_SIZE, NUM_COSETS, NUM_DELEGATION_CHALLENGES, NUM_AUX_BOUNDARY_VALUES>,
    ProofPublicInputs<NUM_STATE_ELEMENTS>,
) {
    let shuffle_ram_inits_and_teardowns_len = imports::VERIFIER_COMPILED_LAYOUT
        .memory_layout
        .shuffle_ram_inits_and_teardowns
        .len();
    set_iterator_from_proof(proof, shuffle_ram_inits_and_teardowns_len);

    let mut proof_state_dst = unsafe {
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
    let mut proof_input_dst =
        unsafe { MaybeUninit::<ProofPublicInputs<NUM_STATE_ELEMENTS>>::uninit().assume_init() };

    unsafe {
        blake_verifier::verify_with_configuration::<ThreadLocalBasedSource, V>(
            &mut proof_state_dst,
            &mut proof_input_dst,
        );
    }

    (proof_state_dst, proof_input_dst)
}

pub type WrappedBlakeProofOutput<F> = WrappedProofOutput<
    F,
    TREE_CAP_SIZE,
    NUM_COSETS,
    NUM_DELEGATION_CHALLENGES,
    NUM_AUX_BOUNDARY_VALUES,
>;

include!("../shared/verify_impl.rs");
