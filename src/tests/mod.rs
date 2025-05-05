use crate::transcript::*;
use crate::wrapper_inner_verifier::skeleton::*;
use crate::wrapper_utils::verifier_traits::CircuitBlake2sForEverythingVerifier;
use crate::wrapper_utils::verifier_traits::CircuitLeafInclusionVerifier;
use boojum::cs::gates::FmaGateInBaseFieldWithoutConstant;
use boojum::cs::gates::NopGate;
use boojum::cs::gates::SelectionGate;
use boojum::cs::gates::ZeroCheckGate;
use boojum::cs::LookupParameters;
use boojum::gadgets::tables::create_range_check_15_bits_table;
use boojum::gadgets::tables::create_range_check_16_bits_table;
use boojum::gadgets::tables::RangeCheck15BitsTable;
use boojum::gadgets::tables::RangeCheck16BitsTable;
use boojum::{
    cs::{
        gates::{ConstantsAllocatorGate, ReductionGate, U32TriAddCarryAsChunkGate, UIntXAddGate},
        traits::{cs::ConstraintSystem, gate::GatePlacementStrategy},
        CSGeometry,
    },
    dag::CircuitResolverOpts,
    gadgets::blake2s::mixing_function::Word,
    gadgets::{
        tables::{
            byte_split::{create_byte_split_table, ByteSplitTable},
            xor8::{create_xor8_table, Xor8Table},
        },
        traits::witnessable::WitnessHookable,
        u32::UInt32,
    },
};
use risc_verifier::prover::prover_stages::Proof as RiscProof;
use std::alloc::Global;

use risc_verifier::concrete::size_constants::*;
use risc_verifier::prover::definitions::Blake2sForEverythingVerifier;
use risc_verifier::verifier_common::{DefaultNonDeterminismSource, ProofOutput, ProofPublicInputs};

type F = boojum::field::goldilocks::GoldilocksField;

mod blake2s_tests;
mod compression_tests;
mod risc_wrapper_tests;
mod snark_wrapper_tests;

#[test]
fn all_layers_full_test() {
    println!("Testing Risc wrapper");
    risc_wrapper_tests::risc_wrapper_full_test();
    println!("Testing compression");
    compression_tests::compression_full_test();
    println!("Testing Snark wrapper");
    snark_wrapper_tests::snark_wrapper_full_test();
}

#[test]
fn all_layers_setup_test() {
    risc_wrapper_tests::risc_wrapper_setup_test();
    compression_tests::compression_setup_test();
    snark_wrapper_tests::snark_wrapper_setup_test();
}

#[test]
fn get_setup_caps() {
    let risc_proof: RiscProof = deserialize_from_file(RISC_PROOF_PATH);

    serialize_to_file(&risc_proof.setup_tree_caps, "testing_data/setup_caps");
}

const RISC_REGISTER_FINAL_STATE_PATH: &str = "testing_data/register_final_values";
const RISC_PROOF_PATH: &str = "testing_data/risc_proof";
const RISC_WRAPPER_PROOF_PATH: &str = "testing_data/risc_wrapper_proof";
const RISC_WRAPPER_VK_PATH: &str = "testing_data/risc_wrapper_vk";
const COMPRESSION_PROOF_PATH: &str = "testing_data/compression_proof";
const COMPRESSION_VK_PATH: &str = "testing_data/compression_vk";
const SNARK_WRAPPER_PROOF_PATH: &str = "testing_data/snark_wrapper_proof";
const SNARK_WRAPPER_VK_PATH: &str = "testing_data/snark_wrapper_vk";

fn deserialize_from_file<T: serde::de::DeserializeOwned>(filename: &str) -> T {
    let src = std::fs::File::open(filename).unwrap();
    serde_json::from_reader(src).unwrap()
}

fn serialize_to_file<T: serde::ser::Serialize>(content: &T, filename: &str) {
    let src = std::fs::File::create(filename).unwrap();
    serde_json::to_writer_pretty(src, content).unwrap();
}
