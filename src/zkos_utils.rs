use risc_verifier::prover::prover_stages::Proof;

use crate::circuits::RiscWrapperWitness;

pub fn read_and_verify_proof(proof_path: &String) -> RiscWrapperWitness {
    // read proof from file
    println!("Verifying proof from {}", proof_path);
    let proof: Proof = deserialize_from_file(proof_path);

    // TODO: read from somewhere.
    let final_registers_state = vec![0u32; 64];

    let risc_wrapper_witness = RiscWrapperWitness {
        final_registers_state: final_registers_state.try_into().unwrap(),
        proof,
    };
    // TODO: check if proof is valid.

    risc_wrapper_witness
}

fn deserialize_from_file<T: serde::de::DeserializeOwned>(filename: &str) -> T {
    let src = std::fs::File::open(filename).unwrap();
    serde_json::from_reader(src).unwrap()
}
