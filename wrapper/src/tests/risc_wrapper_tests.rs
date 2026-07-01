use crate::circuits::{BinaryCommitment, RiscWrapperWitness};
#[cfg(feature = "wrap_with_reduced_log_23")]
use blake_verifier::verifier_common::ProofOutput as BlakeProofOutput;
#[cfg(feature = "wrap_with_reduced_log_23")]
use boojum::gadgets::traits::allocatable::CSAllocatable;
#[cfg(feature = "wrap_with_reduced_log_23")]
use risc_verifier::field::{Field, Mersenne31Quartic};
#[cfg(feature = "wrap_with_reduced_log_23")]
use risc_verifier::prover::definitions::{
    ExternalChallenges, produce_register_contribution_into_memory_accumulator_raw,
};
#[cfg(feature = "wrap_with_reduced_log_23")]
use risc_verifier::prover::risc_v_simulator::cycle::state::NUM_REGISTERS;
#[cfg(feature = "wrap_with_reduced_log_23")]
use risc_verifier::prover::transcript::Blake2sBufferingTranscript;
#[cfg(feature = "wrap_with_reduced_log_23")]
use std::panic::{AssertUnwindSafe, catch_unwind};

use super::*;

#[cfg(feature = "wrap_with_reduced_log_23")]
type BaseProofOutputWitness =
    ProofOutput<TREE_CAP_SIZE, NUM_COSETS, NUM_DELEGATION_CHALLENGES, NUM_AUX_BOUNDARY_VALUES>;
#[cfg(feature = "wrap_with_reduced_log_23")]
type BlakeProofOutputWitness = BlakeProofOutput<
    { blake_verifier::concrete::size_constants::TREE_CAP_SIZE },
    { blake_verifier::concrete::size_constants::NUM_COSETS },
    { blake_verifier::concrete::size_constants::NUM_DELEGATION_CHALLENGES },
    { blake_verifier::concrete::size_constants::NUM_AUX_BOUNDARY_VALUES },
>;

#[test]
pub(crate) fn risc_wrapper_full_test() {
    let worker = boojum::worker::Worker::new();

    let program_proof: execution_utils::ProgramProof = deserialize_from_file(RISC_PROOF_PATH);
    let binary_commitment = BinaryCommitment::from_default_binary();

    let risc_wrapper_witness =
        RiscWrapperWitness::from_full_proof(program_proof, &binary_commitment);

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

    serialize_to_file(&risc_wrapper_proof, RISC_WRAPPER_PROOF_PATH);
    serialize_to_file(&risc_wrapper_vk, RISC_WRAPPER_VK_PATH);
}

#[test]
pub(crate) fn risc_wrapper_setup_test() {
    let worker = boojum::worker::Worker::new();
    let binary_commitment = BinaryCommitment::from_default_binary();

    let (_finalization_hint, _setup_base, _setup, vk, _setup_tree, _vars_hint, _witness_hints) =
        crate::get_risc_wrapper_setup(&worker, binary_commitment);

    serialize_to_file(&vk, RISC_WRAPPER_VK_PATH);
}

#[cfg(feature = "wrap_with_reduced_log_23")]
fn expected_reduced_log_23_blake_delegation_parameters() -> (
    u32,
    &'static [risc_verifier::prover::definitions::MerkleTreeCap<TREE_CAP_SIZE>; NUM_COSETS],
) {
    let parameters = execution_utils::RECURSION_LAYER_CIRCUITS_VERIFICATION_PARAMETERS;
    assert_eq!(
        parameters.len(),
        1,
        "Expected exactly one reduced-log delegation circuit parameter set",
    );

    parameters[0]
}

#[cfg(feature = "wrap_with_reduced_log_23")]
fn load_wrapper_constraint_fixture() -> (
    BinaryCommitment,
    [u32; NUM_REGISTERS * 3],
    BaseProofOutputWitness,
    ProofPublicInputs<NUM_STATE_ELEMENTS>,
    BlakeProofOutputWitness,
) {
    let program_proof: execution_utils::ProgramProof = deserialize_from_file(RISC_PROOF_PATH);
    let binary_commitment = BinaryCommitment::from_default_binary();
    let witness = RiscWrapperWitness::from_full_proof(program_proof, &binary_commitment);
    let final_registers_state = witness.final_registers_state;
    let (proof_state, proof_input) =
        crate::verify_risc_proof::<Blake2sForEverythingVerifier>(&witness.proof);
    let (blake_state, _) = crate::blake2_inner_verifier::verify_blake_proof::<
        Blake2sForEverythingVerifier,
    >(&witness.blake_proof);

    (
        binary_commitment,
        final_registers_state,
        proof_state,
        proof_input,
        blake_state,
    )
}

#[cfg(feature = "wrap_with_reduced_log_23")]
fn wrapper_constraints_are_satisfied(
    binary_commitment: BinaryCommitment,
    final_registers_state_witness: [u32; NUM_REGISTERS * 3],
    proof_state_witness: BaseProofOutputWitness,
    public_input_witness: ProofPublicInputs<NUM_STATE_ELEMENTS>,
    blake_state_witness: BlakeProofOutputWitness,
) -> bool {
    use boojum::config::DevCSConfig;
    use boojum::cs::cs_builder::new_builder;
    use boojum::cs::cs_builder_reference::CsReferenceImplementationBuilder;
    use boojum::cs::gates::{PublicInputGate, ZeroCheckGate};
    use boojum::cs::traits::circuit::CircuitBuilder;
    use boojum::dag::CircuitResolverOpts;
    use boojum::gadgets::num::Num;

    catch_unwind(AssertUnwindSafe(|| {
        let worker = boojum::worker::Worker::new_with_num_threads(4);
        let circuit = crate::RiscWrapper::new(None, false, binary_commitment);
        let (max_trace_len, num_vars) = circuit.size_hint();
        let builder_impl = CsReferenceImplementationBuilder::<F, F, DevCSConfig>::new(
            crate::RiscWrapper::geometry(),
            max_trace_len.unwrap(),
        );
        let builder = new_builder::<_, F>(builder_impl);
        let builder = crate::RiscWrapper::configure_builder(builder);
        let mut owned_cs = builder.build(CircuitResolverOpts::new(num_vars.unwrap()));
        circuit.add_tables(&mut owned_cs);

        let cs = &mut owned_cs;
        let final_registers_state =
            <[UInt32<F>; NUM_REGISTERS * 3]>::allocate(cs, final_registers_state_witness);
        let proof_state = crate::wrapper_utils::prover_structs::WrappedProofOutput::<
            F,
            TREE_CAP_SIZE,
            NUM_COSETS,
            NUM_DELEGATION_CHALLENGES,
            NUM_AUX_BOUNDARY_VALUES,
        >::allocate(cs, proof_state_witness);
        let public_input = crate::wrapper_utils::prover_structs::WrappedProofPublicInputs::<
            F,
            NUM_STATE_ELEMENTS,
        >::allocate(cs, public_input_witness);
        let blake_state = Some(
            crate::blake2_inner_verifier::WrappedBlakeProofOutput::<F>::allocate(
                cs,
                blake_state_witness,
            ),
        );

        crate::check_proof_state(
            cs,
            final_registers_state,
            &proof_state,
            &public_input,
            &blake_state,
            &binary_commitment,
        );

        let zero = Num::zero(cs);
        let _ = ZeroCheckGate::check_if_zero(cs, zero.get_variable());
        PublicInputGate::new(zero.get_variable()).add_to_cs(cs);

        let _ = cs;
        owned_cs.pad_and_shrink();
        let mut owned_cs = owned_cs.into_assembly::<Global>();
        owned_cs.check_if_satisfied(&worker)
    }))
    .unwrap_or(false)
}

#[cfg(feature = "wrap_with_reduced_log_23")]
fn retarget_blake_delegation_type_while_preserving_old_wrapper_relations(
    final_registers_state: [u32; NUM_REGISTERS * 3],
    proof_state: &mut BaseProofOutputWitness,
    blake_state: &mut BlakeProofOutputWitness,
) {
    let (expected_delegation_type, _) = expected_reduced_log_23_blake_delegation_parameters();
    blake_state.delegation_type = expected_delegation_type + 1;

    let mut transcript = Blake2sBufferingTranscript::new();
    transcript.absorb(&final_registers_state);
    transcript.absorb(proof_state.setup_caps_flattened());
    transcript.absorb(proof_state.memory_caps_flattened());

    let mut delegation_header = [0u32; risc_verifier::blake2s_u32::BLAKE2S_BLOCK_SIZE_U32_WORDS];
    delegation_header[0] = blake_state.delegation_type;
    transcript.absorb(&delegation_header);
    transcript.absorb(blake_state.memory_caps_flattened());

    let memory_seed = transcript.finalize_reset();
    let expected_challenges =
        ExternalChallenges::draw_from_transcript_seed(memory_seed, NUM_DELEGATION_CHALLENGES > 0);

    proof_state.memory_challenges = expected_challenges.memory_argument;
    blake_state.memory_challenges = expected_challenges.memory_argument;
    if let Some(expected_delegation_challenge) = expected_challenges.delegation_argument {
        for challenge in proof_state.delegation_challenges.iter_mut() {
            *challenge = expected_delegation_challenge;
        }
        for challenge in blake_state.delegation_challenges.iter_mut() {
            *challenge = expected_delegation_challenge;
        }
    }

    let register_final_data = core::array::from_fn(|idx| {
        let offset = idx * 3;
        (
            final_registers_state[offset],
            (
                final_registers_state[offset + 1],
                final_registers_state[offset + 2],
            ),
        )
    });
    let register_contribution = produce_register_contribution_into_memory_accumulator_raw(
        &register_final_data,
        proof_state
            .memory_challenges
            .memory_argument_linearization_challenges,
        proof_state.memory_challenges.memory_argument_gamma,
    );

    proof_state.memory_grand_product_accumulator = register_contribution.inverse().unwrap();
    blake_state.memory_grand_product_accumulator = Mersenne31Quartic::ONE;
    proof_state.delegation_argument_accumulator =
        [Mersenne31Quartic::ZERO; NUM_DELEGATION_CHALLENGES];
    blake_state.delegation_argument_accumulator =
        [Mersenne31Quartic::ZERO; NUM_DELEGATION_CHALLENGES];
}

#[cfg(feature = "wrap_with_reduced_log_23")]
#[test]
fn witness_construction_rejects_non_canonical_delegation_layout() {
    let program_proof: execution_utils::ProgramProof = deserialize_from_file(RISC_PROOF_PATH);
    let binary_commitment = BinaryCommitment::from_default_binary();
    let (expected_delegation_type, _) = expected_reduced_log_23_blake_delegation_parameters();
    let canonical_proofs = program_proof
        .delegation_proofs
        .get(&expected_delegation_type)
        .unwrap()
        .clone();
    let wrong_delegation_type = expected_delegation_type + 1;

    let mut wrong_key_proof = program_proof.clone();
    wrong_key_proof.delegation_proofs.clear();
    wrong_key_proof
        .delegation_proofs
        .insert(wrong_delegation_type, canonical_proofs.clone());
    assert!(
        catch_unwind(AssertUnwindSafe(|| {
            RiscWrapperWitness::from_full_proof(wrong_key_proof, &binary_commitment);
        }))
        .is_err()
    );

    let mut extra_entry_proof = program_proof.clone();
    extra_entry_proof
        .delegation_proofs
        .insert(wrong_delegation_type, canonical_proofs.clone());
    assert!(
        catch_unwind(AssertUnwindSafe(|| {
            RiscWrapperWitness::from_full_proof(extra_entry_proof, &binary_commitment);
        }))
        .is_err()
    );

    let mut duplicate_proof_entry = program_proof;
    duplicate_proof_entry
        .delegation_proofs
        .get_mut(&expected_delegation_type)
        .unwrap()
        .push(canonical_proofs[0].clone());
    assert!(
        catch_unwind(AssertUnwindSafe(|| {
            RiscWrapperWitness::from_full_proof(duplicate_proof_entry, &binary_commitment);
        }))
        .is_err()
    );
}

#[cfg(feature = "wrap_with_reduced_log_23")]
#[test]
fn delegated_blake_proof_identity_is_bound_to_expected_recursion_parameters() {
    std::thread::Builder::new()
        .name("delegated-blake-identity".to_owned())
        .stack_size(64 * 1024 * 1024)
        .spawn(|| {
            let (binary_commitment, final_registers_state, proof_state, proof_input, blake_state) =
                load_wrapper_constraint_fixture();

            assert!(wrapper_constraints_are_satisfied(
                binary_commitment,
                final_registers_state,
                proof_state,
                proof_input,
                blake_state,
            ));

            let mut mutated_blake_state = blake_state;
            mutated_blake_state.setup_caps[0].cap[0][0] ^= 1;
            assert!(!wrapper_constraints_are_satisfied(
                binary_commitment,
                final_registers_state,
                proof_state,
                proof_input,
                mutated_blake_state,
            ));

            let mut mutated_blake_state = blake_state;
            mutated_blake_state.circuit_sequence = 1;
            assert!(!wrapper_constraints_are_satisfied(
                binary_commitment,
                final_registers_state,
                proof_state,
                proof_input,
                mutated_blake_state,
            ));

            let mut mutated_proof_state = proof_state;
            let mut mutated_blake_state = blake_state;
            retarget_blake_delegation_type_while_preserving_old_wrapper_relations(
                final_registers_state,
                &mut mutated_proof_state,
                &mut mutated_blake_state,
            );
            assert!(!wrapper_constraints_are_satisfied(
                binary_commitment,
                final_registers_state,
                mutated_proof_state,
                proof_input,
                mutated_blake_state,
            ));
        })
        .unwrap()
        .join()
        .unwrap();
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
        read_and_verify_risc_proof(&"testing_data/risc_proof".to_string());

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
