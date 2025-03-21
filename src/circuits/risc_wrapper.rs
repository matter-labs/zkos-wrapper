use boojum::{
    cs::{
        CSGeometry, GateConfigurationHolder, LookupParameters, StaticToolboxHolder,
        cs_builder::{CsBuilder, CsBuilderImpl},
        cs_builder_reference::CsReferenceImplementationBuilder,
        gates::{
            ConstantsAllocatorGate, DotProductGate, FmaGateInBaseFieldWithoutConstant, NopGate,
            PublicInputGate, ReductionGate, SelectionGate, U32AddCarryAsChunkGate,
            U32TriAddCarryAsChunkGate, UIntXAddGate, ZeroCheckGate, public_input,
        },
        implementations::prover::ProofConfig,
        traits::{circuit::CircuitBuilder, cs::ConstraintSystem, gate::GatePlacementStrategy},
    },
    dag::CircuitResolverOpts,
    field::SmallField,
    gadgets::{
        blake2s::{blake2s, mixing_function::Word, round_function::Blake2sControl},
        mersenne_field::{
            MersenneField, extension_trait::CircuitFieldExpression, fourth_ext::MersenneQuartic,
        },
        num::Num,
        tables::{
            and8::{And8Table, create_and8_table},
            byte_split::{ByteSplitTable, create_byte_split_table},
            xor8::{Xor8Table, create_xor8_table},
        },
        traits::{allocatable::CSAllocatable, witnessable::WitnessHookable},
        u8::UInt8,
        u16::UInt16,
        u32::UInt32,
    },
};
use std::mem::MaybeUninit;

use crate::wrapper_inner_verifier::skeleton::{
    WrappedProofSkeletonInstance, WrappedQueryValuesInstance,
};
use crate::wrapper_inner_verifier::*;
use crate::wrapper_utils::prover_structs::*;
use crate::wrapper_utils::verifier_traits::{CircuitLeafInclusionVerifier, PlaceholderSource};
use risc_verifier::blake2s_u32::*;
use risc_verifier::concrete::size_constants::*;
use risc_verifier::prover::definitions::LeafInclusionVerifier;
use risc_verifier::prover::field::Mersenne31Field;
use risc_verifier::prover::risc_v_simulator::cycle::state::NUM_REGISTERS;
use risc_verifier::{concrete::skeleton_instance::ProofSkeletonInstance, skeleton};

use risc_verifier::prover::cs::definitions::*;

use risc_verifier::verifier_common::{
    DefaultLeafInclusionVerifier, DefaultNonDeterminismSource, ProofOutput, ProofPublicInputs,
};

use boojum::gadgets::tables::RangeCheck15BitsTable;
use boojum::gadgets::tables::RangeCheck16BitsTable;
use boojum::gadgets::tables::create_range_check_15_bits_table;
use boojum::gadgets::tables::create_range_check_16_bits_table;
use risc_verifier::prover::prover_stages::Proof as RiscProof;

const NUM_RISC_WRAPPER_PUBLIC_INPUTS: usize = 4;

pub struct RiscWrapperWitness {
    pub final_registers_state: [u32; NUM_REGISTERS * 2],
    pub proof: RiscProof,
    pub preimage: [u32; BLAKE2S_DIGEST_SIZE_U32_WORDS * 2],
}

pub struct RiscWrapperCircuit<F: SmallField, V: CircuitLeafInclusionVerifier<F>> {
    pub witness: Option<RiscWrapperWitness>,
    _phantom: std::marker::PhantomData<(F, V)>,
}

impl<F: SmallField, V: CircuitLeafInclusionVerifier<F>> CircuitBuilder<F>
    for RiscWrapperCircuit<F, V>
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
            num_repetitions: 21,
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
        let builder = SelectionGate::configure_builder(
            builder,
            GatePlacementStrategy::UseGeneralPurposeColumns,
        );
        let builder = U32TriAddCarryAsChunkGate::configure_builder(
            builder,
            GatePlacementStrategy::UseGeneralPurposeColumns,
        );
        let builder = U32AddCarryAsChunkGate::configure_builder(
            builder,
            GatePlacementStrategy::UseGeneralPurposeColumns,
        );

        builder
    }
}

impl<F: SmallField, V: CircuitLeafInclusionVerifier<F>> RiscWrapperCircuit<F, V> {
    pub fn new(witness: Option<RiscWrapperWitness>, verify_inner_proof: bool) -> Self {
        if verify_inner_proof {
            if let Some(witness) = &witness {
                verify_risc_proof::<V::OutOfCircuitImpl>(&witness.proof);
            } else {
                panic!("Proof is required for verification");
            }
        }

        Self {
            witness,
            _phantom: std::marker::PhantomData,
        }
    }

    pub fn size_hint(&self) -> (Option<usize>, Option<usize>) {
        let trace_len = 1 << 20;
        let max_variables = 1 << 26;
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
        let (final_registers_state_witness, preimage_witness) = if let Some(witness) = &self.witness
        {
            (witness.final_registers_state, witness.preimage)
        } else {
            (
                [0u32; NUM_REGISTERS * 2],
                [0u32; BLAKE2S_DIGEST_SIZE_U32_WORDS * 2],
            )
        };

        let (final_registers_state, preimage) = (
            <[UInt32<F>; NUM_REGISTERS * 2]>::allocate(cs, final_registers_state_witness),
            <[UInt32<F>; BLAKE2S_DIGEST_SIZE_U32_WORDS * 2]>::allocate(cs, preimage_witness),
        );

        let (skeleton, queries) = if let Some(witness) = &self.witness {
            prepare_proof_for_wrapper::<_, _, V>(cs, &witness.proof)
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

        let (proof_state, proof_input) =
            crate::wrapper_inner_verifier::verify(cs, skeleton, queries);

        check_proof_state(
            cs,
            final_registers_state,
            &proof_state,
            &proof_input,
            preimage,
        );

        let mut flattened_public_input = vec![];
        for el in proof_input.input_state_variables.iter() {
            flattened_public_input.extend_from_slice(&el.into_uint32().decompose_into_bytes(cs));
        }
        for el in proof_input.output_state_variables.iter() {
            flattened_public_input.extend_from_slice(&el.into_uint32().decompose_into_bytes(cs));
        }

        let input_keccak_hash = boojum::gadgets::keccak256::keccak256(cs, &flattened_public_input);
        let take_by = F::CAPACITY_BITS / 8;

        for chunk in input_keccak_hash
            .chunks_exact(take_by)
            .take(NUM_RISC_WRAPPER_PUBLIC_INPUTS)
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
    proof: &RiscProof,
) -> (
    WrappedProofSkeletonInstance<F>,
    [WrappedQueryValuesInstance<F>; NUM_QUERIES],
) {
    set_iterator_from_proof(proof, true);

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

pub(crate) fn verify_risc_proof<V: LeafInclusionVerifier>(
    proof: &RiscProof,
) -> (
    ProofOutput<TREE_CAP_SIZE, NUM_COSETS, NUM_DELEGATION_CHALLENGES, NUM_AUX_BOUNDARY_VALUES>,
    ProofPublicInputs<NUM_STATE_ELEMENTS>,
) {
    set_iterator_from_proof(proof, true);

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
        risc_verifier::verify_with_configuration::<DefaultNonDeterminismSource, V>(
            &mut proof_state_dst,
            &mut proof_input_dst,
        );
    }

    (proof_state_dst, proof_input_dst)
}

pub(crate) fn set_iterator_from_proof(proof: &RiscProof, shuffle_ram_inits_and_teardowns: bool) {
    let mut oracle_data = vec![];

    oracle_data.extend(
        risc_verifier::verifier_common::proof_flattener::flatten_proof_for_skeleton(
            &proof,
            shuffle_ram_inits_and_teardowns,
        ),
    );
    let idx = oracle_data.len();
    dbg!(oracle_data.len());
    for query in proof.queries.iter() {
        oracle_data.extend(risc_verifier::verifier_common::proof_flattener::flatten_query(query));
    }
    dbg!(oracle_data.len(), &oracle_data[idx]);

    let it = oracle_data.into_iter();

    risc_verifier::prover::nd_source_std::set_iterator(it.clone());
}

pub(crate) fn check_proof_state<F: SmallField, CS: ConstraintSystem<F>>(
    cs: &mut CS,
    final_registers_state: [UInt32<F>; NUM_REGISTERS * 2],
    proof_state: &WrappedProofOutput<
        F,
        TREE_CAP_SIZE,
        NUM_COSETS,
        NUM_DELEGATION_CHALLENGES,
        NUM_AUX_BOUNDARY_VALUES,
    >,
    public_input: &WrappedProofPublicInputs<F, NUM_STATE_ELEMENTS>,
    preimage: [UInt32<F>; BLAKE2S_DIGEST_SIZE_U32_WORDS * 2],
) {
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

    let mut transcript = Blake2sWrappedBufferingTranscript::new(cs);

    // x0 is always 0, for sanity
    let zero = Num::zero(cs);
    Num::enforce_equal(cs, &final_registers_state[0].into_num(), &zero);

    transcript.absorb(cs, &final_registers_state);

    let mut memory_grand_product_accumulator = MersenneQuartic::one(cs);
    let mut delegation_set_accumulator = MersenneQuartic::zero(cs);

    Num::enforce_equal(cs, &proof_state.circuit_sequence.into_num(), &zero);
    Num::enforce_equal(cs, &proof_state.delegation_type.into_num(), &zero);

    for cap in proof_state.setup_caps.iter() {
        transcript.absorb(cs, &cap.to_slice());
    }

    for cap in proof_state.memory_caps.iter() {
        transcript.absorb(cs, &cap.to_slice());
    }

    for pc_chunk in public_input.input_state_variables.iter() {
        Num::enforce_equal(cs, &pc_chunk.into_num(), &zero);
    }

    let [end_pc_low, end_pc_hi] = public_input
        .output_state_variables
        .map(|chunk| UInt16::from_variable_checked(cs, chunk.get_variable()).into_num());
    let shift = Num::allocate_constant(cs, F::from_u64_unchecked(1u64 << 16));
    let mut end_pc = end_pc_hi.mul(cs, &shift);
    end_pc = end_pc.add(cs, &end_pc_low);
    let end_pc = unsafe { UInt32::<F>::from_variable_unchecked(end_pc.get_variable()) };

    memory_grand_product_accumulator =
        memory_grand_product_accumulator.mul(cs, &proof_state.memory_grand_product_accumulator);
    if NUM_DELEGATION_CHALLENGES > 0 {
        delegation_set_accumulator =
            delegation_set_accumulator.add(cs, &proof_state.delegation_argument_accumulator[0]);
    }

    let memory_seed = transcript.finalize_reset(cs);

    let (memory_argument_challenges, delegation_argument_challenges) =
        draw_memory_and_delegation_challenges_from_transcript_seed(
            cs,
            memory_seed,
            NUM_DELEGATION_CHALLENGES > 0,
        );

    dbg!(
        proof_state
            .memory_challenges
            .memory_argument_linearization_challenges
            .witness_hook(cs)(),
        proof_state
            .memory_challenges
            .memory_argument_gamma
            .witness_hook(cs)(),
        memory_argument_challenges
            .memory_argument_linearization_challenges
            .witness_hook(cs)(),
        memory_argument_challenges
            .memory_argument_gamma
            .witness_hook(cs)(),
    );

    memory_argument_challenges.enforce_equal(cs, &proof_state.memory_challenges);
    if NUM_DELEGATION_CHALLENGES > 0 {
        delegation_argument_challenges
            .unwrap()
            .enforce_equal(cs, &proof_state.delegation_challenges[0]);
    }

    // conclude that our memory argument is valid
    let register_contribution = produce_register_contribution_into_memory_accumulator_raw(
        cs,
        &final_registers_state,
        proof_state
            .memory_challenges
            .memory_argument_linearization_challenges,
        proof_state.memory_challenges.memory_argument_gamma,
    );
    memory_grand_product_accumulator =
        memory_grand_product_accumulator.mul(cs, &register_contribution);

    let one_m4 = MersenneQuartic::one(cs);
    let zero_m4 = MersenneQuartic::zero(cs);
    memory_grand_product_accumulator.enforce_equal(cs, &one_m4);
    delegation_set_accumulator.enforce_equal(cs, &zero_m4);

    // Now we only need to reason about "which program do we execute", and "did it finish succesfully or not".

    // the final piece is to make sure that we ended on the PC that is "expected" (basically - loops to itself, and at the right place),
    // so the program ended logical execution and we can conclude that the set of register values is meaningful

    let mut result_hasher = Blake2sWrappedBufferingTranscript::new(cs);
    result_hasher.absorb(cs, &[end_pc]);
    for cap in proof_state.setup_caps.iter() {
        transcript.absorb(cs, &cap.to_slice());
    }
    let end_params_output = result_hasher.finalize_reset(cs);

    // we require that 8 registers (18 - 25) are some hash output in nature, that encodes our
    // chain of executed programs

    result_hasher.absorb(cs, &preimage);
    let preimage_hash = result_hasher.finalize_reset(cs);

    for i in 0..8 {
        let aux_register_idx = (i + 18) * 2;
        let aux_register = final_registers_state[aux_register_idx];
        let hash_word = UInt32::from_le_bytes(cs, preimage_hash.0[i].inner);
        // Num::enforce_equal(cs, &hash_word.into_num(), &aux_register.into_num());
    }

    for i in 0..8 {
        let end_params_word = UInt32::from_le_bytes(cs, end_params_output.0[i].inner);
        // Num::enforce_equal(cs, &preimage[i + 8].into_num(), &end_params_word.into_num());
    }
}

pub fn produce_register_contribution_into_memory_accumulator_raw<
    F: SmallField,
    CS: ConstraintSystem<F>,
>(
    cs: &mut CS,
    register_final_data: &[UInt32<F>; NUM_REGISTERS * 2],
    memory_argument_linearization_challenges: [MersenneQuartic<F>;
        NUM_MEM_ARGUMENT_LINEARIZATION_CHALLENGES],
    memory_argument_gamma: MersenneQuartic<F>,
) -> MersenneQuartic<F> {
    let mut write_set_contribution = MersenneQuartic::one(cs);
    // all registers are write 0 at timestamp 0
    for reg_idx in 0..NUM_REGISTERS {
        let mut contribution = MersenneQuartic::one(cs); // is_register == 1, without challenge
        let mut t =
            memory_argument_linearization_challenges[MEM_ARGUMENT_CHALLENGE_POWERS_ADDRESS_LOW_IDX];
        let idx_allocated = MersenneField::allocate_constant(cs, Mersenne31Field(reg_idx as u32));
        t = t.mul_by_base(cs, &idx_allocated);
        contribution = contribution.add(cs, &t);
        contribution = contribution.add(cs, &memory_argument_gamma);
        write_set_contribution = write_set_contribution.mul(cs, &contribution);
    }

    let mut read_set_contribution = MersenneQuartic::one(cs);
    // all registers are write 0 at timestamp 0
    for (reg_idx, value_and_timestamp) in register_final_data.chunks(2).enumerate() {
        let [value_low, value_high] = split_uint32_into_pair_mersenne(cs, &value_and_timestamp[0]);
        let [timestamp_low, timestamp_high] =
            split_uint32_into_pair_mersenne(cs, &value_and_timestamp[1]);

        let mut contribution = MersenneQuartic::one(cs); // is_register == 1, without challenge
        let mut t =
            memory_argument_linearization_challenges[MEM_ARGUMENT_CHALLENGE_POWERS_ADDRESS_LOW_IDX];
        let idx_allocated = MersenneField::allocate_constant(cs, Mersenne31Field(reg_idx as u32));
        t = t.mul_by_base(cs, &idx_allocated);
        contribution = contribution.add(cs, &t);

        let mut t = memory_argument_linearization_challenges
            [MEM_ARGUMENT_CHALLENGE_POWERS_TIMESTAMP_LOW_IDX];
        t = t.mul_by_base(cs, &timestamp_low);
        contribution = contribution.add(cs, &t);

        let mut t = memory_argument_linearization_challenges
            [MEM_ARGUMENT_CHALLENGE_POWERS_TIMESTAMP_HIGH_IDX];
        t = t.mul_by_base(cs, &timestamp_high);
        contribution = contribution.add(cs, &t);

        let mut t =
            memory_argument_linearization_challenges[MEM_ARGUMENT_CHALLENGE_POWERS_VALUE_LOW_IDX];
        t = t.mul_by_base(cs, &value_low);
        contribution = contribution.add(cs, &t);

        let mut t =
            memory_argument_linearization_challenges[MEM_ARGUMENT_CHALLENGE_POWERS_VALUE_HIGH_IDX];
        t = t.mul_by_base(cs, &value_high);
        contribution = contribution.add(cs, &t);

        contribution = contribution.add(cs, &memory_argument_gamma);
        read_set_contribution = read_set_contribution.mul(cs, &contribution);
    }

    let t = read_set_contribution.inverse_or_zero(cs);
    write_set_contribution.mul(cs, &t)
}

fn split_uint32_into_pair_mersenne<F: SmallField, CS: ConstraintSystem<F>>(
    cs: &mut CS,
    uint32_value: &UInt32<F>,
) -> [MersenneField<F>; 2] {
    let bytes = uint32_value.decompose_into_bytes(cs);
    let chunks = [
        UInt16::from_le_bytes(cs, [bytes[0], bytes[1]]),
        UInt16::from_le_bytes(cs, [bytes[2], bytes[3]]),
    ];

    chunks.map(|chunk| MersenneField::from_variable_checked(cs, chunk.get_variable(), true))
}

pub fn draw_memory_and_delegation_challenges_from_transcript_seed<
    F: SmallField,
    CS: ConstraintSystem<F>,
>(
    cs: &mut CS,
    mut seed: SeedWrapped<F>,
    produce_delegation_challenge: bool,
) -> (
    WrappedExternalMemoryArgumentChallenges<F>,
    Option<WrappedExternalDelegationArgumentChallenges<F>>,
) {
    unsafe {
        if produce_delegation_challenge == false {
            let mut transcript_challenges = [UInt32::zero(cs);
                ((NUM_MEM_ARGUMENT_LINEARIZATION_CHALLENGES + 1) * 4)
                    .next_multiple_of(BLAKE2S_DIGEST_SIZE_U32_WORDS)];
            Blake2sWrappedTranscript::draw_randomness(cs, &mut seed, &mut transcript_challenges);

            let mut it = transcript_challenges.array_chunks::<4>();
            let memory_argument_linearization_challenges: [MersenneQuartic<F>;
                NUM_MEM_ARGUMENT_LINEARIZATION_CHALLENGES] = core::array::from_fn(|_| {
                MersenneQuartic::from_coeffs(
                    it.next()
                        .unwrap_unchecked()
                        .map(|el| MersenneField::from_uint32_with_reduction(cs, el)),
                )
            });
            let memory_argument_gamma = MersenneQuartic::from_coeffs(
                it.next()
                    .unwrap_unchecked()
                    .map(|el| MersenneField::from_uint32_with_reduction(cs, el)),
            );

            let memory_argument = WrappedExternalMemoryArgumentChallenges {
                memory_argument_linearization_challenges,
                memory_argument_gamma,
            };

            (memory_argument, None)
        } else {
            let mut transcript_challenges = [UInt32::zero(cs);
                ((NUM_MEM_ARGUMENT_LINEARIZATION_CHALLENGES
                    + 1
                    + NUM_DELEGATION_ARGUMENT_LINEARIZATION_CHALLENGES
                    + 1)
                    * 4)
                .next_multiple_of(BLAKE2S_DIGEST_SIZE_U32_WORDS)];
            Blake2sWrappedTranscript::draw_randomness(cs, &mut seed, &mut transcript_challenges);

            let mut it = transcript_challenges.array_chunks::<4>();
            let memory_argument_linearization_challenges: [MersenneQuartic<F>;
                NUM_MEM_ARGUMENT_LINEARIZATION_CHALLENGES] = core::array::from_fn(|_| {
                MersenneQuartic::from_coeffs(
                    it.next()
                        .unwrap_unchecked()
                        .map(|el| MersenneField::from_uint32_with_reduction(cs, el)),
                )
            });
            let memory_argument_gamma = MersenneQuartic::from_coeffs(
                it.next()
                    .unwrap_unchecked()
                    .map(|el| MersenneField::from_uint32_with_reduction(cs, el)),
            );

            let delegation_argument_linearization_challenges: [MersenneQuartic<F>;
                NUM_DELEGATION_ARGUMENT_LINEARIZATION_CHALLENGES] = core::array::from_fn(|_| {
                MersenneQuartic::from_coeffs(
                    it.next()
                        .unwrap_unchecked()
                        .map(|el| MersenneField::from_uint32_with_reduction(cs, el)),
                )
            });
            let delegation_argument_gamma = MersenneQuartic::from_coeffs(
                it.next()
                    .unwrap_unchecked()
                    .map(|el| MersenneField::from_uint32_with_reduction(cs, el)),
            );

            let memory_argument = WrappedExternalMemoryArgumentChallenges {
                memory_argument_linearization_challenges,
                memory_argument_gamma,
            };

            let delegation_argument = WrappedExternalDelegationArgumentChallenges {
                delegation_argument_linearization_challenges,
                delegation_argument_gamma,
            };

            (memory_argument, Some(delegation_argument))
        }
    }
}
