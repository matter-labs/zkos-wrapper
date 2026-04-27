use crate::circuits::{BinaryCommitment, RiscWrapperWitness};

use super::*;

#[test]
pub(crate) fn risc_wrapper_full_test() {
    use std::io::Read;
    let worker = boojum::worker::Worker::new_with_num_threads(32);

    let mut binary = vec![];
    let mut file = std::fs::File::open(RISC_PROGRAM_BIN_PATH).unwrap();
    file.read_to_end(&mut binary).unwrap();

    let mut text = vec![];
    let mut file = std::fs::File::open(RISC_PROGRAM_TEXT_PATH).unwrap();
    file.read_to_end(&mut text).unwrap();

    // We use a prove of hashed fibonacci for testing
    let binary_commitment = BinaryCommitment::from_binary(&binary, &text);
    dbg!(binary_commitment);

    let program_proof: execution_utils::unrolled::UnrolledProgramProof =
        deserialize_from_file(RISC_PROOF_PATH).unwrap();

    let risc_wrapper_witness =
        RiscWrapperWitness::from_full_proof(program_proof, &binary_commitment);

    #[cfg(not(feature = "gpu"))]
    let (risc_wrapper_proof, risc_wrapper_vk) = {
        let (
            finalization_hint,
            setup_base,
            setup,
            risc_wrapper_vk,
            setup_tree,
            vars_hint,
            witness_hints,
        ) = crate::get_risc_wrapper_setup(&worker, binary_commitment.clone());

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
            binary_commitment.clone(),
        );

        let is_valid = crate::verify_risc_wrapper_proof(&risc_wrapper_proof, &risc_wrapper_vk);
        assert!(is_valid);

        (risc_wrapper_proof, risc_wrapper_vk)
    };

    #[cfg(feature = "gpu")]
    let (risc_wrapper_proof, risc_wrapper_vk) = {
        let _prover_context = shivini::ProverContext::create().unwrap();
        let (gpu_setup, gpu_vk, finalization_hint) =
            crate::gpu::risc_wrapper::get_risc_wrapper_setup(&worker, binary_commitment.clone());

        let risc_wrapper_proof = crate::gpu::risc_wrapper::prove_risc_wrapper(
            risc_wrapper_witness,
            &finalization_hint,
            &gpu_setup,
            &gpu_vk,
            &worker,
            binary_commitment.clone(),
        );

        let is_valid = crate::verify_risc_wrapper_proof(&risc_wrapper_proof, &gpu_vk);
        assert!(is_valid);

        (risc_wrapper_proof, gpu_vk)
    };

    serialize_to_file(&risc_wrapper_proof, RISC_WRAPPER_PROOF_PATH).unwrap();
    serialize_to_file(&risc_wrapper_vk, RISC_WRAPPER_VK_PATH).unwrap();
}

#[test]
pub(crate) fn risc_wrapper_setup_test() {
    let worker = boojum::worker::Worker::new();
    let binary_commitment = BinaryCommitment::from_default_binary();

    let (_finalization_hint, _setup_base, _setup, vk, _setup_tree, _vars_hint, _witness_hints) =
        crate::get_risc_wrapper_setup(&worker, binary_commitment);

    serialize_to_file(&vk, RISC_WRAPPER_VK_PATH).unwrap();
}

#[test]
fn test_verifier_inner_function() {
    // allocate CS
    let geometry = CSGeometry {
        num_columns_under_copy_permutation: 180,
        num_witness_columns: 0,
        num_constant_columns: 4,
        max_allowed_constraint_degree: 4,
    };

    use boojum::config::DevCSConfig;
    // use boojum::config::SetupCSConfig;
    use boojum::cs::cs_builder_reference::*;
    let builder_impl =
        CsReferenceImplementationBuilder::<F, F, DevCSConfig, StCircuitResolver<_, _>>::new(
            geometry,
            1 << 20,
        );
    use boojum::cs::cs_builder::new_builder;
    let builder = new_builder::<_, F>(builder_impl);

    let builder = builder.allow_lookup(
        LookupParameters::UseSpecializedColumnsWithTableIdAsConstant {
            width: 3,
            num_repetitions: 80,
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
    use boojum::cs::gates::PublicInputGate;
    let builder = PublicInputGate::configure_builder(
        builder,
        GatePlacementStrategy::UseGeneralPurposeColumns,
    );

    // let mut owned_cs = builder.build(CircuitResolverOpts::new(1 << 27));
    let mut owned_cs = builder.build(1 << 27);

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

    let path = "testing_data/unified_proof_for_hashed_fibonacci.json";
    // let path = "testing_data/risc_proof_80sb.json";
    let program_proof: execution_utils::unrolled::UnrolledProgramProof =
        deserialize_from_file(path).unwrap();
    let binary_commitment = BinaryCommitment::from_default_binary();

    let risc_wrapper_witness =
        RiscWrapperWitness::from_full_proof(program_proof, &binary_commitment);

    use crate::RiscWrapper;

    let circuit = RiscWrapper::new(Some(risc_wrapper_witness), binary_commitment);

    circuit.synthesize_into_cs(cs);

    let worker = boojum::worker::Worker::new_with_num_threads(4);

    dbg!(cs.next_available_row());

    let _ = cs;
    owned_cs.pad_and_shrink();
    let mut owned_cs = owned_cs.into_assembly::<Global>();
    owned_cs.print_gate_stats();
    assert!(owned_cs.check_if_satisfied(&worker));
}
