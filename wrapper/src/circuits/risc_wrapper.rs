use circuit_mersenne_field::{
    MersenneField, MersenneQuartic, extension_trait::CircuitFieldExpression,
};
use shivini::boojum::{
    cs::{
        CSGeometry, GateConfigurationHolder, LookupParameters, StaticToolboxHolder,
        cs_builder::{CsBuilder, CsBuilderImpl},
        gates::{
            ConstantsAllocatorGate, FmaGateInBaseFieldWithoutConstant, NopGate, PublicInputGate,
            ReductionGate, SelectionGate, U32AddCarryAsChunkGate, U32TriAddCarryAsChunkGate,
            UIntXAddGate, ZeroCheckGate,
        },
        implementations::prover::ProofConfig,
        traits::{circuit::CircuitBuilder, cs::ConstraintSystem, gate::GatePlacementStrategy},
    },
    field::SmallField,
    gadgets::{
        num::Num,
        tables::{
            byte_split::{ByteSplitTable, create_byte_split_table},
            xor8::{Xor8Table, create_xor8_table},
        },
        traits::allocatable::CSAllocatable,
        u16::UInt16,
        u32::UInt32,
    },
};
use std::mem::MaybeUninit;

use crate::risc_verifier;
use crate::wrapper_inner_verifier::*;
use crate::wrapper_inner_verifier::{
    imports::{FINAL_RISC_CIRCUIT_AUX_REGISTERS_VALUES, FINAL_RISC_CIRCUIT_END_PARAMS},
    skeleton::{WrappedProofSkeletonInstance, WrappedQueryValuesInstance},
};
use crate::wrapper_utils::prover_structs::*;
use crate::wrapper_utils::verifier_traits::{CircuitLeafInclusionVerifier, PlaceholderSource};
use risc_verifier::blake2s_u32::*;
use risc_verifier::concrete::size_constants::*;
use risc_verifier::prover::definitions::LeafInclusionVerifier;
use risc_verifier::prover::field::Mersenne31Field;
use risc_verifier::prover::risc_v_simulator::cycle::state::NUM_REGISTERS;

use risc_verifier::prover::cs::definitions::*;

use risc_verifier::verifier_common::{
    DefaultNonDeterminismSource, ProofOutput, ProofPublicInputs,
    transcript::Blake2sBufferingTranscript,
};

use risc_verifier::prover::prover_stages::Proof as RiscProof;
use shivini::boojum::gadgets::tables::RangeCheck15BitsTable;
use shivini::boojum::gadgets::tables::RangeCheck16BitsTable;
use shivini::boojum::gadgets::tables::create_range_check_15_bits_table;
use shivini::boojum::gadgets::tables::create_range_check_16_bits_table;

const NUM_RISC_WRAPPER_PUBLIC_INPUTS: usize = 4;

pub struct RiscWrapperWitness {
    pub final_registers_state: [u32; NUM_REGISTERS * 3],
    pub proof: RiscProof,
    #[cfg(feature = "wrap_with_reduced_log_23")]
    pub blake_proof: RiscProof,
}

#[derive(Clone, Debug)]
pub struct BinaryCommitment {
    pub end_params: [u32; 8],
    pub aux_params: [u32; 8],
}

impl BinaryCommitment {
    // Uses the binary information that was provided by wrapper generator.
    // In future, this will be the 'active' boojumos binary, but for now this is an example fibonacci code.
    pub fn from_default_binary() -> Self {
        Self {
            end_params: FINAL_RISC_CIRCUIT_END_PARAMS,
            aux_params: FINAL_RISC_CIRCUIT_AUX_REGISTERS_VALUES,
        }
    }
}

impl RiscWrapperWitness {
    pub fn from_full_proof(
        full_proof: execution_utils::ProgramProof,
        binary_commitment: &BinaryCommitment,
    ) -> Self {
        let execution_utils::ProgramProof {
            base_layer_proofs,
            delegation_proofs,
            register_final_values,
            end_params,
            recursion_chain_preimage,
            recursion_chain_hash,
        } = full_proof;

        assert!(base_layer_proofs.len() == 1);
        assert!(register_final_values.len() == NUM_REGISTERS);
        // assert_eq!(end_params, binary_commitment.end_params);

        assert!(recursion_chain_preimage.is_some());
        let mut result_hasher = Blake2sBufferingTranscript::new();
        result_hasher.absorb(&recursion_chain_preimage.unwrap());

        assert!(recursion_chain_hash.is_some());
        assert_eq!(
            recursion_chain_hash.unwrap(),
            result_hasher.finalize_reset().0
        );
        // assert_eq!(recursion_chain_hash.unwrap(), binary_commitment.aux_params);

        let final_registers_state: Vec<_> = register_final_values
            .into_iter()
            .flat_map(|final_values| {
                let (low, high) = risc_verifier::prover::cs::definitions::split_timestamp(
                    final_values.last_access_timestamp,
                );
                [final_values.value, low, high]
            })
            .collect();

        let base_proof = base_layer_proofs.into_iter().next().unwrap();

        let mut dp_iter = delegation_proofs.iter();

        #[cfg(feature = "wrap_with_reduced_log_23")]
        let blake_proof = {
            let blake_proofs = dp_iter.next().unwrap();
            assert!(blake_proofs.1.len() == 1, "Expected only one blake proof");
            blake_proofs.1.into_iter().next().unwrap().clone()
        };

        assert!(dp_iter.next().is_none(), "Too many delegation proofs");

        Self {
            final_registers_state: final_registers_state.try_into().unwrap(),
            proof: base_proof,
            #[cfg(feature = "wrap_with_reduced_log_23")]
            blake_proof,
        }
    }
}

pub struct RiscWrapperCircuit<F: SmallField, V: CircuitLeafInclusionVerifier<F>> {
    pub witness: Option<RiscWrapperWitness>,
    pub binary_commitment: BinaryCommitment,
    _phantom: std::marker::PhantomData<(F, V)>,
}

impl<F: SmallField, V: CircuitLeafInclusionVerifier<F>> CircuitBuilder<F>
    for RiscWrapperCircuit<F, V>
{
    fn geometry() -> CSGeometry {
        #[cfg(feature = "wrap_final_machine")]
        let result = CSGeometry {
            num_columns_under_copy_permutation: 51,
            num_witness_columns: 0,
            num_constant_columns: 4,
            max_allowed_constraint_degree: 4,
        };

        #[cfg(feature = "wrap_with_reduced_log_23")]
        let result = CSGeometry {
            num_columns_under_copy_permutation: 104,
            num_witness_columns: 0,
            num_constant_columns: 4,
            max_allowed_constraint_degree: 4,
        };

        result
    }

    fn lookup_parameters() -> LookupParameters {
        #[cfg(feature = "wrap_final_machine")]
        let result = LookupParameters::UseSpecializedColumnsWithTableIdAsConstant {
            width: 3,
            num_repetitions: 21,
            share_table_id: true,
        };

        #[cfg(feature = "wrap_with_reduced_log_23")]
        let result = LookupParameters::UseSpecializedColumnsWithTableIdAsConstant {
            width: 3,
            num_repetitions: 47,
            share_table_id: true,
        };

        result
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
    pub fn new(
        witness: Option<RiscWrapperWitness>,
        verify_inner_proof: bool,
        binary_commitment: BinaryCommitment,
    ) -> Self {
        if verify_inner_proof {
            if let Some(witness) = &witness {
                verify_risc_proof::<V::OutOfCircuitImpl>(&witness.proof);
                #[cfg(feature = "wrap_with_reduced_log_23")]
                crate::blake2_inner_verifier::verify_blake_proof::<V::OutOfCircuitImpl>(
                    &witness.blake_proof,
                );
            } else {
                panic!("Proof is required for verification");
            }
        }

        Self {
            witness,
            binary_commitment,
            _phantom: std::marker::PhantomData,
        }
    }

    pub fn size_hint(&self) -> (Option<usize>, Option<usize>) {
        let trace_len = 1 << 20;
        #[cfg(feature = "wrap_final_machine")]
        let max_variables = 1 << 26;
        #[cfg(feature = "wrap_with_reduced_log_23")]
        let max_variables = 1 << 27;
        (Some(trace_len), Some(max_variables))
    }

    pub fn add_tables<CS: ConstraintSystem<F>>(&self, cs: &mut CS) {
        let table = create_range_check_16_bits_table::<3, F>();
        cs.add_lookup_table::<RangeCheck16BitsTable<3>, 3>(table);

        let table = create_range_check_15_bits_table::<3, F>();
        cs.add_lookup_table::<RangeCheck15BitsTable<3>, 3>(table);

        let table = create_xor8_table();
        cs.add_lookup_table::<Xor8Table, 3>(table);

        let table = create_byte_split_table::<F, 1>();
        cs.add_lookup_table::<ByteSplitTable<1>, 3>(table);

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
        let final_registers_state_witness = if let Some(witness) = &self.witness {
            witness.final_registers_state
        } else {
            [0u32; NUM_REGISTERS * 3]
        };

        let final_registers_state =
            <[UInt32<F>; NUM_REGISTERS * 3]>::allocate(cs, final_registers_state_witness);

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

        #[cfg(feature = "wrap_with_reduced_log_23")]
        let (skeleton, queries) = if let Some(witness) = &self.witness {
            crate::blake2_inner_verifier::prepare_blake_proof_for_wrapper::<_, _, V>(
                cs,
                &witness.blake_proof,
            )
        } else {
            crate::blake2_inner_verifier::placeholders::<_, _, V>(cs)
        };

        #[cfg(feature = "wrap_with_reduced_log_23")]
        let blake_state = Some(crate::blake2_inner_verifier::verify(cs, skeleton, queries));

        #[cfg(feature = "wrap_final_machine")]
        let blake_state = None;

        check_proof_state(
            cs,
            final_registers_state,
            &proof_state,
            &proof_input,
            &blake_state,
            &self.binary_commitment,
        );

        // we carry registers 10-17 to the next layer - those are the output of the base program
        let output_registers_values: Vec<_> = final_registers_state
            .chunks(3)
            .skip(10)
            .take(8)
            .flat_map(|chunk| chunk[0].decompose_into_bytes(cs))
            .collect();

        let take_by = F::CAPACITY_BITS / 8;
        for chunk in output_registers_values
            .chunks_exact(take_by)
            .take(NUM_RISC_WRAPPER_PUBLIC_INPUTS)
        {
            let mut lc = Vec::with_capacity(chunk.len());
            // treat as LE
            for (idx, el) in chunk.iter().rev().enumerate() {
                lc.push((el.get_variable(), F::SHIFTS[idx * 8]));
            }
            let as_num = Num::linear_combination(cs, &lc);
            use shivini::boojum::cs::gates::PublicInputGate;
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

#[allow(invalid_value)]
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
    for query in proof.queries.iter() {
        oracle_data.extend(risc_verifier::verifier_common::proof_flattener::flatten_query(query));
    }

    let it = oracle_data.into_iter();

    risc_verifier::prover::nd_source_std::set_iterator(it.clone());
}

use crate::blake2_inner_verifier::WrappedBlakeProofOutput;
pub(crate) fn check_proof_state<F: SmallField, CS: ConstraintSystem<F>>(
    cs: &mut CS,
    final_registers_state: [UInt32<F>; NUM_REGISTERS * 3],
    proof_state: &WrappedProofOutput<
        F,
        TREE_CAP_SIZE,
        NUM_COSETS,
        NUM_DELEGATION_CHALLENGES,
        NUM_AUX_BOUNDARY_VALUES,
    >,
    public_input: &WrappedProofPublicInputs<F, NUM_STATE_ELEMENTS>,
    blake_state: &Option<WrappedBlakeProofOutput<F>>,
    binary_commitment: &BinaryCommitment,
) {
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
        for chunk in cap.cap.iter() {
            transcript.absorb(cs, chunk);
        }
    }

    for cap in proof_state.memory_caps.iter() {
        for chunk in cap.cap.iter() {
            transcript.absorb(cs, chunk);
        }
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

    // Now delegation circuit

    if let Some(blake_state) = blake_state {
        let mut buffer = [UInt32::zero(cs); BLAKE2S_BLOCK_SIZE_U32_WORDS];
        buffer[0] = blake_state.delegation_type;
        transcript.absorb(cs, &buffer);

        for cap in blake_state.memory_caps.iter() {
            for chunk in cap.cap.iter() {
                transcript.absorb(cs, chunk);
            }
        }

        memory_grand_product_accumulator =
            memory_grand_product_accumulator.mul(cs, &blake_state.memory_grand_product_accumulator);
        delegation_set_accumulator =
            delegation_set_accumulator.sub(cs, &blake_state.delegation_argument_accumulator[0]);

        blake_state
            .memory_challenges
            .enforce_equal(cs, &proof_state.memory_challenges);
        for (l, r) in blake_state
            .delegation_challenges
            .iter()
            .zip(proof_state.delegation_challenges.iter())
        {
            l.enforce_equal(cs, r);
        }
    }

    let memory_seed = transcript.finalize_reset(cs);

    let (memory_argument_challenges, delegation_argument_challenges) =
        draw_memory_and_delegation_challenges_from_transcript_seed(
            cs,
            memory_seed,
            NUM_DELEGATION_CHALLENGES > 0,
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
        for chunk in cap.cap.iter() {
            result_hasher.absorb(cs, chunk);
        }
    }
    let end_params_output = result_hasher.finalize_reset(cs);

    // We know exactly what program should be executed, so end_params_output should be constant

    for i in 0..8 {
        let end_params_word = UInt32::from_le_bytes(cs, end_params_output.0[i].inner);
        let expected_word = UInt32::allocate_constant(cs, binary_commitment.end_params[i]);
        Num::enforce_equal(cs, &expected_word.into_num(), &end_params_word.into_num());
    }

    // we require that 8 registers (18 - 25) are some hash output in nature, that encodes our
    // chain of executed programs the is also constant

    for i in 0..8 {
        let aux_register_idx = (i + 18) * 3;
        let aux_register = final_registers_state[aux_register_idx];
        let expected_word = UInt32::allocate_constant(cs, binary_commitment.aux_params[i]);
        Num::enforce_equal(cs, &expected_word.into_num(), &aux_register.into_num());
    }
}

pub fn produce_register_contribution_into_memory_accumulator_raw<
    F: SmallField,
    CS: ConstraintSystem<F>,
>(
    cs: &mut CS,
    register_final_data: &[UInt32<F>; NUM_REGISTERS * 3],
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
    for (reg_idx, value_and_timestamp) in register_final_data.chunks(3).enumerate() {
        let [value_low, value_high] = split_uint32_into_pair_mersenne(cs, &value_and_timestamp[0]);
        let timestamp_low =
            MersenneField::from_variable_checked(cs, value_and_timestamp[1].get_variable(), false);
        let timestamp_high =
            MersenneField::from_variable_checked(cs, value_and_timestamp[2].get_variable(), false);

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
