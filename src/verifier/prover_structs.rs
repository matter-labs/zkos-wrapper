use super::*;

pub struct WrappedProofPublicInputs<F: SmallField, const NUM_STATE_ELEMENTS: usize> {
    pub input_state_variables: [MersenneField<F>; NUM_STATE_ELEMENTS],
    pub output_state_variables: [MersenneField<F>; NUM_STATE_ELEMENTS],
}

impl<F: SmallField, const NUM_STATE_ELEMENTS: usize> CSAllocatable<F>
    for WrappedProofPublicInputs<F, NUM_STATE_ELEMENTS>
{
    type Witness = ProofPublicInputs<NUM_STATE_ELEMENTS>;

    fn placeholder_witness() -> Self::Witness {
        ProofPublicInputs {
            input_state_variables: [Mersenne31Field::ZERO; NUM_STATE_ELEMENTS],
            output_state_variables: [Mersenne31Field::ZERO; NUM_STATE_ELEMENTS],
        }
    }
    fn allocate_without_value<CS: ConstraintSystem<F>>(cs: &mut CS) -> Self {
        Self {
            input_state_variables: [(); NUM_STATE_ELEMENTS]
                .map(|_| MersenneField::allocate_without_value(cs)),
            output_state_variables: [(); NUM_STATE_ELEMENTS]
                .map(|_| MersenneField::allocate_without_value(cs)),
        }
    }
    fn allocate<CS: ConstraintSystem<F>>(cs: &mut CS, witness: Self::Witness) -> Self {
        Self {
            input_state_variables: witness
                .input_state_variables
                .map(|x| MersenneField::allocate(cs, x)),
            output_state_variables: witness
                .output_state_variables
                .map(|x| MersenneField::allocate(cs, x)),
        }
    }
}

pub struct WrappedMerkleTreeCap<F: SmallField, const N: usize> {
    pub cap: [[UInt32<F>; DIGEST_SIZE_U32_WORDS]; N],
}

impl<F: SmallField, const N: usize> WrappedMerkleTreeCap<F, N> {
    pub(crate) fn to_slice(&self) -> &[UInt32<F>] {
        let (head, middle, tail) = unsafe { self.cap.align_to::<UInt32<F>>() };
        assert!(head.is_empty() && tail.is_empty());
        middle
    }
}

impl<F: SmallField, const N: usize> CSAllocatable<F> for WrappedMerkleTreeCap<F, N> {
    type Witness = MerkleTreeCap<N>;

    fn placeholder_witness() -> Self::Witness {
        MerkleTreeCap {
            cap: [[0u32; DIGEST_SIZE_U32_WORDS]; N],
        }
    }
    fn allocate_without_value<CS: ConstraintSystem<F>>(cs: &mut CS) -> Self {
        let mut cap = [[UInt32::zero(cs); DIGEST_SIZE_U32_WORDS]; N];
        for i in 0..N {
            for j in 0..DIGEST_SIZE_U32_WORDS {
                cap[i][j] = UInt32::allocate_without_value(cs);
            }
        }

        Self { cap }
    }
    fn allocate<CS: ConstraintSystem<F>>(cs: &mut CS, witness: Self::Witness) -> Self {
        let cap = witness.cap.map(|row| row.map(|x| UInt32::allocate(cs, x)));
        Self { cap }
    }
}

pub struct WrappedExternalMemoryArgumentChallenges<F: SmallField> {
    pub memory_argument_linearization_challenges:
        [MersenneQuartic<F>; NUM_MEM_ARGUMENT_LINEARIZATION_CHALLENGES],
    pub memory_argument_gamma: MersenneQuartic<F>,
}

impl<F: SmallField> WrappedExternalMemoryArgumentChallenges<F> {
    pub(crate) fn to_uint32_vec(&self) -> Vec<UInt32<F>> {
        let mut result = Vec::with_capacity(NUM_MEM_ARGUMENT_LINEARIZATION_CHALLENGES * 4 + 4);
        for i in 0..NUM_MEM_ARGUMENT_LINEARIZATION_CHALLENGES {
            result.extend_from_slice(
                &self.memory_argument_linearization_challenges[i].into_uint32s(),
            );
        }
        result.extend_from_slice(&self.memory_argument_gamma.into_uint32s());
        result
    }
}

impl<F: SmallField> CSAllocatable<F> for WrappedExternalMemoryArgumentChallenges<F> {
    type Witness = ExternalMemoryArgumentChallenges;

    fn placeholder_witness() -> Self::Witness {
        ExternalMemoryArgumentChallenges {
            memory_argument_linearization_challenges: [Mersenne31Quartic::ZERO;
                NUM_MEM_ARGUMENT_LINEARIZATION_CHALLENGES],
            memory_argument_gamma: Mersenne31Quartic::ZERO,
        }
    }
    fn allocate_without_value<CS: ConstraintSystem<F>>(cs: &mut CS) -> Self {
        let memory_argument_linearization_challenges = [();
            NUM_MEM_ARGUMENT_LINEARIZATION_CHALLENGES]
            .map(|_| MersenneQuartic::allocate_without_value(cs));

        Self {
            memory_argument_linearization_challenges,
            memory_argument_gamma: MersenneQuartic::allocate_without_value(cs),
        }
    }
    fn allocate<CS: ConstraintSystem<F>>(cs: &mut CS, witness: Self::Witness) -> Self {
        let memory_argument_linearization_challenges = witness
            .memory_argument_linearization_challenges
            .map(|x| MersenneQuartic::allocate(cs, x));
        let memory_argument_gamma = MersenneQuartic::allocate(cs, witness.memory_argument_gamma);
        Self {
            memory_argument_linearization_challenges,
            memory_argument_gamma,
        }
    }
}

#[derive(Clone, Copy)]
pub struct WrappedExternalDelegationArgumentChallenges<F: SmallField> {
    pub delegation_argument_linearization_challenges:
        [MersenneQuartic<F>; NUM_DELEGATION_ARGUMENT_LINEARIZATION_CHALLENGES],
    pub delegation_argument_gamma: MersenneQuartic<F>,
}

impl<F: SmallField> WrappedExternalDelegationArgumentChallenges<F> {
    pub(crate) fn to_uint32_vec(&self) -> Vec<UInt32<F>> {
        let mut result =
            Vec::with_capacity(NUM_DELEGATION_ARGUMENT_LINEARIZATION_CHALLENGES * 4 + 4);
        for i in 0..NUM_DELEGATION_ARGUMENT_LINEARIZATION_CHALLENGES {
            result.extend_from_slice(
                &self.delegation_argument_linearization_challenges[i].into_uint32s(),
            );
        }
        result.extend_from_slice(&self.delegation_argument_gamma.into_uint32s());
        result
    }
}

impl<F: SmallField> CSAllocatable<F> for WrappedExternalDelegationArgumentChallenges<F> {
    type Witness = ExternalDelegationArgumentChallenges;

    fn placeholder_witness() -> Self::Witness {
        ExternalDelegationArgumentChallenges {
            delegation_argument_linearization_challenges: [Mersenne31Quartic::ZERO;
                NUM_DELEGATION_ARGUMENT_LINEARIZATION_CHALLENGES],
            delegation_argument_gamma: Mersenne31Quartic::ZERO,
        }
    }
    fn allocate_without_value<CS: ConstraintSystem<F>>(cs: &mut CS) -> Self {
        let delegation_argument_linearization_challenges = [();
            NUM_DELEGATION_ARGUMENT_LINEARIZATION_CHALLENGES]
            .map(|_| MersenneQuartic::allocate_without_value(cs));

        Self {
            delegation_argument_linearization_challenges,
            delegation_argument_gamma: MersenneQuartic::allocate_without_value(cs),
        }
    }
    fn allocate<CS: ConstraintSystem<F>>(cs: &mut CS, witness: Self::Witness) -> Self {
        let delegation_argument_linearization_challenges = witness
            .delegation_argument_linearization_challenges
            .map(|x| MersenneQuartic::allocate(cs, x));
        let delegation_argument_gamma =
            MersenneQuartic::allocate(cs, witness.delegation_argument_gamma);
        Self {
            delegation_argument_linearization_challenges,
            delegation_argument_gamma,
        }
    }
}

#[derive(Clone, Copy)]
pub struct WrappedAuxArgumentsBoundaryValues<F: SmallField> {
    pub lazy_init_first_row: [MersenneField<F>; REGISTER_SIZE],
    pub lazy_init_one_before_last_row: [MersenneField<F>; REGISTER_SIZE],
}

impl<F: SmallField> WrappedAuxArgumentsBoundaryValues<F> {
    pub(crate) fn to_uint32_vec(&self) -> Vec<UInt32<F>> {
        let mut result = Vec::with_capacity(REGISTER_SIZE * 2);
        result.extend_from_slice(&self.lazy_init_first_row.map(|x| x.into_uint32()));
        result.extend_from_slice(&self.lazy_init_one_before_last_row.map(|x| x.into_uint32()));
        result
    }
}

impl<F: SmallField> CSAllocatable<F> for WrappedAuxArgumentsBoundaryValues<F> {
    type Witness = AuxArgumentsBoundaryValues;

    fn placeholder_witness() -> Self::Witness {
        AuxArgumentsBoundaryValues {
            lazy_init_first_row: [Mersenne31Field::ZERO; REGISTER_SIZE],
            lazy_init_one_before_last_row: [Mersenne31Field::ZERO; REGISTER_SIZE],
        }
    }
    fn allocate_without_value<CS: ConstraintSystem<F>>(cs: &mut CS) -> Self {
        Self {
            lazy_init_first_row: [(); REGISTER_SIZE]
                .map(|_| MersenneField::allocate_without_value(cs)),
            lazy_init_one_before_last_row: [(); REGISTER_SIZE]
                .map(|_| MersenneField::allocate_without_value(cs)),
        }
    }
    fn allocate<CS: ConstraintSystem<F>>(cs: &mut CS, witness: Self::Witness) -> Self {
        Self {
            lazy_init_first_row: witness
                .lazy_init_first_row
                .map(|x| MersenneField::allocate(cs, x)),
            lazy_init_one_before_last_row: witness
                .lazy_init_one_before_last_row
                .map(|x| MersenneField::allocate(cs, x)),
        }
    }
}

pub struct WrappedProofOutput<
    F: SmallField,
    const CAP_SIZE: usize,
    const NUM_COSETS: usize,
    const NUM_DELEGATION_CHALLENGES: usize,
    const NUM_AUX_BOUNDARY_VALUES: usize,
> {
    pub setup_caps: [WrappedMerkleTreeCap<F, CAP_SIZE>; NUM_COSETS],
    pub memory_caps: [WrappedMerkleTreeCap<F, CAP_SIZE>; NUM_COSETS],
    pub memory_challenges: WrappedExternalMemoryArgumentChallenges<F>,
    pub delegation_challenges:
        [WrappedExternalDelegationArgumentChallenges<F>; NUM_DELEGATION_CHALLENGES],
    pub lazy_init_boundary_values: [WrappedAuxArgumentsBoundaryValues<F>; NUM_AUX_BOUNDARY_VALUES],
    pub memory_grand_product_accumulator: MersenneQuartic<F>,
    pub delegation_argument_accumulator: [MersenneQuartic<F>; NUM_DELEGATION_CHALLENGES],
    pub circuit_sequence: UInt32<F>,
    pub delegation_type: UInt32<F>,
}

impl<
    F: SmallField,
    const CAP_SIZE: usize,
    const NUM_COSETS: usize,
    const NUM_DELEGATION_CHALLENGES: usize,
    const NUM_AUX_BOUNDARY_VALUES: usize,
> CSAllocatable<F>
    for WrappedProofOutput<
        F,
        CAP_SIZE,
        NUM_COSETS,
        NUM_DELEGATION_CHALLENGES,
        NUM_AUX_BOUNDARY_VALUES,
    >
{
    type Witness =
        ProofOutput<CAP_SIZE, NUM_COSETS, NUM_DELEGATION_CHALLENGES, NUM_AUX_BOUNDARY_VALUES>;

    fn placeholder_witness() -> Self::Witness {
        ProofOutput {
            setup_caps: [WrappedMerkleTreeCap::<F, CAP_SIZE>::placeholder_witness(); NUM_COSETS],
            memory_caps: [WrappedMerkleTreeCap::<F, CAP_SIZE>::placeholder_witness(); NUM_COSETS],
            memory_challenges: WrappedExternalMemoryArgumentChallenges::<F>::placeholder_witness(),
            delegation_challenges:
                [WrappedExternalDelegationArgumentChallenges::<F>::placeholder_witness();
                    NUM_DELEGATION_CHALLENGES],
            lazy_init_boundary_values: [WrappedAuxArgumentsBoundaryValues::<F>::placeholder_witness(
            ); NUM_AUX_BOUNDARY_VALUES],
            memory_grand_product_accumulator: Mersenne31Quartic::ZERO,
            delegation_argument_accumulator: [Mersenne31Quartic::ZERO; NUM_DELEGATION_CHALLENGES],
            circuit_sequence: 0u32,
            delegation_type: 0u32,
        }
    }
    fn allocate_without_value<CS: ConstraintSystem<F>>(cs: &mut CS) -> Self {
        Self {
            setup_caps: [(); NUM_COSETS].map(|_| WrappedMerkleTreeCap::allocate_without_value(cs)),
            memory_caps: [(); NUM_COSETS].map(|_| WrappedMerkleTreeCap::allocate_without_value(cs)),
            memory_challenges: WrappedExternalMemoryArgumentChallenges::allocate_without_value(cs),
            delegation_challenges: [(); NUM_DELEGATION_CHALLENGES]
                .map(|_| WrappedExternalDelegationArgumentChallenges::allocate_without_value(cs)),
            lazy_init_boundary_values: [(); NUM_AUX_BOUNDARY_VALUES]
                .map(|_| WrappedAuxArgumentsBoundaryValues::allocate_without_value(cs)),
            memory_grand_product_accumulator: MersenneQuartic::allocate_without_value(cs),
            delegation_argument_accumulator: [(); NUM_DELEGATION_CHALLENGES]
                .map(|_| MersenneQuartic::allocate_without_value(cs)),
            circuit_sequence: UInt32::allocate_without_value(cs),
            delegation_type: UInt32::allocate_without_value(cs),
        }
    }
    fn allocate<CS: ConstraintSystem<F>>(cs: &mut CS, witness: Self::Witness) -> Self {
        Self {
            setup_caps: witness
                .setup_caps
                .map(|x| WrappedMerkleTreeCap::allocate(cs, x)),
            memory_caps: witness
                .memory_caps
                .map(|x| WrappedMerkleTreeCap::allocate(cs, x)),
            memory_challenges: WrappedExternalMemoryArgumentChallenges::allocate(
                cs,
                witness.memory_challenges,
            ),
            delegation_challenges: witness
                .delegation_challenges
                .map(|x| WrappedExternalDelegationArgumentChallenges::allocate(cs, x)),
            lazy_init_boundary_values: witness
                .lazy_init_boundary_values
                .map(|x| WrappedAuxArgumentsBoundaryValues::allocate(cs, x)),
            memory_grand_product_accumulator: MersenneQuartic::allocate(
                cs,
                witness.memory_grand_product_accumulator,
            ),
            delegation_argument_accumulator: witness
                .delegation_argument_accumulator
                .map(|x| MersenneQuartic::allocate(cs, x)),
            circuit_sequence: UInt32::allocate(cs, witness.circuit_sequence),
            delegation_type: UInt32::allocate(cs, witness.delegation_type),
        }
    }
}

// TODO: make SKELETON_PADDING from zkos_verifier public
pub(crate) const SKELETON_PADDING: usize = const {
    let mut size = 0;
    // circuit sequence
    size += core::mem::size_of::<u32>();
    // delegation type
    size += core::mem::size_of::<u32>();
    // variable number of public inputs
    assert!(core::mem::size_of::<u32>() == core::mem::size_of::<Mersenne31Field>());
    size += core::mem::size_of::<Mersenne31Field>() * NUM_PUBLIC_INPUTS_FROM_STATE_ELEMENTS;
    let size_mod_16 = size % 16;
    let required_padding_bytes = (16 - size_mod_16) % 16;
    assert!(required_padding_bytes % core::mem::size_of::<u32>() == 0);

    required_padding_bytes / core::mem::size_of::<u32>()
};

pub(crate) type WrappedProofSkeletonInstance<F> = WrappedProofSkeleton<
    F,
    TREE_CAP_SIZE,
    NUM_COSETS,
    NUM_PUBLIC_INPUTS_FROM_STATE_ELEMENTS,
    NUM_DELEGATION_CHALLENGES,
    NUM_AUX_BOUNDARY_VALUES,
    NUM_PUBLIC_INPUTS_FROM_STATE_ELEMENTS,
    NUM_OPENINGS_AT_Z,
    NUM_OPENINGS_AT_Z_OMEGA,
    NUM_FRI_STEPS_WITH_ORACLES,
    LAST_FRI_STEP_LEAFS_TOTAL_SIZE_PER_COSET,
    FRI_FINAL_DEGREE,
>;

pub struct WrappedProofSkeleton<
    F: SmallField,
    const CAP_SIZE: usize,
    const NUM_COSETS: usize,
    const NUM_PUBLIC_INPUTS: usize,
    const NUM_DELEGATION_CHALLENGES: usize,
    const NUM_AUX_BOUNDARY_VALUES: usize,
    const NUM_PUBLIC_INPUTS_FROM_STATE_ELEMENTS: usize,
    const NUM_OPENINGS_AT_Z: usize,
    const NUM_OPENINGS_AT_Z_OMEGA: usize,
    const NUM_FRI_STEPS_WITH_ORACLES: usize,
    const FINAL_FRI_STEP_LEAF_SIZE_PER_COSET: usize,
    const FRI_FINAL_DEGREE: usize,
> {
    pub(crate) circuit_sequence_idx: UInt32<F>,
    pub(crate) delegation_type: UInt32<F>,
    pub(crate) public_inputs: [MersenneField<F>; NUM_PUBLIC_INPUTS_FROM_STATE_ELEMENTS],
    pub(crate) setup_caps: [WrappedMerkleTreeCap<F, CAP_SIZE>; NUM_COSETS],
    pub(crate) memory_argument_challenges: WrappedExternalMemoryArgumentChallenges<F>,
    pub(crate) delegation_argument_challenges:
        [WrappedExternalDelegationArgumentChallenges<F>; NUM_DELEGATION_CHALLENGES],
    pub(crate) aux_boundary_values: [WrappedAuxArgumentsBoundaryValues<F>; NUM_AUX_BOUNDARY_VALUES],
    pub(crate) witness_caps: [WrappedMerkleTreeCap<F, CAP_SIZE>; NUM_COSETS],
    pub(crate) memory_caps: [WrappedMerkleTreeCap<F, CAP_SIZE>; NUM_COSETS],
    pub(crate) stage_2_caps: [WrappedMerkleTreeCap<F, CAP_SIZE>; NUM_COSETS],
    pub(crate) memory_grand_product_accumulator: MersenneQuartic<F>,
    pub(crate) delegation_argument_accumulator: [MersenneQuartic<F>; NUM_DELEGATION_CHALLENGES],
    pub(crate) quotient_caps: [WrappedMerkleTreeCap<F, CAP_SIZE>; NUM_COSETS],
    pub(crate) openings_at_z: [MersenneQuartic<F>; NUM_OPENINGS_AT_Z],
    pub(crate) openings_at_z_omega: [MersenneQuartic<F>; NUM_OPENINGS_AT_Z_OMEGA],
    pub(crate) fri_intermediate_oracles:
        [[WrappedMerkleTreeCap<F, CAP_SIZE>; NUM_COSETS]; NUM_FRI_STEPS_WITH_ORACLES],
    pub(crate) fri_final_step_leafs:
        [[MersenneQuartic<F>; FINAL_FRI_STEP_LEAF_SIZE_PER_COSET]; NUM_COSETS],
    pub(crate) monomial_coeffs: [MersenneQuartic<F>; FRI_FINAL_DEGREE],
    pub(crate) pow_nonce: [UInt32<F>; 2], // le u64 represented as two u32
}

impl<
    F: SmallField,
    const CAP_SIZE: usize,
    const NUM_COSETS: usize,
    const NUM_PUBLIC_INPUTS: usize,
    const NUM_DELEGATION_CHALLENGES: usize,
    const NUM_AUX_BOUNDARY_VALUES: usize,
    const NUM_PUBLIC_INPUTS_FROM_STATE_ELEMENTS: usize,
    const NUM_OPENINGS_AT_Z: usize,
    const NUM_OPENINGS_AT_Z_OMEGA: usize,
    const NUM_FRI_STEPS_WITH_ORACLES: usize,
    const FINAL_FRI_STEP_LEAF_SIZE_PER_COSET: usize,
    const FRI_FINAL_DEGREE: usize,
> CSAllocatable<F>
    for WrappedProofSkeleton<
        F,
        CAP_SIZE,
        NUM_COSETS,
        NUM_PUBLIC_INPUTS,
        NUM_DELEGATION_CHALLENGES,
        NUM_AUX_BOUNDARY_VALUES,
        NUM_PUBLIC_INPUTS_FROM_STATE_ELEMENTS,
        NUM_OPENINGS_AT_Z,
        NUM_OPENINGS_AT_Z_OMEGA,
        NUM_FRI_STEPS_WITH_ORACLES,
        FINAL_FRI_STEP_LEAF_SIZE_PER_COSET,
        FRI_FINAL_DEGREE,
    >
{
    type Witness = ProofSkeleton<
        SKELETON_PADDING,
        CAP_SIZE,
        NUM_COSETS,
        NUM_PUBLIC_INPUTS,
        NUM_DELEGATION_CHALLENGES,
        NUM_AUX_BOUNDARY_VALUES,
        NUM_PUBLIC_INPUTS_FROM_STATE_ELEMENTS,
        NUM_OPENINGS_AT_Z,
        NUM_OPENINGS_AT_Z_OMEGA,
        NUM_FRI_STEPS_WITH_ORACLES,
        FINAL_FRI_STEP_LEAF_SIZE_PER_COSET,
        FRI_FINAL_DEGREE,
    >;

    fn placeholder_witness() -> Self::Witness {
        unimplemented!()
    }
    fn allocate_without_value<CS: ConstraintSystem<F>>(cs: &mut CS) -> Self {
        Self {
            circuit_sequence_idx: UInt32::allocate_without_value(cs),
            delegation_type: UInt32::allocate_without_value(cs),
            public_inputs: [(); NUM_PUBLIC_INPUTS_FROM_STATE_ELEMENTS]
                .map(|_| MersenneField::allocate_without_value(cs)),
            setup_caps: [(); NUM_COSETS].map(|_| WrappedMerkleTreeCap::allocate_without_value(cs)),
            memory_argument_challenges:
                WrappedExternalMemoryArgumentChallenges::allocate_without_value(cs),
            delegation_argument_challenges: [(); NUM_DELEGATION_CHALLENGES]
                .map(|_| WrappedExternalDelegationArgumentChallenges::allocate_without_value(cs)),
            aux_boundary_values: [(); NUM_AUX_BOUNDARY_VALUES]
                .map(|_| WrappedAuxArgumentsBoundaryValues::allocate_without_value(cs)),
            witness_caps: [(); NUM_COSETS]
                .map(|_| WrappedMerkleTreeCap::allocate_without_value(cs)),
            memory_caps: [(); NUM_COSETS].map(|_| WrappedMerkleTreeCap::allocate_without_value(cs)),
            stage_2_caps: [(); NUM_COSETS]
                .map(|_| WrappedMerkleTreeCap::allocate_without_value(cs)),
            memory_grand_product_accumulator: MersenneQuartic::allocate_without_value(cs),
            delegation_argument_accumulator: [(); NUM_DELEGATION_CHALLENGES]
                .map(|_| MersenneQuartic::allocate_without_value(cs)),
            quotient_caps: [(); NUM_COSETS]
                .map(|_| WrappedMerkleTreeCap::allocate_without_value(cs)),
            openings_at_z: [(); NUM_OPENINGS_AT_Z]
                .map(|_| MersenneQuartic::allocate_without_value(cs)),
            openings_at_z_omega: [(); NUM_OPENINGS_AT_Z_OMEGA]
                .map(|_| MersenneQuartic::allocate_without_value(cs)),
            fri_intermediate_oracles: [[(); NUM_COSETS]; NUM_FRI_STEPS_WITH_ORACLES]
                .map(|x| x.map(|_| WrappedMerkleTreeCap::allocate_without_value(cs))),
            fri_final_step_leafs: [[(); FINAL_FRI_STEP_LEAF_SIZE_PER_COSET]; NUM_COSETS]
                .map(|x| x.map(|_| MersenneQuartic::allocate_without_value(cs))),
            monomial_coeffs: [(); FRI_FINAL_DEGREE]
                .map(|_| MersenneQuartic::allocate_without_value(cs)),
            pow_nonce: [
                UInt32::allocate_without_value(cs),
                UInt32::allocate_without_value(cs),
            ],
        }
    }
    fn allocate<CS: ConstraintSystem<F>>(cs: &mut CS, witness: Self::Witness) -> Self {
        Self {
            circuit_sequence_idx: UInt32::allocate(cs, witness.circuit_sequence_idx),
            delegation_type: UInt32::allocate(cs, witness.delegation_type),
            public_inputs: witness
                .public_inputs
                .map(|x| MersenneField::allocate(cs, x)),
            setup_caps: witness
                .setup_caps
                .map(|x| WrappedMerkleTreeCap::allocate(cs, x)),
            memory_argument_challenges: WrappedExternalMemoryArgumentChallenges::allocate(
                cs,
                witness.memory_argument_challenges,
            ),
            delegation_argument_challenges: witness
                .delegation_argument_challenges
                .map(|x| WrappedExternalDelegationArgumentChallenges::allocate(cs, x)),
            aux_boundary_values: witness
                .aux_boundary_values
                .map(|x| WrappedAuxArgumentsBoundaryValues::allocate(cs, x)),
            witness_caps: witness
                .witness_caps
                .map(|x| WrappedMerkleTreeCap::allocate(cs, x)),
            memory_caps: witness
                .memory_caps
                .map(|x| WrappedMerkleTreeCap::allocate(cs, x)),
            stage_2_caps: witness
                .stage_2_caps
                .map(|x| WrappedMerkleTreeCap::allocate(cs, x)),
            memory_grand_product_accumulator: MersenneQuartic::allocate(
                cs,
                witness.memory_grand_product_accumulator,
            ),
            delegation_argument_accumulator: witness
                .delegation_argument_accumulator
                .map(|x| MersenneQuartic::allocate(cs, x)),
            quotient_caps: witness
                .quotient_caps
                .map(|x| WrappedMerkleTreeCap::allocate(cs, x)),
            openings_at_z: witness
                .openings_at_z
                .map(|x| MersenneQuartic::allocate(cs, x)),
            openings_at_z_omega: witness
                .openings_at_z_omega
                .map(|x| MersenneQuartic::allocate(cs, x)),
            fri_intermediate_oracles: witness
                .fri_intermediate_oracles
                .map(|x| x.map(|y| WrappedMerkleTreeCap::allocate(cs, y))),
            fri_final_step_leafs: witness
                .fri_final_step_leafs
                .map(|x| x.map(|y| MersenneQuartic::allocate(cs, y))),
            monomial_coeffs: witness
                .monomial_coeffs
                .map(|x| MersenneQuartic::allocate(cs, x)),
            pow_nonce: [
                UInt32::allocate(cs, (witness.pow_nonce & 0xffffffff) as u32),
                UInt32::allocate(cs, (witness.pow_nonce >> 32) as u32),
            ],
        }
    }
}

impl<F: SmallField> WrappedProofSkeletonInstance<F> {
    pub(crate) fn from_non_determinism_source<CS: ConstraintSystem<F>, I: NonDeterminismSource>(
        cs: &mut CS,
        source: &mut I,
    ) -> Self {
        todo!()
    }

    pub(crate) fn transcript_elements_before_stage2(&self) -> Vec<UInt32<F>> {
        let mut result = vec![];

        result.push(self.circuit_sequence_idx.clone());
        result.push(self.delegation_type.clone());
        result.extend_from_slice(&self.public_inputs.map(|x| x.into_uint32()));
        for cap in &self.setup_caps {
            result.extend_from_slice(&cap.to_slice());
        }
        result.extend_from_slice(&self.memory_argument_challenges.to_uint32_vec());
        for challenge in &self.delegation_argument_challenges {
            result.extend_from_slice(&challenge.to_uint32_vec());
        }
        for boundary_values in &self.aux_boundary_values {
            result.extend_from_slice(&boundary_values.to_uint32_vec());
        }
        for cap in &self.witness_caps {
            result.extend_from_slice(&cap.to_slice());
        }
        for cap in &self.memory_caps {
            result.extend_from_slice(&cap.to_slice());
        }

        result
    }

    pub fn transcript_elements_stage2_to_stage3(&self) -> Vec<UInt32<F>> {
        let mut result = vec![];
        for cap in &self.stage_2_caps {
            result.extend_from_slice(&cap.to_slice());
        }
        result.extend_from_slice(&self.memory_grand_product_accumulator.into_uint32s());
        for acc in &self.delegation_argument_accumulator {
            result.extend_from_slice(&acc.into_uint32s());
        }

        result
    }

    pub fn transcript_elements_stage3_to_stage4(&self) -> Vec<UInt32<F>> {
        let mut result = vec![];

        for cap in &self.quotient_caps {
            result.extend_from_slice(&cap.to_slice());
        }

        result
    }

    pub fn transcript_elements_evaluations_at_z(&self) -> Vec<UInt32<F>> {
        let mut result = vec![];

        for opening in &self.openings_at_z {
            result.extend_from_slice(&opening.into_uint32s());
        }
        for opening in &self.openings_at_z_omega {
            result.extend_from_slice(&opening.into_uint32s());
        }

        result
    }

    pub fn transcript_elements_fri_intermediate_oracles(
        &self,
    ) -> [Vec<UInt32<F>>; NUM_FRI_STEPS_WITH_ORACLES] {
        self.fri_intermediate_oracles
            .each_ref()
            .map(|current_step| {
                let mut result = vec![];
                for coset in current_step {
                    result.extend_from_slice(&coset.to_slice());
                }
                result
            })
    }

    pub fn transcript_elements_last_fri_step_leaf_values(&self) -> Vec<UInt32<F>> {
        let mut result = vec![];

        for coset in &self.fri_final_step_leafs {
            for leaf in coset {
                result.extend_from_slice(&leaf.into_uint32s());
            }
        }

        result
    }

    pub fn transcript_elements_monomial_coefficients(&self) -> Vec<UInt32<F>> {
        let mut result = vec![];

        for coeff in &self.monomial_coeffs {
            result.extend_from_slice(&coeff.into_uint32s());
        }

        result
    }
}

pub(crate) type WrappedQueryValuesInstance<F> = WrappedQueryValues<
    F,
    BITS_FOR_QUERY_INDEX,
    DEFAULT_MERKLE_PATH_LENGTH,
    TOTAL_FRI_ORACLES_PATHS_LENGTH,
    LEAF_SIZE_SETUP,
    LEAF_SIZE_WITNESS_TREE,
    LEAF_SIZE_MEMORY_TREE,
    LEAF_SIZE_STAGE_2,
    LEAF_SIZE_QUOTIENT,
    TOTAL_FRI_LEAFS_SIZES,
    NUM_FRI_STEPS,
>;

pub(crate) struct WrappedQueryValues<
    F: SmallField,
    const BITS_FOR_QUERY_INDEX: usize,
    const DEFAULT_MERKLE_PATH_LENGTH: usize,
    const TOTAL_FRI_ORACLES_PATHS_LENGTH: usize,
    const LEAF_SIZE_SETUP: usize,
    const LEAF_SIZE_WITNESS_TREE: usize,
    const LEAF_SIZE_MEMORY_TREE: usize,
    const LEAF_SIZE_STAGE_2: usize,
    const LEAF_SIZE_QUOTIENT: usize,
    const TOTAL_FRI_LEAFS_SIZES: usize,
    const NUM_FRI_STEPS: usize, // we will still bind it here
> {
    pub(crate) query_index: UInt32<F>,
    pub(crate) setup_leaf: [MersenneField<F>; LEAF_SIZE_SETUP],
    pub(crate) witness_leaf: [MersenneField<F>; LEAF_SIZE_WITNESS_TREE],
    pub(crate) memory_leaf: [MersenneField<F>; LEAF_SIZE_MEMORY_TREE],
    pub(crate) stage_2_leaf: [MersenneField<F>; LEAF_SIZE_STAGE_2],
    pub(crate) quotient_leaf: [MersenneField<F>; LEAF_SIZE_QUOTIENT],
    pub(crate) fri_oracles_leafs: [MersenneField<F>; TOTAL_FRI_LEAFS_SIZES],
}

impl<
    F: SmallField,
    const BITS_FOR_QUERY_INDEX: usize,
    const DEFAULT_MERKLE_PATH_LENGTH: usize,
    const TOTAL_FRI_ORACLES_PATHS_LENGTH: usize,
    const LEAF_SIZE_SETUP: usize,
    const LEAF_SIZE_WITNESS_TREE: usize,
    const LEAF_SIZE_MEMORY_TREE: usize,
    const LEAF_SIZE_STAGE_2: usize,
    const LEAF_SIZE_QUOTIENT: usize,
    const TOTAL_FRI_LEAFS_SIZES: usize,
    const NUM_FRI_STEPS: usize,
> CSAllocatable<F>
    for WrappedQueryValues<
        F,
        BITS_FOR_QUERY_INDEX,
        DEFAULT_MERKLE_PATH_LENGTH,
        TOTAL_FRI_ORACLES_PATHS_LENGTH,
        LEAF_SIZE_SETUP,
        LEAF_SIZE_WITNESS_TREE,
        LEAF_SIZE_MEMORY_TREE,
        LEAF_SIZE_STAGE_2,
        LEAF_SIZE_QUOTIENT,
        TOTAL_FRI_LEAFS_SIZES,
        NUM_FRI_STEPS,
    >
{
    type Witness = QueryValues<
        BITS_FOR_QUERY_INDEX,
        DEFAULT_MERKLE_PATH_LENGTH,
        TOTAL_FRI_ORACLES_PATHS_LENGTH,
        LEAF_SIZE_SETUP,
        LEAF_SIZE_WITNESS_TREE,
        LEAF_SIZE_MEMORY_TREE,
        LEAF_SIZE_STAGE_2,
        LEAF_SIZE_QUOTIENT,
        TOTAL_FRI_LEAFS_SIZES,
        NUM_FRI_STEPS,
    >;

    fn placeholder_witness() -> Self::Witness {
        unimplemented!()
    }
    fn allocate_without_value<CS: ConstraintSystem<F>>(cs: &mut CS) -> Self {
        Self {
            query_index: UInt32::allocate_without_value(cs),
            setup_leaf: [(); LEAF_SIZE_SETUP].map(|_| MersenneField::allocate_without_value(cs)),
            witness_leaf: [(); LEAF_SIZE_WITNESS_TREE]
                .map(|_| MersenneField::allocate_without_value(cs)),
            memory_leaf: [(); LEAF_SIZE_MEMORY_TREE]
                .map(|_| MersenneField::allocate_without_value(cs)),
            stage_2_leaf: [(); LEAF_SIZE_STAGE_2]
                .map(|_| MersenneField::allocate_without_value(cs)),
            quotient_leaf: [(); LEAF_SIZE_QUOTIENT]
                .map(|_| MersenneField::allocate_without_value(cs)),
            fri_oracles_leafs: [(); TOTAL_FRI_LEAFS_SIZES]
                .map(|_| MersenneField::allocate_without_value(cs)),
        }
    }
    fn allocate<CS: ConstraintSystem<F>>(cs: &mut CS, witness: Self::Witness) -> Self {
        Self {
            query_index: UInt32::allocate(cs, witness.query_index),
            setup_leaf: witness.setup_leaf.map(|x| MersenneField::allocate(cs, x)),
            witness_leaf: witness.witness_leaf.map(|x| MersenneField::allocate(cs, x)),
            memory_leaf: witness.memory_leaf.map(|x| MersenneField::allocate(cs, x)),
            stage_2_leaf: witness.stage_2_leaf.map(|x| MersenneField::allocate(cs, x)),
            quotient_leaf: witness
                .quotient_leaf
                .map(|x| MersenneField::allocate(cs, x)),
            fri_oracles_leafs: witness
                .fri_oracles_leafs
                .map(|x| MersenneField::allocate(cs, x)),
        }
    }
}

impl<F: SmallField> WrappedQueryValuesInstance<F> {
    fn from_non_determinism_source<
        CS: ConstraintSystem<F>,
        I: NonDeterminismSource,
        V: CircuitLeafInclusionVerifier<F>,
    >(
        cs: &mut CS,
        proof_skeleton: &WrappedProofSkeletonInstance<F>,
        hasher: &mut V,
    ) -> Self {
        // QueryValuesInstance::fill
        todo!()
    }
}

pub struct WrappedProofAuxValues<F: SmallField> {
    pub memory_grand_product_accumulator_final_value: MersenneQuartic<F>,
    pub delegation_argument_accumulator_sum: MersenneQuartic<F>,
}

pub struct WrappedBitSource<F: SmallField> {
    bytes: Vec<UInt8<F>>,
    rest: Vec<Boolean<F>>,
}

impl<F: SmallField> WrappedBitSource<F> {
    pub fn new<CS: ConstraintSystem<F>>(cs: &mut CS, uint32_values: &[UInt32<F>]) -> Self {
        let mut bytes = uint32_values
            .iter()
            .flat_map(|value| value.decompose_into_bytes(cs))
            .rev() // we are going to take from the top
            .collect();

        Self {
            bytes,
            rest: vec![],
        }
    }

    pub fn get_next_bit<CS: ConstraintSystem<F>>(&mut self, cs: &mut CS) -> Boolean<F> {
        if let Some(bit) = self.rest.pop() {
            return bit;
        }

        let next_byte = self.bytes.pop().expect("Must have enough bits");

        self.rest = next_byte
            .into_num()
            .spread_into_bits::<CS, 8>(cs)
            .into_iter()
            .rev()
            .collect();

        self.rest.pop().unwrap()
    }

    pub fn get_next_bits<CS: ConstraintSystem<F>>(
        &mut self,
        cs: &mut CS,
        num_bits: usize,
    ) -> Vec<Boolean<F>> {
        let mut result = Vec::with_capacity(num_bits);
        for _ in 0..num_bits {
            result.push(self.get_next_bit(cs));
        }
        result
    }
}
