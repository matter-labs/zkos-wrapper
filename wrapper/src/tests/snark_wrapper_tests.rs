use super::*;
use bellman::kate_commitment::{Crs, CrsForMonomialForm};

#[test]
pub(crate) fn snark_wrapper_full_test() {
    let worker = crate::BellmanWorker::new_with_cpus(4);

    let compression_proof = deserialize_from_file(COMPRESSION_PROOF_PATH);
    let compression_vk: crate::CompressionVK = deserialize_from_file(COMPRESSION_VK_PATH);
    let crs_mons = Crs::<crate::Bn256, CrsForMonomialForm>::crs_42(
        1 << crate::L1_VERIFIER_DOMAIN_SIZE_LOG,
        &worker,
    );

    let (snark_setup, snark_wrapper_vk) =
        crate::get_snark_wrapper_setup(compression_vk.clone(), &crs_mons, &worker);

    let snark_wrapper_proof = crate::prove_snark_wrapper(
        compression_proof,
        compression_vk,
        &snark_setup,
        &crs_mons,
        &worker,
    );

    let is_valid = crate::verify_snark_wrapper_proof(&snark_wrapper_proof, &snark_wrapper_vk);

    assert!(is_valid);

    serialize_to_file(&snark_wrapper_proof, SNARK_WRAPPER_PROOF_PATH);
    serialize_to_file(&snark_wrapper_vk, SNARK_WRAPPER_VK_PATH);
}

#[test]
pub(crate) fn snark_wrapper_setup_test() {
    let worker = crate::BellmanWorker::new_with_cpus(4);

    let compression_vk = deserialize_from_file(COMPRESSION_VK_PATH);
    let crs_mons = Crs::<crate::Bn256, CrsForMonomialForm>::crs_42(
        1 << crate::L1_VERIFIER_DOMAIN_SIZE_LOG,
        &worker,
    );

    let (_snark_wrapper_setup, snark_wrapper_vk) =
        crate::get_snark_wrapper_setup(compression_vk, &crs_mons, &worker);

    serialize_to_file(&snark_wrapper_vk, SNARK_WRAPPER_VK_PATH);
}
