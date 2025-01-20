use zkos_verifier::prover::prover_stages::Proof;

pub(crate) fn verify_proof_and_set_iterator(proof_path: &String) {
    // read proof from file
    println!("Verifying proof from {}", proof_path);
    let proof: Proof = deserialize_from_file(proof_path);

    // verify proof
    // TODO

    // set iterator
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

    zkos_verifier::prover::nd_source_std::set_iterator(it);
}

fn deserialize_from_file<T: serde::de::DeserializeOwned>(filename: &str) -> T {
    let src = std::fs::File::open(filename).unwrap();
    serde_json::from_reader(src).unwrap()
}
