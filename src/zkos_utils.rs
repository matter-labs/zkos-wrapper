use crate::verify_zkos_proof;
use zkos_verifier::prover::prover_stages::Proof;

use zkos_verifier::concrete::size_constants::*;
use zkos_verifier::prover::definitions::Blake2sForEverythingVerifier;
use zkos_verifier::verifier_common::{ProofOutput, ProofPublicInputs};

pub fn read_and_verify_proof(
    proof_path: &String,
) -> (
    Proof,
    ProofOutput<TREE_CAP_SIZE, NUM_COSETS, NUM_DELEGATION_CHALLENGES, NUM_AUX_BOUNDARY_VALUES>,
    ProofPublicInputs<NUM_STATE_ELEMENTS>,
) {
    // read proof from file
    println!("Verifying proof from {}", proof_path);
    let proof: Proof = deserialize_from_file(proof_path);

    // verify proof
    let (proof_state_dst, proof_input_dst) =
        verify_zkos_proof::<Blake2sForEverythingVerifier>(&proof);

    (proof, proof_state_dst, proof_input_dst)
}

fn deserialize_from_file<T: serde::de::DeserializeOwned>(filename: &str) -> T {
    let src = std::fs::File::open(filename).unwrap();
    serde_json::from_reader(src).unwrap()
}
