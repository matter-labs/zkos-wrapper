use crate::circuits::RiscWrapperWitness;

use super::*;

#[test]
pub(crate) fn risc_wrapper_full_test() {
    let worker = boojum::worker::Worker::new_with_num_threads(4);

    let final_registers_state: Vec<u32> = deserialize_from_file(RISC_REGISTER_FINAL_STATE_PATH);
    let risc_proof = deserialize_from_file(RISC_PROOF_PATH);

    let risc_wrapper_witness = RiscWrapperWitness {
        final_registers_state: final_registers_state.try_into().unwrap(),
        proof: risc_proof,
    };

    let (
        finalization_hint,
        setup_base,
        setup,
        risc_wrapper_vk,
        setup_tree,
        vars_hint,
        witness_hints,
    ) = crate::get_risc_wrapper_setup(&worker);

    let risc_wrapper_proof = crate::prove_risc_wrapper(
        risc_wrapper_witness,
        &finalization_hint,
        &setup_base,
        &setup,
        &risc_wrapper_vk,
        &setup_tree,
        &vars_hint,
        &witness_hints,
        &worker,
    );

    let is_valid = crate::verify_risc_wrapper_proof(&risc_wrapper_proof, &risc_wrapper_vk);

    assert!(is_valid);

    serialize_to_file(&risc_wrapper_proof, RISC_WRAPPER_PROOF_PATH);
    serialize_to_file(&risc_wrapper_vk, RISC_WRAPPER_VK_PATH);
}

#[test]
pub(crate) fn risc_wrapper_setup_test() {
    let worker = boojum::worker::Worker::new_with_num_threads(4);

    let (_finalization_hint, _setup_base, _setup, vk, _setup_tree, _vars_hint, _witness_hints) =
        crate::get_risc_wrapper_setup(&worker);

    serialize_to_file(&vk, RISC_WRAPPER_VK_PATH);
}

#[test]
fn test_verifier_inner_function() {
    // allocate CS
    let geometry = CSGeometry {
        num_columns_under_copy_permutation: 51,
        num_witness_columns: 0,
        num_constant_columns: 4,
        max_allowed_constraint_degree: 4,
    };

    use boojum::config::DevCSConfig;
    // use boojum::config::SetupCSConfig;
    use boojum::cs::cs_builder_reference::*;
    let builder_impl =
        CsReferenceImplementationBuilder::<F, F, DevCSConfig>::new(geometry, 1 << 20);
    use boojum::cs::cs_builder::new_builder;
    let builder = new_builder::<_, F>(builder_impl);

    let builder = builder.allow_lookup(
        LookupParameters::UseSpecializedColumnsWithTableIdAsConstant {
            width: 3,
            num_repetitions: 21,
            share_table_id: true,
        },
    );

    let builder = ConstantsAllocatorGate::configure_builder(
        builder,
        GatePlacementStrategy::UseGeneralPurposeColumns,
    );
    let builder = FmaGateInBaseFieldWithoutConstant::configure_builder(
        builder,
        GatePlacementStrategy::UseGeneralPurposeColumns,
    );
    let builder = ReductionGate::<F, 4>::configure_builder(
        builder,
        GatePlacementStrategy::UseGeneralPurposeColumns,
    );
    let builder = UIntXAddGate::<16>::configure_builder(
        builder,
        GatePlacementStrategy::UseGeneralPurposeColumns,
    );
    let builder = UIntXAddGate::<8>::configure_builder(
        builder,
        GatePlacementStrategy::UseGeneralPurposeColumns,
    );
    let builder =
        SelectionGate::configure_builder(builder, GatePlacementStrategy::UseGeneralPurposeColumns);
    let builder = U32TriAddCarryAsChunkGate::configure_builder(
        builder,
        GatePlacementStrategy::UseGeneralPurposeColumns,
    );
    // let builder = U32AddCarryAsChunkGate::configure_builder(
    //     builder,
    //     GatePlacementStrategy::UseGeneralPurposeColumns,
    // );
    let builder =
        NopGate::configure_builder(builder, GatePlacementStrategy::UseGeneralPurposeColumns);

    let builder = ReductionGate::<F, 2>::configure_builder(
        builder,
        GatePlacementStrategy::UseGeneralPurposeColumns,
    );
    let builder = ZeroCheckGate::configure_builder(
        builder,
        GatePlacementStrategy::UseGeneralPurposeColumns,
        false,
    );

    let mut owned_cs = builder.build(CircuitResolverOpts::new(1 << 26));

    // add tables
    let table = create_range_check_16_bits_table::<3, F>();
    owned_cs.add_lookup_table::<RangeCheck16BitsTable<3>, 3>(table);

    let table = create_range_check_15_bits_table::<3, F>();
    owned_cs.add_lookup_table::<RangeCheck15BitsTable<3>, 3>(table);

    let table = create_xor8_table();
    owned_cs.add_lookup_table::<Xor8Table, 3>(table);

    let table = create_byte_split_table::<F, 4>();
    owned_cs.add_lookup_table::<ByteSplitTable<4>, 3>(table);

    let table = create_byte_split_table::<F, 7>();
    owned_cs.add_lookup_table::<ByteSplitTable<7>, 3>(table);

    let table = create_byte_split_table::<F, 1>();
    owned_cs.add_lookup_table::<ByteSplitTable<1>, 3>(table);

    let cs = &mut owned_cs;

    // read proof and set iterator
    let (proof, expected_proof_state_dst, expected_proof_input_dst) =
        read_and_verify_risc_proof(&"proof".to_string());

    // use boojum::gadgets::traits::allocatable::CSAllocatable;
    // use crate::wrapper_utils::verifier_traits::PlaceholderSource;

    // // allocate from placeholder
    // let skeleton_witness = WrappedProofSkeletonInstance::<F>::placeholder_witness();
    // let skeleton = WrappedProofSkeletonInstance::allocate(cs, skeleton_witness);

    // let mut leaf_inclusion_verifier = CircuitBlake2sForEverythingVerifier::<F>::new(cs);

    // let queries: [_; NUM_QUERIES] = std::array::from_fn(|_idx| unsafe {
    //     WrappedQueryValuesInstance::from_non_determinism_source::<_, PlaceholderSource, _>(
    //         cs,
    //         &skeleton,
    //         &mut leaf_inclusion_verifier,
    //     )
    // });

    // allocate prove parts
    let (skeleton, queries) =
        crate::prepare_proof_for_wrapper::<F, _, CircuitBlake2sForEverythingVerifier<F>>(
            cs, &proof,
        );

    // verify function
    println!("Start verification");
    let (proof_state_dst, proof_input_dst) =
        crate::wrapper_inner_verifier::verify(cs, skeleton, queries);

    // pub setup_caps: [WrappedMerkleTreeCap<F, CAP_SIZE>; NUM_COSETS],
    // pub memory_caps: [WrappedMerkleTreeCap<F, CAP_SIZE>; NUM_COSETS],
    // pub memory_challenges: WrappedExternalMemoryArgumentChallenges<F>,
    // pub delegation_challenges:
    //     [WrappedExternalDelegationArgumentChallenges<F>; NUM_DELEGATION_CHALLENGES],
    // pub lazy_init_boundary_values: [WrappedAuxArgumentsBoundaryValues<F>; NUM_AUX_BOUNDARY_VALUES],
    // pub memory_grand_product_accumulator: MersenneQuartic<F>,
    // pub delegation_argument_accumulator: [MersenneQuartic<F>; NUM_DELEGATION_CHALLENGES],
    // pub circuit_sequence: UInt32<F>,
    // pub delegation_type: UInt32<F>,
    dbg!(expected_proof_state_dst.circuit_sequence);
    dbg!(expected_proof_state_dst.delegation_type);
    dbg!(expected_proof_state_dst.delegation_argument_accumulator);
    dbg!(expected_proof_state_dst.memory_grand_product_accumulator);
    dbg!(expected_proof_state_dst.lazy_init_boundary_values);
    dbg!(NUM_DELEGATION_CHALLENGES, NUM_AUX_BOUNDARY_VALUES);
    dbg!(expected_proof_input_dst);

    // verify outputs
    for (a, b) in proof_state_dst
        .setup_caps
        .iter()
        .zip(expected_proof_state_dst.setup_caps.iter())
    {
        assert_eq!(a.cap.witness_hook(cs)().unwrap(), b.cap);
    }
    for (a, b) in proof_state_dst
        .memory_caps
        .iter()
        .zip(expected_proof_state_dst.memory_caps.iter())
    {
        assert_eq!(a.cap.witness_hook(cs)().unwrap(), b.cap);
    }
    assert_eq!(
        proof_state_dst
            .memory_challenges
            .memory_argument_linearization_challenges
            .witness_hook(cs)()
        .unwrap(),
        expected_proof_state_dst
            .memory_challenges
            .memory_argument_linearization_challenges
    );
    assert_eq!(
        proof_state_dst
            .memory_challenges
            .memory_argument_gamma
            .witness_hook(cs)()
        .unwrap(),
        expected_proof_state_dst
            .memory_challenges
            .memory_argument_gamma
    );
    for (a, b) in proof_state_dst
        .delegation_challenges
        .iter()
        .zip(expected_proof_state_dst.delegation_challenges.iter())
    {
        assert_eq!(
            a.delegation_argument_linearization_challenges
                .witness_hook(cs)()
            .unwrap(),
            b.delegation_argument_linearization_challenges
        );
        assert_eq!(
            a.delegation_argument_gamma.witness_hook(cs)().unwrap(),
            b.delegation_argument_gamma
        );
    }
    for (a, b) in proof_state_dst
        .lazy_init_boundary_values
        .iter()
        .zip(expected_proof_state_dst.lazy_init_boundary_values.iter())
    {
        assert_eq!(
            a.lazy_init_first_row.witness_hook(cs)().unwrap(),
            b.lazy_init_first_row
        );
        assert_eq!(
            a.lazy_init_one_before_last_row.witness_hook(cs)().unwrap(),
            b.lazy_init_one_before_last_row
        );
    }
    assert_eq!(
        proof_state_dst
            .memory_grand_product_accumulator
            .witness_hook(cs)()
        .unwrap(),
        expected_proof_state_dst.memory_grand_product_accumulator
    );
    assert_eq!(
        proof_state_dst
            .delegation_argument_accumulator
            .witness_hook(cs)()
        .unwrap(),
        expected_proof_state_dst.delegation_argument_accumulator
    );
    assert_eq!(
        proof_state_dst.circuit_sequence.witness_hook(cs)().unwrap(),
        expected_proof_state_dst.circuit_sequence
    );
    assert_eq!(
        proof_state_dst.delegation_type.witness_hook(cs)().unwrap(),
        expected_proof_state_dst.delegation_type
    );

    assert_eq!(
        proof_input_dst.input_state_variables.witness_hook(cs)().unwrap(),
        expected_proof_input_dst.input_state_variables
    );
    assert_eq!(
        proof_input_dst.output_state_variables.witness_hook(cs)().unwrap(),
        expected_proof_input_dst.output_state_variables
    );

    let worker = boojum::worker::Worker::new_with_num_threads(4);

    dbg!(cs.next_available_row());

    let _ = cs;
    owned_cs.pad_and_shrink();
    let mut owned_cs = owned_cs.into_assembly::<Global>();
    owned_cs.print_gate_stats();
    assert!(owned_cs.check_if_satisfied(&worker));
}

fn read_and_verify_risc_proof(
    proof_path: &str,
) -> (
    RiscProof,
    ProofOutput<TREE_CAP_SIZE, NUM_COSETS, NUM_DELEGATION_CHALLENGES, NUM_AUX_BOUNDARY_VALUES>,
    ProofPublicInputs<NUM_STATE_ELEMENTS>,
) {
    // read proof from file
    println!("Verifying proof from {}", proof_path);
    let proof: RiscProof = deserialize_from_file(proof_path);

    // verify proof
    let (proof_state_dst, proof_input_dst) =
        crate::verify_risc_proof::<Blake2sForEverythingVerifier>(&proof);

    (proof, proof_state_dst, proof_input_dst)
}
