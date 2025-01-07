use boojum::field::SmallField;
use boojum::gadgets::u32::UInt32;
use boojum::cs::traits::cs::ConstraintSystem;
use boojum::gadgets::mersenne_field::fourth_ext::MersenneQuartic;
use boojum::gadgets::mersenne_field::MersenneField;
use boojum::gadgets::traits::allocatable::CSAllocatable;

use zkos_verifier::concrete::size_constants::*;
use zkos_verifier::prover::definitions::*;
use zkos_verifier::prover::cs::definitions::{
    NUM_DELEGATION_ARGUMENT_LINEARIZATION_CHALLENGES, REGISTER_SIZE,
    NUM_MEM_ARGUMENT_LINEARIZATION_CHALLENGES
};
use zkos_verifier::field::*;
use zkos_verifier::verifier_common::{ProofPublicInputs, ProofOutput};

mod prover_structs;
mod transcript;
mod blake2s_reduced;

use prover_structs::*;
use transcript::*;
use blake2s_reduced::*;

pub unsafe fn verify<
    F: SmallField,
    CS: ConstraintSystem<F>,
    // I: NonDeterminismSource,
    // V: LeafInclusionVerifier
>(
    proof_state_dst: &mut WrappedProofOutput<
        F, 
        TREE_CAP_SIZE,
        NUM_COSETS,
        NUM_DELEGATION_CHALLENGES,
        NUM_AUX_BOUNDARY_VALUES,
    >,
    proof_input_dst: &mut WrappedProofPublicInputs<F, NUM_STATE_ELEMENTS>,
) {
    todo!()
}

