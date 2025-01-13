use boojum::field::SmallField;
use boojum::gadgets::u32::UInt32;
use boojum::gadgets::blake2s::mixing_function::Word;
use boojum::gadgets::u8::UInt8;
use boojum::cs::traits::cs::ConstraintSystem;
use boojum::gadgets::mersenne_field::fourth_ext::MersenneQuartic;
use boojum::gadgets::mersenne_field::MersenneField;
use boojum::gadgets::boolean::Boolean;
use boojum::gadgets::u16::UInt16;
use boojum::gadgets::traits::allocatable::CSAllocatable;

use zkos_verifier::concrete::size_constants::*;
use zkos_verifier::prover::definitions::*;
use zkos_verifier::prover::cs::definitions::{
    NUM_DELEGATION_ARGUMENT_LINEARIZATION_CHALLENGES, REGISTER_SIZE,
    NUM_MEM_ARGUMENT_LINEARIZATION_CHALLENGES
};
use zkos_verifier::field::*;
use zkos_verifier::verifier_common::{ProofPublicInputs, ProofOutput};

// mod prover_structs;
pub mod transcript;
mod transcript_opt;
pub mod blake2s_reduced;

// use prover_structs::*;
pub use transcript::*;
use transcript_opt::*;
pub use blake2s_reduced::*;

// pub unsafe fn verify<
//     F: SmallField,
//     CS: ConstraintSystem<F>,
//     // I: NonDeterminismSource,
//     // V: LeafInclusionVerifier
// >(
//     proof_state_dst: &mut WrappedProofOutput<
//         F, 
//         TREE_CAP_SIZE,
//         NUM_COSETS,
//         NUM_DELEGATION_CHALLENGES,
//         NUM_AUX_BOUNDARY_VALUES,
//     >,
//     proof_input_dst: &mut WrappedProofPublicInputs<F, NUM_STATE_ELEMENTS>,
// ) {
//     todo!()
// }

