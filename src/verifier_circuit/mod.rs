use boojum::{
    blake2::*,
    config::CSConfig,
    cs::{
        cs_builder::{CsBuilder, CsBuilderImpl},
        cs_builder_reference::CsReferenceImplementationBuilder,
        gates::{
            ConstantsAllocatorGate, DotProductGate, FmaGateInBaseFieldWithoutConstant, NopGate,
            PublicInputGate, ReductionGate, SelectionGate, U32TriAddCarryAsChunkGate, UIntXAddGate,
            ZeroCheckGate,
        },
        implementations::prover::ProofConfig,
        traits::{circuit::CircuitBuilder, cs::ConstraintSystem, gate::GatePlacementStrategy},
        CSGeometry, GateConfigurationHolder, LookupParameters, StaticToolboxHolder,
    },
    dag::CircuitResolverOpts,
    field::SmallField,
    gadgets::{
        blake2s::{blake2s, mixing_function::Word, round_function::Blake2sControl},
        num::Num,
        tables::{
            and8::{create_and8_table, And8Table},
            byte_split::{create_byte_split_table, ByteSplitTable},
            xor8::{create_xor8_table, Xor8Table},
        },
        traits::{allocatable::CSAllocatable, witnessable::WitnessHookable},
        u32::UInt32,
        u8::UInt8,
    },
};
use std::mem::MaybeUninit;

use crate::verifier::prover_structs::WrappedQueryValuesInstance;
use crate::verifier::{
    prover_structs::WrappedProofSkeletonInstance,
    verifier_traits::{CircuitLeafInclusionVerifier, PlaceholderSource},
};
use zkos_verifier::concrete::size_constants::*;
use zkos_verifier::prover::definitions::LeafInclusionVerifier;
use zkos_verifier::{concrete::skeleton_instance::ProofSkeletonInstance, skeleton};

use zkos_verifier::verifier_common::{
    DefaultLeafInclusionVerifier, DefaultNonDeterminismSource, ProofOutput, ProofPublicInputs,
};

use boojum::gadgets::tables::create_range_check_15_bits_table;
use boojum::gadgets::tables::create_range_check_16_bits_table;
use boojum::gadgets::tables::RangeCheck15BitsTable;
use boojum::gadgets::tables::RangeCheck16BitsTable;
use zkos_verifier::prover::prover_stages::Proof;

const NUM_ZKOS_WRAPPER_PUBLIC_INPUTS: usize = 4;

pub struct ZKOSWrapperCircuit<F: SmallField, V: CircuitLeafInclusionVerifier<F>> {
    pub witness: Option<Proof>,
    _phantom: std::marker::PhantomData<(F, V)>,
}

impl<F: SmallField, V: CircuitLeafInclusionVerifier<F>> CircuitBuilder<F>
    for ZKOSWrapperCircuit<F, V>
{
    fn geometry() -> CSGeometry {
        CSGeometry {
            num_columns_under_copy_permutation: 51,
            num_witness_columns: 0,
            num_constant_columns: 4,
            max_allowed_constraint_degree: 4,
        }
    }

    fn lookup_parameters() -> LookupParameters {
        LookupParameters::UseSpecializedColumnsWithTableIdAsConstant {
            width: 3,
            num_repetitions: 17,
            share_table_id: true,
        }
    }

    fn configure_builder<
        T: CsBuilderImpl<F, T>,
        GC: GateConfigurationHolder<F>,
        TB: StaticToolboxHolder,
    >(
        builder: CsBuilder<T, F, GC, TB>,
    ) -> CsBuilder<T, F, impl GateConfigurationHolder<F>, impl StaticToolboxHolder> {
        let builder = builder.allow_lookup(Self::lookup_parameters());

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
        let builder = SelectionGate::configure_builder(
            builder,
            GatePlacementStrategy::UseGeneralPurposeColumns,
        );
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
        let builder = PublicInputGate::configure_builder(
            builder,
            GatePlacementStrategy::UseGeneralPurposeColumns,
        );

        builder
    }
}

impl<F: SmallField, V: CircuitLeafInclusionVerifier<F>> ZKOSWrapperCircuit<F, V> {
    pub fn new(zkos_proof: Option<Proof>, verify_inner_proof: bool) -> Self {
        if verify_inner_proof {
            if let Some(proof) = &zkos_proof {
                verify_zkos_proof::<V::OutOfCircuitImpl>(proof);
            } else {
                panic!("Proof is required for verification");
            }
        }

        Self {
            witness: zkos_proof,
            _phantom: std::marker::PhantomData,
        }
    }

    pub fn size_hint(&self) -> (Option<usize>, Option<usize>) {
        let trace_len = 1 << 20;
        let max_variables = 1 << 25;
        (Some(trace_len), Some(max_variables))
    }

    pub fn add_tables<CS: ConstraintSystem<F>>(&self, cs: &mut CS) {
        let table = create_range_check_16_bits_table::<3, F>();
        cs.add_lookup_table::<RangeCheck16BitsTable<3>, 3>(table);

        let table = create_range_check_15_bits_table::<3, F>();
        cs.add_lookup_table::<RangeCheck15BitsTable<3>, 3>(table);

        let table = create_xor8_table();
        cs.add_lookup_table::<Xor8Table, 3>(table);

        let table = create_and8_table();
        cs.add_lookup_table::<And8Table, 3>(table);

        let table = create_byte_split_table::<F, 1>();
        cs.add_lookup_table::<ByteSplitTable<1>, 3>(table);

        let table = create_byte_split_table::<F, 2>();
        cs.add_lookup_table::<ByteSplitTable<2>, 3>(table);

        let table = create_byte_split_table::<F, 3>();
        cs.add_lookup_table::<ByteSplitTable<3>, 3>(table);

        let table = create_byte_split_table::<F, 4>();
        cs.add_lookup_table::<ByteSplitTable<4>, 3>(table);

        let table = create_byte_split_table::<F, 7>();
        cs.add_lookup_table::<ByteSplitTable<7>, 3>(table);
    }

    pub fn get_proof_config() -> ProofConfig {
        ProofConfig {
            fri_lde_factor: 2,
            merkle_tree_cap_size: 16,
            fri_folding_schedule: None,
            security_level: 100,
            pow_bits: 0,
        }
    }

    pub fn synthesize_into_cs<CS: ConstraintSystem<F>>(&self, cs: &mut CS) {
        let (skeleton, queries) = if let Some(proof) = &self.witness {
            prepare_proof_for_wrapper::<_, _, V>(cs, proof)
        } else {
            // allocate from placeholder
            let skeleton_witness = WrappedProofSkeletonInstance::<F>::placeholder_witness();
            let skeleton = WrappedProofSkeletonInstance::allocate(cs, skeleton_witness);

            let mut leaf_inclusion_verifier = V::new(cs);

            let queries: [_; NUM_QUERIES] = std::array::from_fn(|_idx| unsafe {
                WrappedQueryValuesInstance::from_non_determinism_source::<_, PlaceholderSource, _>(
                    cs,
                    &skeleton,
                    &mut leaf_inclusion_verifier,
                )
            });

            (skeleton, queries)
        };

        let (proof_state_dst, proof_input_dst) = crate::verifier::verify(cs, skeleton, queries);

        // TODO: check proof_state_dest

        let mut flattened_public_input = vec![];
        for el in proof_input_dst.input_state_variables.iter() {
            flattened_public_input.extend_from_slice(&el.into_uint32().decompose_into_bytes(cs));
        }
        for el in proof_input_dst.output_state_variables.iter() {
            flattened_public_input.extend_from_slice(&el.into_uint32().decompose_into_bytes(cs));
        }

        let input_keccak_hash = boojum::gadgets::keccak256::keccak256(cs, &flattened_public_input);
        let input_keccak_hash = [UInt8::zero(cs); 32];
        let take_by = F::CAPACITY_BITS / 8;

        for chunk in input_keccak_hash
            .chunks_exact(take_by)
            .take(NUM_ZKOS_WRAPPER_PUBLIC_INPUTS)
        {
            let mut lc = Vec::with_capacity(chunk.len());
            // treat as BE
            for (idx, el) in chunk.iter().rev().enumerate() {
                lc.push((el.get_variable(), F::SHIFTS[idx * 8]));
            }
            let as_num = Num::linear_combination(cs, &lc);
            use boojum::cs::gates::PublicInputGate;
            let gate = PublicInputGate::new(as_num.get_variable());
            gate.add_to_cs(cs);
        }
    }
}

pub(crate) fn prepare_proof_for_wrapper<
    F: SmallField,
    CS: ConstraintSystem<F>,
    V: CircuitLeafInclusionVerifier<F>,
>(
    cs: &mut CS,
    proof: &Proof,
) -> (
    WrappedProofSkeletonInstance<F>,
    [WrappedQueryValuesInstance<F>; NUM_QUERIES],
) {
    set_iterator_from_proof(proof);

    let skeleton = unsafe {
        WrappedProofSkeletonInstance::from_non_determinism_source::<_, DefaultNonDeterminismSource>(
            cs,
        )
    };

    let mut leaf_inclusion_verifier = V::new(cs);

    let queries: [_; NUM_QUERIES] = std::array::from_fn(|_idx| unsafe {
        WrappedQueryValuesInstance::from_non_determinism_source::<_, DefaultNonDeterminismSource, _>(
            cs,
            &skeleton,
            &mut leaf_inclusion_verifier,
        )
    });

    (skeleton, queries)
}

pub(crate) fn verify_zkos_proof<V: LeafInclusionVerifier>(
    proof: &Proof,
) -> (
    ProofOutput<TREE_CAP_SIZE, NUM_COSETS, NUM_DELEGATION_CHALLENGES, NUM_AUX_BOUNDARY_VALUES>,
    ProofPublicInputs<NUM_STATE_ELEMENTS>,
) {
    set_iterator_from_proof(proof);

    let mut proof_state_dst = unsafe {
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
    let mut proof_input_dst =
        unsafe { MaybeUninit::<ProofPublicInputs<NUM_STATE_ELEMENTS>>::uninit().assume_init() };

    unsafe {
        zkos_verifier::verify_with_configuration::<DefaultNonDeterminismSource, V>(
            &mut proof_state_dst,
            &mut proof_input_dst,
        );
    }

    (proof_state_dst, proof_input_dst)
}

pub(crate) fn set_iterator_from_proof(proof: &Proof) {
    let mut oracle_data = vec![];

    let shuffle_ram_inits_and_teardowns = true;

    oracle_data.extend(
        zkos_verifier::verifier_common::proof_flattener::flatten_proof_for_skeleton(
            &proof,
            shuffle_ram_inits_and_teardowns,
        ),
    );
    for query in proof.queries.iter() {
        oracle_data.extend(zkos_verifier::verifier_common::proof_flattener::flatten_query(query));
    }

    let it = oracle_data.into_iter();

    zkos_verifier::prover::nd_source_std::set_iterator(it.clone());
}
