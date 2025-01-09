use boojum::field::SmallField;
use boojum::gadgets::num::Num;
use boojum::gadgets::u32::UInt32;
use boojum::cs::traits::cs::ConstraintSystem;
use boojum::gadgets::mersenne_field::fourth_ext::MersenneQuartic;
use boojum::gadgets::mersenne_field::MersenneField;
use boojum::gadgets::traits::allocatable::CSAllocatable;
use boojum::gadgets::boolean::Boolean;
use boojum::cs::Place;

use zkos_verifier::concrete::size_constants::*;
use zkos_verifier::prover::definitions::*;
use zkos_verifier::prover::cs::definitions::*;
use zkos_verifier::skeleton::{ProofSkeleton, QueryValues};
use zkos_verifier::field::*;
use zkos_verifier::verifier_common::{ProofPublicInputs, ProofOutput};
use zkos_verifier::verifier_common::non_determinism_source::NonDeterminismSource;
use zkos_verifier::blake2s_u32::*;

mod prover_structs;
mod verifier_traits;
mod transcript;
mod blake2s_reduced;


use prover_structs::*;
use verifier_traits::*;
use transcript::*;
use blake2s_reduced::*;

pub unsafe fn verify<
    F: SmallField,
    CS: ConstraintSystem<F>,
    // I: CircuitNonDeterminismSource<F>,
    V: CircuitLeafInclusionVerifier<F>
>(
    cs: &mut CS,
    proof_state_dst: &mut WrappedProofOutput<
        F, 
        TREE_CAP_SIZE,
        NUM_COSETS,
        NUM_DELEGATION_CHALLENGES,
        NUM_AUX_BOUNDARY_VALUES,
    >,
    proof_input_dst: &mut WrappedProofPublicInputs<F, NUM_STATE_ELEMENTS>,
    skeleton: WrappedProofSkeletonInstance<F>,
    queries: [WrappedQueryValuesInstance<F>; NUM_QUERIES],
) {
    let mut leaf_inclusion_verifier = V::new(cs);

    // now drive the transcript and continue
    let mut transcript_hasher = Blake2sStateGate::<F>::new(cs);
    let mut seed = Blake2sWrappedTranscript::commit_initial_using_hasher(
        cs,
        &mut transcript_hasher,
        &skeleton.transcript_elements_before_stage2(),
    );

    // draw local lookup argument challenges
    const NUM_LOOKUP_CHALLENGES: usize = ((NUM_LOOKUP_ARGUMENT_LINEARIZATION_CHALLENGES + 1) * 4).next_multiple_of(BLAKE2S_DIGEST_SIZE_U32_WORDS);
    let mut transcript_challenges = [UInt32::zero(cs); NUM_LOOKUP_CHALLENGES];
    Blake2sWrappedTranscript::draw_randomness_using_hasher(
        cs,
        &mut transcript_hasher,
        &mut seed,
        &mut transcript_challenges,
    );

    let mut it = transcript_challenges.array_chunks::<4>();
    let lookup_argument_linearization_challenges: [MersenneQuartic<F>; NUM_LOOKUP_ARGUMENT_LINEARIZATION_CHALLENGES] 
        = core::array::from_fn(|_| {
        MersenneQuartic::from_coeffs(
            it.next()
                .unwrap_unchecked()
                .map(|el| MersenneField::from_uint32_with_reduction(cs, el)),
        )
    });
    let lookup_argument_gamma = MersenneQuartic::from_coeffs(
        it.next()
            .unwrap_unchecked()
            .map(|el| MersenneField::from_uint32_with_reduction(cs, el)),
    );

    // commit stage 2 artifacts - tree and memory grand product / delegation set accumulator
    Blake2sWrappedTranscript::commit_with_seed_using_hasher(
        cs,
        &mut transcript_hasher,
        &mut seed,
        &skeleton.transcript_elements_stage2_to_stage3(),
    );

    // draw quotient linearization challenges
    let mut transcript_challenges = [UInt32::zero(cs); (2usize * 4).next_multiple_of(BLAKE2S_DIGEST_SIZE_U32_WORDS)];
    Blake2sWrappedTranscript::draw_randomness_using_hasher(
        cs,
        &mut transcript_hasher,
        &mut seed,
        &mut transcript_challenges,
    );

    let mut it = transcript_challenges.array_chunks::<4>();
    let quotient_alpha = MersenneQuartic::from_coeffs(
        it.next()
            .unwrap_unchecked()
            .map(|el| MersenneField::from_uint32_with_reduction(cs, el)),
    );

    let quotient_beta = MersenneQuartic::from_coeffs(
        it.next()
            .unwrap_unchecked()
            .map(|el| MersenneField::from_uint32_with_reduction(cs, el)),
    );

    // commit quotient tree
    Blake2sWrappedTranscript::commit_with_seed_using_hasher(
        cs,
        &mut transcript_hasher,
        &mut seed,
        &skeleton.transcript_elements_stage3_to_stage4(),
    );

    // draw DEEP poly linearization challenge
    let mut transcript_challenges = [UInt32::zero(cs); (1usize * 4).next_multiple_of(BLAKE2S_DIGEST_SIZE_U32_WORDS)];
    Blake2sWrappedTranscript::draw_randomness_using_hasher(
        cs,
        &mut transcript_hasher,
        &mut seed,
        &mut transcript_challenges,
    );

    let mut it = transcript_challenges.array_chunks::<4>();
    let z = MersenneQuartic::from_coeffs(
        it.next()
            .unwrap_unchecked()
            .map(|el| MersenneField::from_uint32_with_reduction(cs, el)),
    );

    // commit evaluations
    Blake2sWrappedTranscript::commit_with_seed_using_hasher(
        cs,
        &mut transcript_hasher,
        &mut seed,
        &skeleton.transcript_elements_evaluations_at_z(),
    );

    // draw initial challenge for DEEP-poly
    let mut transcript_challenges = [UInt32::zero(cs); (1usize * 4).next_multiple_of(BLAKE2S_DIGEST_SIZE_U32_WORDS)];
    Blake2sWrappedTranscript::draw_randomness_using_hasher(
        cs,
        &mut transcript_hasher,
        &mut seed,
        &mut transcript_challenges,
    );

    let mut it = transcript_challenges.array_chunks::<4>();
    let deep_poly_alpha = MersenneQuartic::from_coeffs(
        it.next()
            .unwrap_unchecked()
            .map(|el| MersenneField::from_uint32_with_reduction(cs, el)),
    );

    // now we should draw challenges and commit FRI oracles
    let mut fri_folding_challenges = [MersenneQuartic::<F>::zero(cs); NUM_FRI_STEPS];

    for (caps, challenge) in skeleton
        .transcript_elements_fri_intermediate_oracles()
        .into_iter()
        .zip(fri_folding_challenges.iter_mut())
    {
        Blake2sWrappedTranscript::commit_with_seed_using_hasher(cs, &mut transcript_hasher, &mut seed, &caps);

        let mut transcript_challenges = [UInt32::zero(cs); (1usize * 4).next_multiple_of(BLAKE2S_DIGEST_SIZE_U32_WORDS)];
        Blake2sWrappedTranscript::draw_randomness_using_hasher(
            cs,
            &mut transcript_hasher,
            &mut seed,
            &mut transcript_challenges,
        );

        let mut it = transcript_challenges.array_chunks::<4>();
        *challenge = MersenneQuartic::from_coeffs(
            it.next()
                .unwrap_unchecked()
                .map(|el| MersenneField::from_uint32_with_reduction(cs, el)),
        );
    }

    if LAST_FRI_STEP_EXPOSE_LEAFS {
        let dst = &mut fri_folding_challenges[NUM_FRI_STEPS - 1];
        Blake2sWrappedTranscript::commit_with_seed_using_hasher(
            cs,
            &mut transcript_hasher,
            &mut seed,
            &skeleton.transcript_elements_last_fri_step_leaf_values(),
        );

        let mut transcript_challenges = [UInt32::zero(cs); (1usize * 4).next_multiple_of(BLAKE2S_DIGEST_SIZE_U32_WORDS)];
        Blake2sWrappedTranscript::draw_randomness_using_hasher(
            cs,
            &mut transcript_hasher,
            &mut seed,
            &mut transcript_challenges,
        );

        let mut it = transcript_challenges.array_chunks::<4>();
        *dst = MersenneQuartic::from_coeffs(
            it.next()
                .unwrap_unchecked()
                .map(|el| MersenneField::from_uint32_with_reduction(cs, el)),
        );
    }

    // commit monomial coefficients before drawing queries
    Blake2sWrappedTranscript::commit_with_seed_using_hasher(
        cs,
        &mut transcript_hasher,
        &mut seed,
        &skeleton.transcript_elements_monomial_coefficients(),
    );

    // now we can verify PoW
    let pow_bits = UInt32::allocate_constant(cs, POW_BITS as u32); // TODO: check if we should use UInt32 instead of u32
    Blake2sWrappedTranscript::verify_pow_using_hasher(
        cs,
        &mut transcript_hasher,
        &mut seed,
        skeleton.pow_nonce,
        pow_bits,
    );

    // now we need to draw enough bits to form query indexes
    let mut indexes_bits = [UInt32::zero(cs); NUM_REQUIRED_WORDS_FOR_QUERY_INDEXES];
    Blake2sWrappedTranscript::draw_randomness_using_hasher(cs, &mut transcript_hasher, &mut seed, &mut indexes_bits);

    todo!()
}

