use super::*;

#[test]
pub(crate) fn compression_full_test() {
    let worker = boojum::worker::Worker::new_with_num_threads(4);

    let risc_wrapper_proof = deserialize_from_file(RISC_WRAPPER_PROOF_PATH);
    let risc_wrapper_vk: crate::RiscWrapperVK = deserialize_from_file(RISC_WRAPPER_VK_PATH);

    let (
        finalization_hint,
        setup_base,
        setup,
        compression_vk,
        setup_tree,
        vars_hint,
        witness_hints,
    ) = crate::get_compression_setup(risc_wrapper_vk.clone(), &worker);

    let compression_proof = crate::prove_compression(
        risc_wrapper_proof,
        risc_wrapper_vk,
        &finalization_hint,
        &setup_base,
        &setup,
        &compression_vk,
        &setup_tree,
        &vars_hint,
        &witness_hints,
        &worker,
    );

    let is_valid = crate::verify_compression_proof(&compression_proof, &compression_vk);

    assert!(is_valid);

    serialize_to_file(&compression_proof, COMPRESSION_PROOF_PATH);
    serialize_to_file(&compression_vk, COMPRESSION_VK_PATH);
}

#[test]
pub(crate) fn compression_setup_test() {
    let worker = boojum::worker::Worker::new_with_num_threads(4);

    let risc_wrapper_vk = deserialize_from_file(RISC_WRAPPER_VK_PATH);

    let (
        finalization_hint,
        _setup_base,
        _setup,
        compression_vk,
        _setup_tree,
        _vars_hint,
        _witness_hints,
    ) = crate::get_compression_setup(risc_wrapper_vk, &worker);

    dbg!(finalization_hint);

    serialize_to_file(&compression_vk, COMPRESSION_VK_PATH);
}
