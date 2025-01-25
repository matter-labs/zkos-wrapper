use crate::verifier::transcript::*;
// use boojum::{
//     blake2::*, config::CSConfig, cs::{gates::{
//         ConstantsAllocatorGate, ReductionGate, U32TriAddCarryAsChunkGate, UIntXAddGate, ZeroCheckGate,
//     }, traits::{cs::ConstraintSystem, gate::GatePlacementStrategy}, CSGeometry}, dag::CircuitResolverOpts, gadgets::{blake2s::{blake2s, mixing_function::Word, round_function::Blake2sControl}, tables::{
//         byte_split::{create_byte_split_table, ByteSplitTable},
//         xor8::{create_xor8_table, Xor8Table},
//     }, traits::{allocatable::CSAllocatable, witnessable::WitnessHookable}, u32::UInt32, u8::UInt8},
// };
use std::alloc::Global;
use crate::verifier::Blake2sStateGate;
use crate::verifier::blake2s_reduced_round_function;
use zkos_verifier::{blake2s_u32::CONFIGURED_IV, prover::cs::cs::circuit, skeleton};
use crate::verifier::verifier_traits::CircuitBlake2sForEverythingVerifier;
use crate::verifier::verifier_traits::CircuitLeafInclusionVerifier;
use std::mem::MaybeUninit;
use boojum::cs::traits::evaluator::GateConstraintEvaluator;
use boojum::cs::LookupParameters;
use boojum::cs::cs_builder_reference::CsReferenceImplementationBuilder;
use boojum::cs::gates::DotProductGate;
use boojum::cs::gates::FmaGateInBaseFieldWithoutConstant;
use boojum::cs::gates::NopGate;
use boojum::cs::gates::SelectionGate;
use boojum::cs::gates::ZeroCheckGate;
use boojum::gadgets::tables::RangeCheck15BitsTable;
use boojum::gadgets::tables::RangeCheck16BitsTable;
use boojum::gadgets::tables::create_range_check_15_bits_table;
use boojum::gadgets::tables::create_range_check_16_bits_table;
use boojum::{
    blake2::*,
    config::CSConfig,
    cs::{
        CSGeometry,
        gates::{ConstantsAllocatorGate, ReductionGate, U32TriAddCarryAsChunkGate, UIntXAddGate},
        traits::{cs::ConstraintSystem, gate::GatePlacementStrategy},
    },
    dag::CircuitResolverOpts,
    gadgets::blake2s::mixing_function::Word,
    gadgets::blake2s::round_function::Blake2sControl,
    gadgets::traits::allocatable::CSAllocatable,
    gadgets::{
        blake2s::blake2s,
        tables::{
            byte_split::{ByteSplitTable, create_byte_split_table},
            xor8::{Xor8Table, create_xor8_table},
        },
        traits::witnessable::WitnessHookable,
        u8::UInt8,
        u32::UInt32,
    },
};

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
    type RCfg = <DevCSConfig as CSConfig>::ResolverConfig;
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
    unsafe {
        hasher.run_round_function::<_, true>(cs, len, true);
    }
    let output = hasher
        .read_state_for_output()
        .map(|el| UInt32::from_le_bytes(cs, el.inner));

    let output = output.witness_hook(cs)().unwrap();
    let reference_output = reference_output.as_slice();
    assert_eq!(output, reference_output);

    drop(cs);
    let _owned_cs = owned_cs.into_assembly::<Global>();
}

#[test]
fn test_transcript_circuit_initial() {
    test_transcript_circuit(200);
}

fn test_transcript_circuit(len: usize) {
    use rand::{Rng, SeedableRng};
    let mut rng = rand::rngs::StdRng::seed_from_u64(42);

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
    type RCfg = <DevCSConfig as CSConfig>::ResolverConfig;
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
    let builder = NopGate::configure_builder(
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

    drop(cs);
    let worker = boojum::worker::Worker::new_with_num_threads(8);

    owned_cs.pad_and_shrink();
    let mut owned_cs = owned_cs.into_assembly::<Global>();
    assert!(owned_cs.check_if_satisfied(&worker));
}

#[test]
fn test_leaf_inclusion() {
    crate::prepare_proof::verify_proof_and_set_iterator(&"delegation_proof".to_string());

    // prepare verifier structs
    let (skeleton, queries) = unsafe {
        get_prove_parts::<DefaultNonDeterminismSource, DefaultLeafInclusionVerifier>()
    };

    let geometry = CSGeometry {
        num_columns_under_copy_permutation: 80,
        num_witness_columns: 0,
        num_constant_columns: 4,
        max_allowed_constraint_degree: 4,
    };

    use boojum::config::DevCSConfig;
    type RCfg = <DevCSConfig as CSConfig>::ResolverConfig;
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
    let builder = SelectionGate::configure_builder(
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
    let builder = NopGate::configure_builder(
        builder,
        GatePlacementStrategy::UseGeneralPurposeColumns,
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
    crate::prepare_proof::verify_proof_and_set_iterator(&"delegation_proof".to_string());
   
    let ooc_skeleton = unsafe {
        let mut skeleton = MaybeUninit::<ProofSkeletonInstance>::uninit().assume_init();
        ProofSkeletonInstance::fill::<DefaultNonDeterminismSource>((&mut skeleton) as *mut _);
        skeleton
    };

    let mut leaf_inclusion_verifier = CircuitBlake2sForEverythingVerifier::new(cs);
    let skeleton = unsafe { WrappedProofSkeletonInstance::from_non_determinism_source(cs, ooc_skeleton.clone()) }; 
    
    let queries: [_; NUM_QUERIES] = std::array::from_fn(|_idx| { 
        unsafe { 
            WrappedQueryValuesInstance::from_non_determinism_source::<_,DefaultNonDeterminismSource,_>(
                cs, 
                &skeleton, 
                &mut leaf_inclusion_verifier,
            ) 
        }
    });
    
    drop(cs);
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

    let reference_output: [u8; 4] = std::array::from_fn(|idx| (input >> (idx * 8)) as u8);
    let reference_output = input;

    let geometry = CSGeometry {
        num_columns_under_copy_permutation: 20,
        num_witness_columns: 0,
        num_constant_columns: 4,
        max_allowed_constraint_degree: 4,
    };

    use boojum::config::DevCSConfig;
    type RCfg = <DevCSConfig as CSConfig>::ResolverConfig;
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

    drop(cs);
    let worker = boojum::worker::Worker::new_with_num_threads(8);

    owned_cs.pad_and_shrink();
    let mut owned_cs = owned_cs.into_assembly::<Global>();
    assert!(owned_cs.check_if_satisfied(&worker));
}

use crate::verifier::prover_structs::*;
use zkos_verifier::concrete::size_constants::*;
use zkos_verifier::concrete::skeleton_instance::{ProofSkeletonInstance, QueryValuesInstance};
use zkos_verifier::prover::definitions::Blake2sForEverythingVerifier;
use zkos_verifier::prover::definitions::LeafInclusionVerifier;
use zkos_verifier::verifier_common::non_determinism_source::NonDeterminismSource;
use zkos_verifier::verifier_common::{
    DefaultLeafInclusionVerifier, DefaultNonDeterminismSource, ProofOutput, ProofPublicInputs,
};

#[test]
fn test_verifier_inner_function() {
    // allocate CS
    let geometry = CSGeometry {
        num_columns_under_copy_permutation: 100,
        num_witness_columns: 0,
        num_constant_columns: 4,
        max_allowed_constraint_degree: 4,
    };

    use boojum::config::DevCSConfig;
    type RCfg = <DevCSConfig as CSConfig>::ResolverConfig;
    use boojum::cs::cs_builder_reference::*;
    let builder_impl =
        CsReferenceImplementationBuilder::<F, F, DevCSConfig>::new(geometry, 1 << 20);
    use boojum::cs::cs_builder::new_builder;
    let builder = new_builder::<_, F>(builder_impl);

    let builder = builder.allow_lookup(
        LookupParameters::UseSpecializedColumnsWithTableIdAsConstant {
            width: 3,
            num_repetitions: 20,
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
    let builder = DotProductGate::<4>::configure_builder(
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
    crate::prepare_proof::verify_proof_and_set_iterator(&"delegation_proof".to_string());

    // prepare verifier structs
    let ooc_skeleton = unsafe {
        let mut skeleton = MaybeUninit::<ProofSkeletonInstance>::uninit().assume_init();
        ProofSkeletonInstance::fill::<DefaultNonDeterminismSource>((&mut skeleton) as *mut _);
        skeleton
    };

    let mut leaf_inclusion_verifier = CircuitBlake2sForEverythingVerifier::new(cs);
    let skeleton = unsafe { WrappedProofSkeletonInstance::from_non_determinism_source(cs, ooc_skeleton.clone()) }; 
    
    let queries: [_; NUM_QUERIES] = std::array::from_fn(|_idx| { 
        unsafe { 
            WrappedQueryValuesInstance::from_non_determinism_source::<_,DefaultNonDeterminismSource,_>(
                cs, 
                &skeleton, 
                &mut leaf_inclusion_verifier,
            ) 
        }
    });

    // allocate empty
    let proof_state_dst = unsafe {
        MaybeUninit::<
            ProofOutput<
                TREE_CAP_SIZE,
                NUM_COSETS,
                NUM_DELEGATION_CHALLENGES,
                NUM_AUX_BOUNDARY_VALUES,
            >,
        >::uninit()
        .assume_init()
    };
    let proof_input_dst =
        unsafe { MaybeUninit::<ProofPublicInputs<NUM_STATE_ELEMENTS>>::uninit().assume_init() };

    let mut proof_state_dst = WrappedProofOutput::allocate(cs, proof_state_dst);
    let mut proof_input_dst = WrappedProofPublicInputs::allocate(cs, proof_input_dst);

    // verify function
    println!("Start verification");
    crate::verifier::verify(
        cs,
        &mut proof_state_dst,
        &mut proof_input_dst,
        skeleton,
        queries,
    );

    let worker = boojum::worker::Worker::new_with_num_threads(8);

    dbg!(cs.next_available_row());

    drop(cs);
    owned_cs.pad_and_shrink();
    let mut owned_cs = owned_cs.into_assembly::<Global>();
    assert!(owned_cs.check_if_satisfied(&worker));
}

unsafe fn get_prove_parts<I: NonDeterminismSource, V: LeafInclusionVerifier>()
-> (ProofSkeletonInstance, [QueryValuesInstance; NUM_QUERIES]) {
    let mut leaf_inclusion_verifier = V::new();

    let mut skeleton = MaybeUninit::<ProofSkeletonInstance>::uninit().assume_init();
    ProofSkeletonInstance::fill::<I>((&mut skeleton) as *mut _);
    // let skeleton = skeleton.assume_init();

    let mut queries = MaybeUninit::<[QueryValuesInstance; NUM_QUERIES]>::uninit().assume_init();
    QueryValuesInstance::fill_array::<I, V, NUM_QUERIES>(
        (&mut queries) as *mut _,
        &skeleton,
        &mut leaf_inclusion_verifier,
    );

    (skeleton, queries)
}
