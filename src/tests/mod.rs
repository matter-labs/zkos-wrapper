use crate::delegation_verifier::skeleton::*;
use crate::transcript::blake2s_reduced_round_function;
use crate::transcript::*;
use crate::verifier_circuit::*;
use crate::wrapper_utils::verifier_traits::CircuitBlake2sForEverythingVerifier;
use crate::wrapper_utils::verifier_traits::CircuitLeafInclusionVerifier;
use boojum::cs::LookupParameters;
use boojum::cs::cs_builder_reference::CsReferenceImplementationBuilder;
use boojum::cs::gates::FmaGateInBaseFieldWithoutConstant;
use boojum::cs::gates::NopGate;
use boojum::cs::gates::SelectionGate;
use boojum::cs::gates::ZeroCheckGate;
use boojum::cs::traits::evaluator::GateConstraintEvaluator;
use boojum::gadgets::tables::RangeCheck15BitsTable;
use boojum::gadgets::tables::RangeCheck16BitsTable;
use boojum::gadgets::tables::create_range_check_15_bits_table;
use boojum::gadgets::tables::create_range_check_16_bits_table;
use boojum::{
    blake2::*,
    cs::{
        CSGeometry,
        gates::{
            ConstantsAllocatorGate, ReductionGate, U32AddCarryAsChunkGate,
            U32TriAddCarryAsChunkGate, UIntXAddGate,
        },
        traits::{cs::ConstraintSystem, gate::GatePlacementStrategy},
    },
    dag::CircuitResolverOpts,
    gadgets::blake2s::mixing_function::Word,
    gadgets::{
        tables::{
            byte_split::{ByteSplitTable, create_byte_split_table},
            xor8::{Xor8Table, create_xor8_table},
        },
        traits::witnessable::WitnessHookable,
        u32::UInt32,
    },
};
use std::alloc::Global;
use std::mem::MaybeUninit;
use zkos_verifier::prover::prover_stages::Proof;
// use zkos_verifier::{blake2s_u32::CONFIGURED_IV, prover::cs::cs::circuit, skeleton};

type F = boojum::field::goldilocks::GoldilocksField;

// #[test]
// fn test_transcript_circuit() {
//     let input_len = 64;
//     let mut input_u32: Vec<u32> = vec![];
//     for i in 0..input_len {
//         input_u32.push(i as u32);
//     }

// }

// #[test]
// fn test_single_round_exact() {
//     test_blake2s(64);
// }

#[test]
fn test_blake2s_round_function() {
    let len = 16;
    use rand::{Rng, SeedableRng};
    let mut rng = rand::rngs::StdRng::seed_from_u64(42);

    let mut input = vec![];
    for _ in 0..len {
        let byte: u32 = rng.r#gen();
        input.push(byte);
    }

    let mut hasher = zkos_verifier::blake2s_u32::Blake2sState::new();
    hasher
        .input_buffer
        .iter_mut()
        .enumerate()
        .for_each(|(i, x)| {
            *x = input[i];
        });
    unsafe {
        hasher.run_round_function::<true>(len, true);
    }
    let reference_output = hasher.read_state_for_output();

    let geometry = CSGeometry {
        num_columns_under_copy_permutation: 20,
        num_witness_columns: 0,
        num_constant_columns: 4,
        max_allowed_constraint_degree: 4,
    };

    use boojum::config::DevCSConfig;
    use boojum::cs::cs_builder_reference::*;
    let builder_impl =
        CsReferenceImplementationBuilder::<F, F, DevCSConfig>::new(geometry, 1 << 17);
    use boojum::cs::cs_builder::new_builder;
    let builder = new_builder::<_, F>(builder_impl);

    let builder = builder.allow_lookup(
        boojum::cs::LookupParameters::UseSpecializedColumnsWithTableIdAsConstant {
            width: 3,
            num_repetitions: 5,
            share_table_id: true,
        },
    );
    let builder = ConstantsAllocatorGate::configure_builder(
        builder,
        GatePlacementStrategy::UseGeneralPurposeColumns,
    );
    let builder = U32TriAddCarryAsChunkGate::configure_builder(
        builder,
        GatePlacementStrategy::UseGeneralPurposeColumns,
    );
    let builder = ReductionGate::<F, 4>::configure_builder(
        builder,
        GatePlacementStrategy::UseGeneralPurposeColumns,
    );

    let mut owned_cs = builder.build(CircuitResolverOpts::new(1 << 20));

    // add tables
    let table = create_xor8_table();
    owned_cs.add_lookup_table::<Xor8Table, 3>(table);

    let table = create_byte_split_table::<F, 4>();
    owned_cs.add_lookup_table::<ByteSplitTable<4>, 3>(table);

    let table = create_byte_split_table::<F, 7>();
    owned_cs.add_lookup_table::<ByteSplitTable<7>, 3>(table);

    let table = create_byte_split_table::<F, 1>();
    owned_cs.add_lookup_table::<ByteSplitTable<1>, 3>(table);

    // let table = create_byte_split_table::<F, 2>();
    // owned_cs.add_lookup_table::<ByteSplitTable<2>, 3>(table);

    // let table = create_byte_split_table::<F, 3>();
    // owned_cs.add_lookup_table::<ByteSplitTable<3>, 3>(table);

    let mut circuit_input = vec![];

    let cs = &mut owned_cs;

    for val in input.iter() {
        let val = UInt32::allocate_checked(cs, *val);
        circuit_input.push(val);
    }

    let mut hasher = Blake2sStateGate::new(cs);
    hasher
        .input_buffer
        .iter_mut()
        .enumerate()
        .for_each(|(i, x)| {
            *x = Word {
                inner: circuit_input[i].to_le_bytes(cs),
            };
        });
    hasher.run_round_function::<_, true>(cs, len, true);

    let output = hasher
        .read_state_for_output()
        .map(|el| UInt32::from_le_bytes(cs, el.inner));

    let output = output.witness_hook(cs)().unwrap();
    let reference_output = reference_output.as_slice();
    assert_eq!(output, reference_output);

    let _ = cs;
    let _owned_cs = owned_cs.into_assembly::<Global>();
}

#[test]
fn test_transcript_circuit_initial() {
    test_transcript_circuit(200);
}

fn test_transcript_circuit(len: usize) {
    // use rand::{Rng, SeedableRng};
    // let mut rng = rand::rngs::StdRng::seed_from_u64(42);

    let mut input = vec![];
    for i in 0..len {
        let byte: u32 = i as u32; // rng.r#gen();
        input.push(byte);
    }

    const POW_BITS: usize = 28;

    let mut transcript_hasher = zkos_verifier::blake2s_u32::Blake2sState::new();
    let mut seed = zkos_verifier::transcript::Blake2sTranscript::commit_initial_using_hasher(
        &mut transcript_hasher,
        &input[..],
    );
    zkos_verifier::transcript::Blake2sTranscript::commit_with_seed_using_hasher(
        &mut transcript_hasher,
        &mut seed,
        &input,
    );
    let mut transcript_challenges = unsafe {
        MaybeUninit::<
            [u32; (1usize * 4)
                .next_multiple_of(zkos_verifier::blake2s_u32::BLAKE2S_DIGEST_SIZE_U32_WORDS)],
        >::uninit()
        .assume_init()
    };
    zkos_verifier::transcript::Blake2sTranscript::draw_randomness_using_hasher(
        &mut transcript_hasher,
        &mut seed,
        &mut transcript_challenges,
    );

    // let worker = zkos_verifier_worker::Worker::new_with_num_threads(8);
    // let (mut _seed, pow_nonce) = zkos_verifier::transcript::Blake2sTranscript::search_pow(&seed, POW_BITS, &worker);
    let pow_nonce: u64 = 280946043;
    println!("pow_nonce: {}", pow_nonce);
    // let mut transcript_hasher = zkos_verifier::blake2s_u32::Blake2sState::new();
    zkos_verifier::transcript::Blake2sTranscript::verify_pow_using_hasher(
        &mut transcript_hasher,
        &mut seed,
        pow_nonce as u64,
        POW_BITS as u32,
    );

    let reference_output = transcript_challenges; // seed.0;

    let geometry = CSGeometry {
        num_columns_under_copy_permutation: 20,
        num_witness_columns: 0,
        num_constant_columns: 4,
        max_allowed_constraint_degree: 4,
    };

    use boojum::config::DevCSConfig;
    use boojum::cs::cs_builder_reference::*;
    let builder_impl =
        CsReferenceImplementationBuilder::<F, F, DevCSConfig>::new(geometry, 1 << 17);
    use boojum::cs::cs_builder::new_builder;
    let builder = new_builder::<_, F>(builder_impl);

    let builder = builder.allow_lookup(
        boojum::cs::LookupParameters::UseSpecializedColumnsWithTableIdAsConstant {
            width: 3,
            num_repetitions: 5,
            share_table_id: true,
        },
    );
    let builder = ConstantsAllocatorGate::configure_builder(
        builder,
        GatePlacementStrategy::UseGeneralPurposeColumns,
    );
    let builder = U32TriAddCarryAsChunkGate::configure_builder(
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
        NopGate::configure_builder(builder, GatePlacementStrategy::UseGeneralPurposeColumns);

    let mut owned_cs = builder.build(CircuitResolverOpts::new(1 << 20));

    // add tables
    let table = create_xor8_table();
    owned_cs.add_lookup_table::<Xor8Table, 3>(table);

    let table = create_byte_split_table::<F, 4>();
    owned_cs.add_lookup_table::<ByteSplitTable<4>, 3>(table);

    let table = create_byte_split_table::<F, 7>();
    owned_cs.add_lookup_table::<ByteSplitTable<7>, 3>(table);

    let table = create_byte_split_table::<F, 1>();
    owned_cs.add_lookup_table::<ByteSplitTable<1>, 3>(table);

    // let table = create_byte_split_table::<F, 2>();
    // owned_cs.add_lookup_table::<ByteSplitTable<2>, 3>(table);

    // let table = create_byte_split_table::<F, 3>();
    // owned_cs.add_lookup_table::<ByteSplitTable<3>, 3>(table);

    let mut circuit_input = vec![];

    let cs = &mut owned_cs;

    for pair in input.iter() {
        let pair = UInt32::<F>::allocate_checked(cs, *pair);
        circuit_input.push(pair);
    }

    let pow_nonce = [
        UInt32::allocate_checked(cs, pow_nonce as u32),
        UInt32::allocate_checked(cs, (pow_nonce >> 32) as u32),
    ];

    // let output = blake2s(cs, &circuit_input);
    let mut transcript_hasher = Blake2sStateGate::new(cs);
    let mut seed = Blake2sWrappedTranscript::commit_initial_using_hasher(
        cs,
        &mut transcript_hasher,
        &circuit_input,
    );
    Blake2sWrappedTranscript::commit_with_seed_using_hasher(
        cs,
        &mut transcript_hasher,
        &mut seed,
        &circuit_input,
    );
    let mut transcript_challenges = [UInt32::zero(cs);
        (1usize * 4).next_multiple_of(zkos_verifier::blake2s_u32::BLAKE2S_DIGEST_SIZE_U32_WORDS)];
    Blake2sWrappedTranscript::draw_randomness_using_hasher(
        cs,
        &mut transcript_hasher,
        &mut seed,
        &mut transcript_challenges,
    );

    Blake2sWrappedTranscript::verify_pow_using_hasher::<_, _, POW_BITS>(
        cs,
        &mut transcript_hasher,
        &mut seed,
        pow_nonce,
        // POW_BITS as u32,
    );
    let output = transcript_challenges; // seed.0.map(|el| UInt32::from_le_bytes(cs, el.inner));

    let output = output.witness_hook(cs)().unwrap();
    let reference_output = reference_output.as_slice();
    assert_eq!(output, reference_output);

    let _ = cs;
    let worker = boojum::worker::Worker::new_with_num_threads(8);

    owned_cs.pad_and_shrink();
    let mut owned_cs = owned_cs.into_assembly::<Global>();
    assert!(owned_cs.check_if_satisfied(&worker));
}

#[test]
fn test_leaf_inclusion() {
    let geometry = CSGeometry {
        num_columns_under_copy_permutation: 80,
        num_witness_columns: 0,
        num_constant_columns: 4,
        max_allowed_constraint_degree: 4,
    };

    use boojum::config::DevCSConfig;
    use boojum::cs::cs_builder_reference::*;
    let builder_impl =
        CsReferenceImplementationBuilder::<F, F, DevCSConfig>::new(geometry, 1 << 20);
    use boojum::cs::cs_builder::new_builder;
    let builder = new_builder::<_, F>(builder_impl);

    let builder = builder.allow_lookup(
        boojum::cs::LookupParameters::UseSpecializedColumnsWithTableIdAsConstant {
            width: 3,
            num_repetitions: 20,
            share_table_id: true,
        },
    );
    let builder = ConstantsAllocatorGate::configure_builder(
        builder,
        GatePlacementStrategy::UseGeneralPurposeColumns,
    );
    // let builder = ZeroCheckGate::configure_builder(
    //     builder,
    //     GatePlacementStrategy::UseGeneralPurposeColumns,
    //     false,
    // );
    let builder = FmaGateInBaseFieldWithoutConstant::configure_builder(
        builder,
        GatePlacementStrategy::UseGeneralPurposeColumns,
    );
    let builder = U32TriAddCarryAsChunkGate::configure_builder(
        builder,
        GatePlacementStrategy::UseGeneralPurposeColumns,
    );
    let builder = ReductionGate::<F, 4>::configure_builder(
        builder,
        GatePlacementStrategy::UseGeneralPurposeColumns,
    );
    let builder = ReductionGate::<F, 2>::configure_builder(
        builder,
        GatePlacementStrategy::UseGeneralPurposeColumns,
    );
    let builder =
        SelectionGate::configure_builder(builder, GatePlacementStrategy::UseGeneralPurposeColumns);
    let builder = UIntXAddGate::<16>::configure_builder(
        builder,
        GatePlacementStrategy::UseGeneralPurposeColumns,
    );
    let builder = UIntXAddGate::<8>::configure_builder(
        builder,
        GatePlacementStrategy::UseGeneralPurposeColumns,
    );
    let builder =
        NopGate::configure_builder(builder, GatePlacementStrategy::UseGeneralPurposeColumns);

    let mut owned_cs = builder.build(CircuitResolverOpts::new(1 << 25));

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
    let (proof, _, _) = read_and_verify_proof(&"delegation_proof".to_string());
    set_iterator_from_proof(&proof);

    let mut leaf_inclusion_verifier = CircuitBlake2sForEverythingVerifier::new(cs);
    let skeleton = unsafe {
        WrappedProofSkeletonInstance::from_non_determinism_source::<_, DefaultNonDeterminismSource>(
            cs,
        )
    };

    let _queries: [_; NUM_QUERIES] = std::array::from_fn(|_idx| unsafe {
        WrappedQueryValuesInstance::from_non_determinism_source::<_, DefaultNonDeterminismSource, _>(
            cs,
            &skeleton,
            &mut leaf_inclusion_verifier,
        )
    });

    let _ = cs;
    let worker = boojum::worker::Worker::new_with_num_threads(8);

    owned_cs.pad_and_shrink();
    let mut owned_cs = owned_cs.into_assembly::<Global>();
    assert!(owned_cs.check_if_satisfied(&worker));
}

#[test]
fn test_decompose() {
    use rand::{Rng, SeedableRng};
    let mut rng = rand::rngs::StdRng::seed_from_u64(42);

    let input: u32 = rng.r#gen();

    let reference_output = input;

    let geometry = CSGeometry {
        num_columns_under_copy_permutation: 20,
        num_witness_columns: 0,
        num_constant_columns: 4,
        max_allowed_constraint_degree: 4,
    };

    use boojum::config::DevCSConfig;
    use boojum::cs::cs_builder_reference::*;
    let builder_impl =
        CsReferenceImplementationBuilder::<F, F, DevCSConfig>::new(geometry, 1 << 17);
    use boojum::cs::cs_builder::new_builder;
    let builder = new_builder::<_, F>(builder_impl);

    let builder = builder.allow_lookup(
        boojum::cs::LookupParameters::UseSpecializedColumnsWithTableIdAsConstant {
            width: 3,
            num_repetitions: 5,
            share_table_id: true,
        },
    );
    let builder = ConstantsAllocatorGate::configure_builder(
        builder,
        GatePlacementStrategy::UseGeneralPurposeColumns,
    );
    let builder = U32TriAddCarryAsChunkGate::configure_builder(
        builder,
        GatePlacementStrategy::UseGeneralPurposeColumns,
    );
    let builder = ReductionGate::<F, 4>::configure_builder(
        builder,
        GatePlacementStrategy::UseGeneralPurposeColumns,
    );

    let mut owned_cs = builder.build(CircuitResolverOpts::new(1 << 20));

    // add tables
    let table = create_xor8_table();
    owned_cs.add_lookup_table::<Xor8Table, 3>(table);

    let table = create_byte_split_table::<F, 4>();
    owned_cs.add_lookup_table::<ByteSplitTable<4>, 3>(table);

    let table = create_byte_split_table::<F, 7>();
    owned_cs.add_lookup_table::<ByteSplitTable<7>, 3>(table);

    let table = create_byte_split_table::<F, 1>();
    owned_cs.add_lookup_table::<ByteSplitTable<1>, 3>(table);

    let table = create_byte_split_table::<F, 2>();
    owned_cs.add_lookup_table::<ByteSplitTable<2>, 3>(table);

    let table = create_byte_split_table::<F, 3>();
    owned_cs.add_lookup_table::<ByteSplitTable<3>, 3>(table);

    let cs = &mut owned_cs;

    let circuit_input = UInt32::<F>::allocate_checked(cs, input);

    let output = circuit_input.to_le_bytes(cs);
    let output = UInt32::from_le_bytes(cs, output);

    let output = output.witness_hook(cs)().unwrap();
    let reference_output = reference_output; //.as_slice();
    assert_eq!(output, reference_output);

    let _ = cs;
    let worker = boojum::worker::Worker::new_with_num_threads(8);

    owned_cs.pad_and_shrink();
    let mut owned_cs = owned_cs.into_assembly::<Global>();
    assert!(owned_cs.check_if_satisfied(&worker));
}

use crate::wrapper_utils::prover_structs::*;
use zkos_verifier::concrete::size_constants::*;
use zkos_verifier::prover::definitions::Blake2sForEverythingVerifier;
use zkos_verifier::verifier_common::{DefaultNonDeterminismSource, ProofOutput, ProofPublicInputs};

#[test]
fn test_verifier_inner_function() {
    // allocate CS
    let geometry = CSGeometry {
        num_columns_under_copy_permutation: 8,
        num_witness_columns: 0,
        num_constant_columns: 4,
        max_allowed_constraint_degree: 4,
    };

    use boojum::config::DevCSConfig;
    use boojum::cs::cs_builder_reference::*;
    let builder_impl =
        CsReferenceImplementationBuilder::<F, F, DevCSConfig>::new(geometry, 1 << 20);
    use boojum::cs::cs_builder::new_builder;
    let builder = new_builder::<_, F>(builder_impl);

    let builder = builder.allow_lookup(
        LookupParameters::UseSpecializedColumnsWithTableIdAsConstant {
            width: 3,
            num_repetitions: 17,
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
        SelectionGate::configure_builder(builder, GatePlacementStrategy::UseSpecializedColumns {
            num_repetitions: 1,
            share_constants: true,
        });
    let builder = U32TriAddCarryAsChunkGate::configure_builder(
        builder,
        GatePlacementStrategy::UseSpecializedColumns {
            num_repetitions: 1,
            share_constants: true,
        },
    );
    let builder = U32AddCarryAsChunkGate::configure_builder(
        builder,
        GatePlacementStrategy::UseSpecializedColumns {
            num_repetitions: 1,
            share_constants: true,
        },
    );
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

    let mut owned_cs = builder.build(CircuitResolverOpts::new(1 << 25));

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
        read_and_verify_proof(&"delegation_proof".to_string());

    // allocate prove parts
    let (skeleton, queries) =
        prepare_proof_for_wrapper::<F, _, CircuitBlake2sForEverythingVerifier<F>>(cs, &proof);

    // verify function
    println!("Start verification");
    let (proof_state_dst, proof_input_dst) =
        crate::delegation_verifier::verify(cs, skeleton, queries);

    // let proof: Proof = deserialize_from_file(&"blake2s_delegator_proof");

    // // allocate prove parts
    // let (skeleton, queries) =
    //     prepare_proof_for_wrapper::<F, _, CircuitBlake2sForEverythingVerifier<F>>(cs, &proof);

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

    let worker = boojum::worker::Worker::new_with_num_threads(8);

    dbg!(cs.next_available_row());

    let _ = cs;
    owned_cs.pad_and_shrink();
    let mut owned_cs = owned_cs.into_assembly::<Global>();
    owned_cs.print_gate_stats();
    assert!(owned_cs.check_if_satisfied(&worker));
}

pub(crate) fn read_and_verify_proof(
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

#[test]
fn test_wrapper_circuit() {
    let worker = boojum::worker::Worker::new_with_num_threads(4);
    let (proof, _, _) = read_and_verify_proof(&"delegation_proof".to_string());

    let (finalization_hint, setup_base, setup, vk, setup_tree, vars_hint, witness_hints) =
        crate::get_zkos_wrapper_setup(&worker);

    let proof = crate::get_zkos_wrapper_proof(
        proof,
        &finalization_hint,
        &setup_base,
        &setup,
        &vk,
        &setup_tree,
        &vars_hint,
        &witness_hints,
        &worker,
    );

    let is_valid = crate::verify_zkos_wrapper_proof(&proof, &vk);

    assert!(is_valid);
}
