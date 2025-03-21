use boojum::{
    cs::{
        CSGeometry, GateConfigurationHolder, LookupParameters, StaticToolboxHolder,
        cs_builder::{CsBuilder, CsBuilderImpl},
        cs_builder_reference::CsReferenceImplementationBuilder,
        gates::{
            BooleanConstraintGate, ConditionalSwapGate, ConstantsAllocatorGate, DotProductGate,
            FmaGateInBaseFieldWithoutConstant, FmaGateInExtensionWithoutConstant, NopGate,
            ParallelSelectionGate, PublicInputGate, ReductionGate, SelectionGate,
            U32AddCarryAsChunkGate, U32TriAddCarryAsChunkGate, UIntXAddGate, ZeroCheckGate,
            public_input,
        },
        implementations::{
            proof::Proof, prover::ProofConfig, transcript::Transcript, verifier::VerificationKey,
        },
        oracle::TreeHasher,
        traits::{circuit::CircuitBuilder, cs::ConstraintSystem, gate::GatePlacementStrategy},
    },
    dag::CircuitResolverOpts,
    field::{FieldExtension, SmallField},
    gadgets::{
        blake2s::{blake2s, mixing_function::Word, round_function::Blake2sControl},
        boolean::Boolean,
        num::Num,
        recursion::{
            allocated_proof::AllocatedProof, allocated_vk::AllocatedVerificationKey,
            recursive_transcript::CircuitAlgebraicSpongeBasedTranscript,
            recursive_tree_hasher::CircuitGoldilocksPoseidon2Sponge,
        },
        tables::{
            and8::{And8Table, create_and8_table},
            byte_split::{ByteSplitTable, create_byte_split_table},
            xor8::{Xor8Table, create_xor8_table},
        },
        traits::{
            allocatable::CSAllocatable, round_function::BuildableCircuitRoundFunction,
            witnessable::WitnessHookable,
        },
        u8::UInt8,
        u32::UInt32,
    },
    implementations::poseidon2::Poseidon2Goldilocks,
};

use crate::*;

pub struct CompressionCircuit {
    pub witness: Option<RiscWrapperProof>,
    pub vk: RiscWrapperVK,
    // pub transcript_params: <TR as Transcript<GL>>::TransciptParameters,
}

impl CircuitBuilder<GL> for CompressionCircuit {
    fn geometry() -> CSGeometry {
        CSGeometry {
            num_columns_under_copy_permutation: 52,
            num_witness_columns: 78,
            num_constant_columns: 4,
            max_allowed_constraint_degree: 8,
        }
    }

    fn lookup_parameters() -> LookupParameters {
        LookupParameters::NoLookup
    }

    fn configure_builder<
        T: CsBuilderImpl<GL, T>,
        GC: GateConfigurationHolder<GL>,
        TB: StaticToolboxHolder,
    >(
        builder: CsBuilder<T, GL, GC, TB>,
    ) -> CsBuilder<T, GL, impl GateConfigurationHolder<GL>, impl StaticToolboxHolder> {
        let builder = ConstantsAllocatorGate::configure_builder(
            builder,
            GatePlacementStrategy::UseGeneralPurposeColumns,
        );
        let builder = BooleanConstraintGate::configure_builder(
            builder,
            GatePlacementStrategy::UseSpecializedColumns {
                num_repetitions: 1,
                share_constants: false,
            },
        );
        let builder = Poseidon2Goldilocks::configure_builder(
            builder,
            GatePlacementStrategy::UseGeneralPurposeColumns,
        );
        let builder = ZeroCheckGate::configure_builder(
            builder,
            GatePlacementStrategy::UseGeneralPurposeColumns,
            true,
        );
        let builder = FmaGateInBaseFieldWithoutConstant::configure_builder(
            builder,
            GatePlacementStrategy::UseGeneralPurposeColumns,
        );
        let builder = FmaGateInExtensionWithoutConstant::<GL, GLExt2>::configure_builder(
            builder,
            GatePlacementStrategy::UseGeneralPurposeColumns,
        );
        let builder = SelectionGate::configure_builder(
            builder,
            GatePlacementStrategy::UseGeneralPurposeColumns,
        );
        let builder = ParallelSelectionGate::<4>::configure_builder(
            builder,
            GatePlacementStrategy::UseGeneralPurposeColumns,
        );
        let builder = ConditionalSwapGate::<4>::configure_builder(
            builder,
            GatePlacementStrategy::UseGeneralPurposeColumns,
        );
        let builder = PublicInputGate::configure_builder(
            builder,
            GatePlacementStrategy::UseGeneralPurposeColumns,
        );
        let builder = ReductionGate::<_, 4>::configure_builder(
            builder,
            GatePlacementStrategy::UseGeneralPurposeColumns,
        );
        let builder =
            NopGate::configure_builder(builder, GatePlacementStrategy::UseGeneralPurposeColumns);

        builder
    }
}

impl CompressionCircuit {
    pub fn new(
        risc_wrapper_proof: Option<RiscWrapperProof>,
        risc_wrapper_vk: RiscWrapperVK,
        verify_inner_proof: bool,
    ) -> Self {
        if verify_inner_proof {
            if let Some(proof) = &risc_wrapper_proof {
                let is_valid = crate::verify_risc_wrapper_proof(proof, &risc_wrapper_vk);
                assert!(is_valid, "Proof is invalid");
            } else {
                panic!("Proof is required for verification");
            }
        }

        Self {
            witness: risc_wrapper_proof,
            vk: risc_wrapper_vk,
        }
    }

    pub fn size_hint(&self) -> (Option<usize>, Option<usize>) {
        let trace_len = 1 << 16;
        let max_variables = 1 << 24;
        (Some(trace_len), Some(max_variables))
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

    pub fn synthesize_into_cs<CS: ConstraintSystem<GL> + 'static>(self, cs: &mut CS) {
        let Self { witness, vk } = self;

        let verifier_builder =
            RiscWrapperCircuitBuilder::dyn_recursive_verifier_builder::<GLExt2, CS>();

        // use this and deal with borrow checker
        let r = cs as *mut CS;

        assert_eq!(vk.fixed_parameters.parameters, verifier_builder.geometry());

        let fixed_parameters = vk.fixed_parameters.clone();

        let verifier = verifier_builder.create_recursive_verifier(cs);

        let cs = unsafe { &mut *r };

        let vk = AllocatedVerificationKey::allocate_constant(cs, vk);

        let proof_config = RiscWrapper::get_proof_config();

        let proof = AllocatedProof::allocate_from_witness(
            cs,
            witness,
            &verifier,
            &fixed_parameters,
            &proof_config,
        );

        // verify the proof
        let (is_valid, public_inputs) = verifier.verify::<
            CircuitRiscWrapperTreeHasher,
            RiscWrapperTranscript,
            CircuitRiscWrapperTranscript, 
            NoPow
        >(
            cs,
            (),
            &proof,
            &fixed_parameters,
            &proof_config,
            &vk,
        );

        let boolean_true = Boolean::allocated_constant(cs, true);
        Boolean::enforce_equal(cs, &is_valid, &boolean_true);

        assert_eq!(public_inputs.len(), fixed_parameters.num_public_inputs());

        for el in public_inputs.into_iter() {
            use boojum::cs::gates::PublicInputGate;
            let gate = PublicInputGate::new(el.get_variable());
            gate.add_to_cs(cs);
        }
    }
}
