use risc_verifier::prover::{definitions::Blake2sForEverythingVerifier, prover_stages::Proof};

use crate::circuits::RiscWrapperWitness;

pub fn read_and_verify_proof(proof_path: &String, registers_path: &String) -> RiscWrapperWitness {
    // read proof from file
    println!("Verifying proof from {}", proof_path);
    let proof: Proof = deserialize_from_file(proof_path);
    let final_registers_state: Vec<u32> = deserialize_from_file(registers_path);

    // verify proof
    let (_, _) = crate::verify_risc_proof::<Blake2sForEverythingVerifier>(&proof);

    // TODO: have a quick way of checking that final register values are matching the proof.
    // currently this is done only once we start generating the wapper proof (which is late).

    let risc_wrapper_witness = RiscWrapperWitness {
        final_registers_state: final_registers_state.try_into().unwrap(),
        proof,
    };

    risc_wrapper_witness
}

fn deserialize_from_file<T: serde::de::DeserializeOwned>(filename: &str) -> T {
    let src = std::fs::File::open(filename).unwrap();
    serde_json::from_reader(src).unwrap()
}
