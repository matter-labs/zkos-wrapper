use super::*;

use zkos_verifier::prover::nd_source_std::ThreadLocalBasedSource;

pub trait CircuitNonDeterminismSource<F: SmallField> {
    type OutOfCircuitVersion: NonDeterminismSource;

    fn read_word<CS: ConstraintSystem<F>>(&mut self, cs: &mut CS) -> UInt32<F>;
    fn read_reduced_field_element<CS: ConstraintSystem<F>>(&mut self, cs: &mut CS, modulus: UInt32<F>) -> UInt32<F>;
}

impl<F: SmallField> CircuitNonDeterminismSource<F> for ThreadLocalBasedSource {
    type OutOfCircuitVersion = ThreadLocalBasedSource;

    fn read_word<CS: ConstraintSystem<F>>(&mut self, cs: &mut CS) -> UInt32<F> {
        UInt32::allocate_checked(cs, <ThreadLocalBasedSource as NonDeterminismSource>::read_word())
    }

    fn read_reduced_field_element<CS: ConstraintSystem<F>>(&mut self, cs: &mut CS, modulus: UInt32<F>) -> UInt32<F> {
        // TODO change result to MersenneField<F>
        let result_unreduced_value = <ThreadLocalBasedSource as NonDeterminismSource>::read_word();

        let result = UInt32::allocate_from_closure_and_dependencies(
            cs,
            move |inputs: &[F]| {
                let modulus = inputs[0].as_u64_reduced() as u32;
                let result = result_unreduced_value % modulus;
                result
            },
            &[Place::from_variable(modulus.get_variable())],
        );

        // check that result is less than modulus
        let one = Num::allocate_constant(cs, F::ONE);
        let mut check = modulus.into_num();
        check = check.sub(cs, &result.into_num());
        check = check.sub(cs, &one);
        let _ = UInt32::from_variable_checked(cs, check.get_variable());

        result
    }
}

pub trait CircuitLeafInclusionVerifier<F: SmallField> {
    type OutOfCircuitVersion: LeafInclusionVerifier;

    fn new<CS: ConstraintSystem<F>>(cs: &mut CS) -> Self;

    unsafe fn verify_leaf_inclusion<
        CS: ConstraintSystem<F>,
        I: CircuitNonDeterminismSource<F>,
        const CAP_SIZE: usize,
        const NUM_COSETS: usize,
    >(
        &mut self,
        cs: &mut CS,
        coset_index: UInt32<F>,
        leaf_index: UInt32<F>,
        depth: usize,
        leaf_encoding: &[UInt32<F>],
        merkle_cap: &[WrappedMerkleTreeCap<F, CAP_SIZE>; NUM_COSETS],
    ) -> Boolean<F>;
}

