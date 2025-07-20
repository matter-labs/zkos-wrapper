use crate::transcript::*;
use crate::wrapper_inner_verifier::skeleton::*;
use crate::wrapper_utils::verifier_traits::CircuitBlake2sForEverythingVerifier;
use crate::wrapper_utils::verifier_traits::CircuitLeafInclusionVerifier;
use boojum::cs::LookupParameters;
use boojum::cs::gates::FmaGateInBaseFieldWithoutConstant;
use boojum::cs::gates::NopGate;
use boojum::cs::gates::SelectionGate;
use boojum::cs::gates::ZeroCheckGate;
use boojum::gadgets::tables::RangeCheck15BitsTable;
use boojum::gadgets::tables::RangeCheck16BitsTable;
use boojum::gadgets::tables::create_range_check_15_bits_table;
use boojum::gadgets::tables::create_range_check_16_bits_table;
use boojum::{
    cs::{
        CSGeometry,
        gates::{ConstantsAllocatorGate, ReductionGate, U32TriAddCarryAsChunkGate, UIntXAddGate},
        traits::{cs::ConstraintSystem, gate::GatePlacementStrategy},
    },
    dag::CircuitResolverOpts,
    gadgets::blake2s::mixing_function::Word,
    gadgets::{
        tables::{
            byte_split::{ByteSplitTable, create_byte_split_table},
            xor8::{Xor8Table, create_xor8_table},
        },
        traits::witnessable::WitnessHookable,
        u32::UInt32,
    },
};
use crate::risc_verifier;
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

// To regenerate this test data (which must happen every time something changes in the verifier)
// please run (from the zksync-airbender - and make sure to match the current dependency):
// Make sure that you have a machine with >140GB of RAM for this step:
// cargo run -p cli --release prove --bin examples/hashed_fibonacci/app.bin --input-file examples/hashed_fibonacci/input.txt --output-dir /tmp/ --until final-proof
// And then copy /tmp/final_program_proof testing_data/risc_final_machine_proof

#[cfg(feature = "final_machine")]
mod path_constants {
    pub(crate) const RISC_PROOF_PATH: &str = "testing_data/risc_final_machine_proof";
    pub(crate) const RISC_WRAPPER_PROOF_PATH: &str = "testing_data/final_machine_mode_risc_wrapper_proof";
    pub(crate) const RISC_WRAPPER_VK_PATH: &str = "testing_data/final_machine_mode_risc_wrapper_vk";
    pub(crate) const COMPRESSION_PROOF_PATH: &str = "testing_data/final_machine_mode_compression_proof";
    pub(crate) const COMPRESSION_VK_PATH: &str = "testing_data/final_machine_mode_compression_vk";
    pub(crate) const SNARK_WRAPPER_PROOF_PATH: &str = "testing_data/final_machine_mode_snark_wrapper_proof";
    pub(crate) const SNARK_WRAPPER_VK_PATH: &str = "testing_data/final_machine_mode_snark_wrapper_vk";
}

#[cfg(not(feature = "final_machine"))]
mod path_constants {
    pub(crate) const RISC_PROOF_PATH: &str = "testing_data/risc_proof";
    pub(crate) const RISC_WRAPPER_PROOF_PATH: &str = "testing_data/risc_wrapper_proof";
    pub(crate) const RISC_WRAPPER_VK_PATH: &str = "testing_data/risc_wrapper_vk";
    pub(crate) const COMPRESSION_PROOF_PATH: &str = "testing_data/compression_proof";
    pub(crate) const COMPRESSION_VK_PATH: &str = "testing_data/compression_vk";
    pub(crate) const SNARK_WRAPPER_PROOF_PATH: &str = "testing_data/snark_wrapper_proof";
    pub(crate) const SNARK_WRAPPER_VK_PATH: &str = "testing_data/snark_wrapper_vk";
}

use path_constants::*;

fn deserialize_from_file<T: serde::de::DeserializeOwned>(filename: &str) -> T {
    let src = std::fs::File::open(filename).unwrap();
    serde_json::from_reader(src).unwrap()
}

fn serialize_to_file<T: serde::ser::Serialize>(content: &T, filename: &str) {
    let src = std::fs::File::create(filename).unwrap();
    serde_json::to_writer_pretty(src, content).unwrap();
}
