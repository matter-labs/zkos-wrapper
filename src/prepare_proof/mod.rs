use std::mem::MaybeUninit;
use zkos_verifier::prover::prover_stages::Proof;
use zkos_verifier::prover::definitions::Blake2sForEverythingVerifier;
use zkos_verifier::concrete::size_constants::*;
use zkos_verifier::verify_with_configuration;
use zkos_verifier::verifier_common::{DefaultNonDeterminismSource, ProofPublicInputs, ProofOutput};

pub(crate) fn verify_proof_and_set_iterator(proof_path: &String) {
    // read proof from file
    println!("Verifying proof from {}", proof_path);
    let proof: Proof = deserialize_from_file(proof_path);

    // set iterator for verification
    let mut oracle_data = vec![];

    let shuffle_ram_inits_and_teardowns = true;

    oracle_data.extend(
        zkos_verifier::verifier_common::proof_flattener::flatten_proof_for_skeleton(
            &proof,
            shuffle_ram_inits_and_teardowns,
        ),
    );
    for query in proof.queries.iter() {
        oracle_data.extend(zkos_verifier::verifier_common::proof_flattener::flatten_query(query));
    }

    let it = oracle_data.into_iter();

    zkos_verifier::prover::nd_source_std::set_iterator(it.clone());

    // verify proof

    let mut proof_state_dst = unsafe {MaybeUninit::<ProofOutput<
        TREE_CAP_SIZE,
        NUM_COSETS,
        NUM_DELEGATION_CHALLENGES,
        NUM_AUX_BOUNDARY_VALUES,
    >>::uninit().assume_init()};
    let mut proof_input_dst = unsafe {MaybeUninit::<ProofPublicInputs<NUM_STATE_ELEMENTS>>::uninit().assume_init()};

    unsafe {
        verify_with_configuration::<
            DefaultNonDeterminismSource,
            Blake2sForEverythingVerifier
        >(&mut proof_state_dst, &mut proof_input_dst);
    }

    // set iterator
    zkos_verifier::prover::nd_source_std::set_iterator(it.clone());
}

fn deserialize_from_file<T: serde::de::DeserializeOwned>(filename: &str) -> T {
    let src = std::fs::File::open(filename).unwrap();
    serde_json::from_reader(src).unwrap()
}
