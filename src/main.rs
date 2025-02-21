#![feature(allocator_api)]
#![feature(array_chunks)]
#![feature(generic_arg_infer)]

use boojum::algebraic_props::round_function::AbsorptionModeOverwrite;
use boojum::algebraic_props::sponge::GoldilocksPoseidon2Sponge;
use boojum::config::CSConfig;
use boojum::cs::gates::{
    ConstantAllocatableCS, ConstantsAllocatorGate, DotProductGate,
    FmaGateInBaseFieldWithoutConstant, NopGate, ReductionGate, SelectionGate,
    U32TriAddCarryAsChunkGate, UIntXAddGate, ZeroCheckGate,
};
use boojum::cs::implementations::pow::NoPow;
use boojum::cs::implementations::prover::ProofConfig;
use boojum::cs::implementations::transcript::GoldilocksPoisedon2Transcript;
use boojum::cs::traits::cs::ConstraintSystem;
use boojum::cs::traits::gate::GatePlacementStrategy;
use boojum::cs::{CSGeometry, LookupParameters};
use boojum::dag::CircuitResolverOpts;
use boojum::field::goldilocks::{GoldilocksExt2, GoldilocksField};
use boojum::gadgets::tables::{
    create_byte_split_table, create_range_check_15_bits_table, create_range_check_16_bits_table,
    create_xor8_table, ByteSplitTable, RangeCheck15BitsTable, RangeCheck16BitsTable, Xor8Table,
};
use boojum::gadgets::traits::allocatable::CSAllocatable;
use boojum::gadgets::traits::witnessable::{CSWitnessable, WitnessHookable};
use boojum::worker::Worker;
use serde::Deserialize;
use std::fs::File;
use std::io::BufReader;
use std::mem::MaybeUninit;
use verifier::prover_structs::{
    WrappedProofOutput, WrappedProofPublicInputs, WrappedProofSkeleton, WrappedQueryValues,
};
use zkos_verifier::blake2s_u32::BLAKE2S_DIGEST_SIZE_U32_WORDS;
use zkos_verifier::concrete::size_constants::{
    NUM_AUX_BOUNDARY_VALUES, NUM_COSETS, NUM_DELEGATION_CHALLENGES, NUM_OPENINGS_AT_Z, NUM_QUERIES,
    NUM_STATE_ELEMENTS, TREE_CAP_SIZE,
};
use zkos_verifier::concrete::skeleton_instance::{ProofSkeletonInstance, QueryValuesInstance};
use zkos_verifier::field::{Mersenne31Field, Mersenne31Quartic};
use zkos_verifier::prover::definitions::{ExternalValues, LeafInclusionVerifier, MerkleTreeCap};
use zkos_verifier::prover::merkle_trees::MerkleTreeCapVarLength;
use zkos_verifier::prover::nd_source_std;
use zkos_verifier::prover::prover_stages::Proof;
use zkos_verifier::verifier_common::non_determinism_source::NonDeterminismSource;
use zkos_verifier::verifier_common::proof_flattener::{flatten_proof_for_skeleton, flatten_query};

use zkos_verifier::verifier_common::{
    DefaultLeafInclusionVerifier, DefaultNonDeterminismSource, ProofOutput,
};
use zkos_verifier::ProofPublicInputs;

mod verifier;

#[derive(Deserialize, Debug)]
struct MyStruct {
    pub public_inputs: Vec<u64>,
}
/*
#[derive(Clone, Debug, Hash, serde::Serialize, serde::Deserialize)]
pub struct Proof {
    pub external_values: ExternalValues,
    pub public_inputs: Vec<Mersenne31Field>,
    pub witness_tree_caps: Vec<MerkleTreeCapVarLength>,
    pub memory_tree_caps: Vec<MerkleTreeCapVarLength>,
    pub setup_tree_caps: Vec<MerkleTreeCapVarLength>,
    pub stage_2_tree_caps: Vec<MerkleTreeCapVarLength>,
    pub memory_grand_product_accumulator: Mersenne31Quartic,
    pub delegation_argument_accumulator: Option<Mersenne31Quartic>,
    pub quotient_tree_caps: Vec<MerkleTreeCapVarLength>,
    pub evaluations_at_random_points: Vec<Mersenne31Quartic>,
    pub deep_poly_caps: Vec<MerkleTreeCapVarLength>,
    pub intermediate_fri_oracle_caps: Vec<Vec<MerkleTreeCapVarLength>>,
    pub last_fri_step_plain_leaf_values: Vec<Vec<Mersenne31Quartic>>,
    pub final_monomial_form: Vec<Mersenne31Quartic>,
    pub queries: Vec<QuerySet>,
    pub pow_nonce: u64,
    pub circuit_sequence: u16,
    pub delegation_type: u16,
}*/

#[derive(Clone, Debug, Hash, serde::Serialize, serde::Deserialize)]
pub struct QuerySet {
    pub witness_query: Query,
    pub memory_query: Query,
    pub setup_query: Query,
    pub stage_2_query: Query,
    pub quotient_query: Query,
    pub initial_fri_query: Query,
    pub intermediate_fri_queries: Vec<Query>,
}

#[derive(Clone, Debug, Hash, serde::Serialize, serde::Deserialize)]
pub struct Query {
    pub query_index: u32,
    pub tree_index: u32,
    pub leaf_content: Vec<Mersenne31Field>,
    pub merkle_proof: Vec<[u32; BLAKE2S_DIGEST_SIZE_U32_WORDS]>,
}

type F = boojum::field::goldilocks::GoldilocksField;

fn main() -> Result<(), Box<dyn std::error::Error>> {
    // Open the file in read-only mode with buffer.

    let geometry = CSGeometry {
        // to align with scheduler.
        num_columns_under_copy_permutation: 26 * 5, //60,
        num_witness_columns: 0,
        num_constant_columns: 4,
        max_allowed_constraint_degree: 8, //4,
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
            num_repetitions: 4, //10,
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
    let builder = PublicInputGate::configure_builder(
        builder,
        GatePlacementStrategy::UseGeneralPurposeColumns,
    );
    /*let builder = DotProductGate::<4>::configure_builder(
        builder,
        GatePlacementStrategy::UseGeneralPurposeColumns,
    );*/
    let builder = ZeroCheckGate::configure_builder(
        builder,
        GatePlacementStrategy::UseGeneralPurposeColumns,
        false,
    );
    /*let builder = UIntXAddGate::<16>::configure_builder(
        builder,
        GatePlacementStrategy::UseGeneralPurposeColumns,
    );*/
    let builder = U32TriAddCarryAsChunkGate::configure_builder(
        builder,
        GatePlacementStrategy::UseGeneralPurposeColumns,
    );

    let builder =
        SelectionGate::configure_builder(builder, GatePlacementStrategy::UseGeneralPurposeColumns);
    let builder =
        NopGate::configure_builder(builder, GatePlacementStrategy::UseGeneralPurposeColumns);

    let builder = ReductionGate::<F, 2>::configure_builder(
        builder,
        GatePlacementStrategy::UseGeneralPurposeColumns,
    );

    let mut owned_cs = builder.build(CircuitResolverOpts::new(1 << 23));

    // add tables
    let table = create_range_check_16_bits_table();
    owned_cs.add_lookup_table::<RangeCheck16BitsTable<3>, 3>(table);

    let table = create_range_check_15_bits_table();
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

    // prepare verifier structs
    load_proof_from_file();

    let (skeleton, queries) =
        unsafe { get_prove_parts::<DefaultNonDeterminismSource, DefaultLeafInclusionVerifier>() };

    let skeleton = WrappedProofSkeleton::allocate(cs, skeleton);
    let queries = queries.map(|query| WrappedQueryValues::allocate(cs, query));

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

    let mut proof_state_dst: WrappedProofOutput<GoldilocksField, _, 2, _, 1> =
        WrappedProofOutput::allocate(cs, proof_state_dst);
    let mut proof_input_dst = WrappedProofPublicInputs::allocate(cs, proof_input_dst);

    use boojum::cs::gates::PublicInputGate;
    //let aa = proof_input_dst.input_state_variables[0].as_variables_set()[0];

    //let gate = PublicInputGate::new(proof_input_dst.input_state_variables[0].as_variables_set()[0]);
    //gate.add_to_cs(cs);

    // verify function
    crate::verifier::verify(
        cs,
        &mut proof_state_dst,
        &mut proof_input_dst,
        skeleton,
        queries,
    );

    /*for i in 0..2000 {
        let foo = cs.allocate_constant(GoldilocksField::from_nonreduced_u64(i));
        ZeroCheckGate::check_if_zero(cs, foo);
    }*/

    for i in 1u64..5u64 {
        let aa = cs.allocate_constant(GoldilocksField::from_nonreduced_u64(i));

        let gate = PublicInputGate::new(aa);
        gate.add_to_cs(cs);
    }

    let (final_size, finalization_hint) = cs.pad_and_shrink();
    dbg!(final_size);
    let cs_assembly = owned_cs.into_assembly::<std::alloc::Global>();

    // Now we have a 'CS'
    let worker = Worker::new_with_num_threads(8);

    let proof_config = ProofConfig {
        fri_lde_factor: 2,        //FRI_LDE_FACTOR,
        merkle_tree_cap_size: 16, //BASE_LAYER_CAP_SIZE,
        fri_folding_schedule: None,
        security_level: 100, //SECURITY_BITS_TARGET,
        pow_bits: 0,
    };

    let (setup_base, setup, vk, setup_tree, vars_hint, witness_hints) =
        cs_assembly.get_full_setup::<GoldilocksPoseidon2Sponge<AbsorptionModeOverwrite>>(
            &worker,
            proof_config.fri_lde_factor,
            proof_config.merkle_tree_cap_size,
        );

    let proof: boojum::cs::implementations::proof::Proof<
        GoldilocksField,
        GoldilocksPoseidon2Sponge<AbsorptionModeOverwrite>,
        GoldilocksExt2,
    > = cs_assembly
        .prove_from_precomputations::<GoldilocksExt2, GoldilocksPoisedon2Transcript, GoldilocksPoseidon2Sponge<AbsorptionModeOverwrite>, NoPow>(
            proof_config,
            &setup_base,
            &setup,
            &setup_tree,
            &vk,
            &vars_hint,
            &witness_hints,
            (),
            &worker,
        );

    // Write proof to file
    let proof_file = File::create("proof_output.json")?;
    serde_json::to_writer(proof_file, &proof)?;

    let vk_file = File::create("vk.json")?;
    serde_json::to_writer(vk_file, &vk)?;

    // TODO: verify
    // TODO: write it to file.

    Ok(())
}

pub fn to_cap(input: &Vec<MerkleTreeCapVarLength>) -> Vec<MerkleTreeCap<64>> {
    input
        .iter()
        .map(|x| MerkleTreeCap::<64> {
            cap: x.cap.clone().try_into().unwrap(),
        })
        .collect()
}

pub fn query_set_to_query_values(q: QuerySet) -> QueryValuesInstance {
    QueryValuesInstance {
        query_index: q.initial_fri_query.query_index,
        setup_leaf: q.setup_query.leaf_content.try_into().unwrap(),
        witness_leaf: q.witness_query.leaf_content.try_into().unwrap(),
        memory_leaf: q.memory_query.leaf_content.try_into().unwrap(),
        stage_2_leaf: q.stage_2_query.leaf_content.try_into().unwrap(),
        quotient_leaf: q.quotient_query.leaf_content.try_into().unwrap(),
        // FIXME: incorrect
        fri_oracles_leafs: q.initial_fri_query.leaf_content.try_into().unwrap(),
    }
}

unsafe fn get_prove_parts<I: NonDeterminismSource, V: LeafInclusionVerifier>(
) -> (ProofSkeletonInstance, [QueryValuesInstance; NUM_QUERIES]) {
    // TODO: set non-determinism source iterator

    let mut leaf_inclusion_verifier = V::new();

    let mut skeleton = MaybeUninit::<ProofSkeletonInstance>::uninit().assume_init();
    ProofSkeletonInstance::fill::<I>((&mut skeleton) as *mut _);
    // let skeleton = skeleton.assume_init();

    dbg!(&skeleton.pow_nonce);

    let mut queries = MaybeUninit::<[QueryValuesInstance; NUM_QUERIES]>::uninit().assume_init();
    QueryValuesInstance::fill_array::<I, V, NUM_QUERIES>(
        (&mut queries) as *mut _,
        &skeleton,
        &mut leaf_inclusion_verifier,
    );

    (skeleton, queries)
}

pub fn load_proof_from_file() {
    let file = File::open("examples/proof_old.json").unwrap();
    let reader = BufReader::new(file);

    // Parse the JSON file into the struct.
    let my_struct: Proof = serde_json::from_reader(reader).unwrap();

    println!("NUM : {}", NUM_OPENINGS_AT_Z);

    println!(
        "openings size: {}",
        my_struct.evaluations_at_random_points.len()
    );

    // Print the struct to verify it worked.
    // TODO: use 'flatten_proof_for_skeleton'

    let shuffle_ram_inits_and_teardowns = true;

    //let stream_data = flatten_proof_for_skeleton(&my_struct, apply_shuffle_ram_lazy_init);

    let mut oracle_data = vec![];

    oracle_data.extend(flatten_proof_for_skeleton(
        &my_struct,
        shuffle_ram_inits_and_teardowns,
    ));
    for query in my_struct.queries.iter() {
        oracle_data.extend(flatten_query(query));
    }

    let it = oracle_data.into_iter();

    nd_source_std::set_iterator(it);

    /*let proof = ProofSkeletonInstance {
        _padding: Default::default(),
        circuit_sequence_idx: my_struct.circuit_sequence as u32,
        delegation_type: my_struct.delegation_type as u32,
        public_inputs: my_struct.public_inputs.try_into().unwrap(),
        setup_caps: to_cap(&my_struct.setup_tree_caps).try_into().unwrap(),
        // FIXME
        memory_argument_challenges: Default::default(),
        delegation_argument_challenges: Default::default(),
        aux_boundary_values: Default::default(),
        witness_caps: to_cap(&my_struct.witness_tree_caps).try_into().unwrap(),
        memory_caps: to_cap(&my_struct.memory_tree_caps).try_into().unwrap(),
        stage_2_caps: to_cap(&my_struct.stage_2_tree_caps).try_into().unwrap(),
        memory_grand_product_accumulator: my_struct.memory_grand_product_accumulator,
        delegation_argument_accumulator: [my_struct.delegation_argument_accumulator.unwrap()],
        quotient_caps: to_cap(&my_struct.quotient_tree_caps).try_into().unwrap(),

        openings_at_z: my_struct.evaluations_at_random_points.try_into().unwrap(),
        // FIXME
        openings_at_z_omega: Default::default(),
        fri_intermediate_oracles: my_struct
            .intermediate_fri_oracle_caps
            .iter()
            .map(|x| {
                let tmp: [MerkleTreeCap<64>; 2] = to_cap(x).try_into().unwrap();
                tmp
            })
            .collect::<Vec<_>>()
            .try_into()
            .unwrap(),
        fri_final_step_leafs: my_struct
            .last_fri_step_plain_leaf_values
            .iter()
            .map(|x| x.clone().try_into().unwrap())
            .collect::<Vec<[Mersenne31Quartic; 256]>>()
            .try_into()
            .unwrap(),
        monomial_coeffs: my_struct
            .final_monomial_form
            .try_into()
            .expect("Expected a Vec of length 32"),
        pow_nonce: my_struct.pow_nonce,
    };

    let aa = my_struct
        .queries
        .into_iter()
        .map(|q| query_set_to_query_values(q))
        .collect::<Vec<_>>();*/

    //(proof, aa.try_into().unwrap())
}
