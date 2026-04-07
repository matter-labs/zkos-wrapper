use std::mem::MaybeUninit;

use boojum::cs::traits::cs::ConstraintSystem;
use boojum::field::SmallField;
use boojum::gadgets::boolean::Boolean;
use boojum::gadgets::traits::allocatable::CSAllocatable;
use boojum::gadgets::traits::selectable::Selectable;
use boojum::gadgets::u32::UInt32;
use circuit_mersenne_field::extension_trait::CircuitFieldExpression;
use circuit_mersenne_field::{MersenneComplex, MersenneField, MersenneQuartic};

use crate::risc_verifier;
use risc_verifier::blake2s_u32::*;
use risc_verifier::concrete::size_constants::*;
// use risc_verifier::concrete::skeleton_instance::BASE_CIRCUIT_QUERY_VALUES_NO_PADDING_U32_WORDS;
use risc_verifier::concrete::skeleton_instance::ProofSkeletonInstance;
use risc_verifier::concrete::skeleton_instance::QueryValuesInstance;
use risc_verifier::field::*;
use risc_verifier::prover::cs::definitions::*;
use risc_verifier::skeleton::{ProofSkeleton, QueryValues};
use risc_verifier::verifier_common;
use risc_verifier::verifier_common::non_determinism_source::NonDeterminismSource;
use risc_verifier::verifier_common::{SizedProofPowChallenges, transcript_challenge_array_size};

use crate::wrapper_utils::prover_structs::*;

pub(crate) mod imports;
pub mod skeleton;

pub use crate::transcript::*;
use crate::wrapper_utils::verifier_traits::*;
use skeleton::*;

include!("../shared/verify_impl.rs");
