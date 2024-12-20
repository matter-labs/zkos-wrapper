use super::*;

pub struct WrappedProofPublicInputs<F: SmallField, const NUM_STATE_ELEMENTS: usize> {
    pub input_state_variables: [MersenneField<F>; NUM_STATE_ELEMENTS],
    pub output_state_variables: [MersenneField<F>; NUM_STATE_ELEMENTS],
}

impl<F: SmallField, const NUM_STATE_ELEMENTS: usize> CSAllocatable<F> for WrappedProofPublicInputs<F, NUM_STATE_ELEMENTS> {
    type Witness = ProofPublicInputs<NUM_STATE_ELEMENTS>;

    fn placeholder_witness() -> Self::Witness {
        ProofPublicInputs {
            input_state_variables: [Mersenne31Field::ZERO; NUM_STATE_ELEMENTS],
            output_state_variables: [Mersenne31Field::ZERO; NUM_STATE_ELEMENTS],
        }
    }
    fn allocate_without_value<CS: ConstraintSystem<F>>(cs: &mut CS) -> Self {
        Self {
            input_state_variables: [(); NUM_STATE_ELEMENTS].map(|_| MersenneField::allocate_without_value(cs)),
            output_state_variables: [(); NUM_STATE_ELEMENTS].map(|_| MersenneField::allocate_without_value(cs)),
        }
    }
    fn allocate<CS: ConstraintSystem<F>>(cs: &mut CS, witness: Self::Witness) -> Self {
        Self {
            input_state_variables: witness.input_state_variables.map(|x| MersenneField::allocate(cs, x)),
            output_state_variables: witness.output_state_variables.map(|x| MersenneField::allocate(cs, x)),
        }
    }
}

pub struct WrappedMerkleTreeCap<F: SmallField, const N: usize> {
    pub cap: [[UInt32<F>; DIGEST_SIZE_U32_WORDS]; N],
}

impl<F: SmallField, const N: usize> CSAllocatable<F> for WrappedMerkleTreeCap<F, N> {
    type Witness = MerkleTreeCap<N>;

    fn placeholder_witness() -> Self::Witness {
        MerkleTreeCap {cap: [[0u32; DIGEST_SIZE_U32_WORDS]; N]}
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

impl<F: SmallField> CSAllocatable<F> for WrappedExternalMemoryArgumentChallenges<F> {
    type Witness = ExternalMemoryArgumentChallenges;

    fn placeholder_witness() -> Self::Witness {
        ExternalMemoryArgumentChallenges {
            memory_argument_linearization_challenges: [Mersenne31Quartic::ZERO; NUM_MEM_ARGUMENT_LINEARIZATION_CHALLENGES],
            memory_argument_gamma: Mersenne31Quartic::ZERO,
        }
    }
    fn allocate_without_value<CS: ConstraintSystem<F>>(cs: &mut CS) -> Self {
        let memory_argument_linearization_challenges = [(); NUM_MEM_ARGUMENT_LINEARIZATION_CHALLENGES]
            .map(|_| MersenneQuartic::allocate_without_value(cs));

        Self {
            memory_argument_linearization_challenges,
            memory_argument_gamma: MersenneQuartic::allocate_without_value(cs),
        }
    }
    fn allocate<CS: ConstraintSystem<F>>(cs: &mut CS, witness: Self::Witness) -> Self {
        let memory_argument_linearization_challenges = witness.memory_argument_linearization_challenges
            .map(|x| MersenneQuartic::allocate(cs, x));
        let memory_argument_gamma = MersenneQuartic::allocate(cs, witness.memory_argument_gamma);
        Self { memory_argument_linearization_challenges, memory_argument_gamma}
    }
}

pub struct WrappedExternalDelegationArgumentChallenges<F: SmallField> {
    pub delegation_argument_linearization_challenges:
        [MersenneQuartic<F>; NUM_DELEGATION_ARGUMENT_LINEARIZATION_CHALLENGES],
    pub delegation_argument_gamma: MersenneQuartic<F>,
}

impl<F: SmallField> CSAllocatable<F> for WrappedExternalDelegationArgumentChallenges<F> {
    type Witness = ExternalDelegationArgumentChallenges;

    fn placeholder_witness() -> Self::Witness {
        ExternalDelegationArgumentChallenges {
            delegation_argument_linearization_challenges: [Mersenne31Quartic::ZERO; NUM_DELEGATION_ARGUMENT_LINEARIZATION_CHALLENGES],
            delegation_argument_gamma: Mersenne31Quartic::ZERO,
        }
    }
    fn allocate_without_value<CS: ConstraintSystem<F>>(cs: &mut CS) -> Self {
        let delegation_argument_linearization_challenges = [(); NUM_DELEGATION_ARGUMENT_LINEARIZATION_CHALLENGES]
            .map(|_| MersenneQuartic::allocate_without_value(cs));

        Self {
            delegation_argument_linearization_challenges,
            delegation_argument_gamma: MersenneQuartic::allocate_without_value(cs),
        }
    }
    fn allocate<CS: ConstraintSystem<F>>(cs: &mut CS, witness: Self::Witness) -> Self {
        let delegation_argument_linearization_challenges = witness.delegation_argument_linearization_challenges
            .map(|x| MersenneQuartic::allocate(cs, x));
        let delegation_argument_gamma = MersenneQuartic::allocate(cs, witness.delegation_argument_gamma);
        Self { delegation_argument_linearization_challenges, delegation_argument_gamma}
    }
}

pub struct WrappedAuxArgumentsBoundaryValues<F: SmallField> {
    pub lazy_init_first_row: [MersenneField<F>; REGISTER_SIZE],
    pub lazy_init_one_before_last_row: [MersenneField<F>; REGISTER_SIZE],
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
            lazy_init_first_row: [(); REGISTER_SIZE].map(|_| MersenneField::allocate_without_value(cs)),
            lazy_init_one_before_last_row: [(); REGISTER_SIZE].map(|_| MersenneField::allocate_without_value(cs)),
        }
    }
    fn allocate<CS: ConstraintSystem<F>>(cs: &mut CS, witness: Self::Witness) -> Self {
        Self {
            lazy_init_first_row: witness.lazy_init_first_row.map(|x| MersenneField::allocate(cs, x)),
            lazy_init_one_before_last_row: witness.lazy_init_one_before_last_row.map(|x| MersenneField::allocate(cs, x)),
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
    pub delegation_challenges: [WrappedExternalDelegationArgumentChallenges<F>; NUM_DELEGATION_CHALLENGES],
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
> CSAllocatable<F> for WrappedProofOutput<F, CAP_SIZE, NUM_COSETS, NUM_DELEGATION_CHALLENGES, NUM_AUX_BOUNDARY_VALUES> {
    type Witness = ProofOutput<CAP_SIZE, NUM_COSETS, NUM_DELEGATION_CHALLENGES, NUM_AUX_BOUNDARY_VALUES>;

    fn placeholder_witness() -> Self::Witness {
        ProofOutput {
            setup_caps: [WrappedMerkleTreeCap::<F, CAP_SIZE>::placeholder_witness(); NUM_COSETS],
            memory_caps: [WrappedMerkleTreeCap::<F, CAP_SIZE>::placeholder_witness(); NUM_COSETS],
            memory_challenges: WrappedExternalMemoryArgumentChallenges::<F>::placeholder_witness(),
            delegation_challenges: [WrappedExternalDelegationArgumentChallenges::<F>::placeholder_witness(); NUM_DELEGATION_CHALLENGES],
            lazy_init_boundary_values: [WrappedAuxArgumentsBoundaryValues::<F>::placeholder_witness(); NUM_AUX_BOUNDARY_VALUES],
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
            delegation_challenges: [(); NUM_DELEGATION_CHALLENGES].map(|_| WrappedExternalDelegationArgumentChallenges::allocate_without_value(cs)),
            lazy_init_boundary_values: [(); NUM_AUX_BOUNDARY_VALUES].map(|_| WrappedAuxArgumentsBoundaryValues::allocate_without_value(cs)),
            memory_grand_product_accumulator: MersenneQuartic::allocate_without_value(cs),
            delegation_argument_accumulator: [(); NUM_DELEGATION_CHALLENGES].map(|_| MersenneQuartic::allocate_without_value(cs)),
            circuit_sequence: UInt32::allocate_without_value(cs),
            delegation_type: UInt32::allocate_without_value(cs),
        }
    }
    fn allocate<CS: ConstraintSystem<F>>(cs: &mut CS, witness: Self::Witness) -> Self {
        Self {
            setup_caps: witness.setup_caps.map(|x| WrappedMerkleTreeCap::allocate(cs, x)),
            memory_caps: witness.memory_caps.map(|x| WrappedMerkleTreeCap::allocate(cs, x)),
            memory_challenges: WrappedExternalMemoryArgumentChallenges::allocate(cs, witness.memory_challenges),
            delegation_challenges: witness.delegation_challenges.map(|x| WrappedExternalDelegationArgumentChallenges::allocate(cs, x)),
            lazy_init_boundary_values: witness.lazy_init_boundary_values.map(|x| WrappedAuxArgumentsBoundaryValues::allocate(cs, x)),
            memory_grand_product_accumulator: MersenneQuartic::allocate(cs, witness.memory_grand_product_accumulator),
            delegation_argument_accumulator: witness.delegation_argument_accumulator.map(|x| MersenneQuartic::allocate(cs, x)),
            circuit_sequence: UInt32::allocate(cs, witness.circuit_sequence),
            delegation_type: UInt32::allocate(cs, witness.delegation_type),
        }
    }
}
