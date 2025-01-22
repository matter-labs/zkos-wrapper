use std::result;

use boojum::cs::traits::cs::ConstraintSystem;
use boojum::cs::Place;
use boojum::field::SmallField;
use boojum::gadgets::blake2s::mixing_function::Word;
use boojum::gadgets::boolean::Boolean;
use boojum::gadgets::mersenne_field::fourth_ext::MersenneQuartic;
use boojum::gadgets::mersenne_field::second_ext::MersenneComplex;
use boojum::gadgets::mersenne_field::MersenneField;
use boojum::gadgets::num::Num;
use boojum::gadgets::traits::allocatable::CSAllocatable;
use boojum::gadgets::traits::selectable::Selectable;
use boojum::gadgets::u16::UInt16;
use boojum::gadgets::u32::UInt32;
use boojum::gadgets::u8::UInt8;

use zkos_verifier::blake2s_u32::*;
use zkos_verifier::concrete::size_constants::*;
use zkos_verifier::field::*;
use zkos_verifier::prover::cs::definitions::*;
use zkos_verifier::prover::definitions::*;
use zkos_verifier::skeleton::{ProofSkeleton, QueryValues};
use zkos_verifier::verifier_common::non_determinism_source::NonDeterminismSource;
use zkos_verifier::verifier_common::{ProofOutput, ProofPublicInputs};

pub mod blake2s_reduced;
pub(crate) mod prover_structs;
pub mod transcript;
mod transcript_opt;
pub(crate) mod verifier_traits;

mod risc_v_cycles_circuit_layout;

pub use blake2s_reduced::*;
use prover_structs::*;
pub use transcript::*;
use transcript_opt::*;
use verifier_traits::*;

pub fn verify<
    F: SmallField,
    CS: ConstraintSystem<F>,
    // I: CircuitNonDeterminismSource<F>,
    // V: CircuitLeafInclusionVerifier<F>
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
    // now drive the transcript and continue
    let mut transcript_hasher = Blake2sStateGate::<F>::new(cs);
    let mut seed = Blake2sWrappedTranscript::commit_initial_using_hasher(
        cs,
        &mut transcript_hasher,
        &skeleton.transcript_elements_before_stage2(),
    );

    // draw local lookup argument challenges
    const NUM_LOOKUP_CHALLENGES: usize = ((NUM_LOOKUP_ARGUMENT_LINEARIZATION_CHALLENGES + 1) * 4)
        .next_multiple_of(BLAKE2S_DIGEST_SIZE_U32_WORDS);
    let mut transcript_challenges = [UInt32::zero(cs); NUM_LOOKUP_CHALLENGES];
    Blake2sWrappedTranscript::draw_randomness_using_hasher(
        cs,
        &mut transcript_hasher,
        &mut seed,
        &mut transcript_challenges,
    );

    let mut it = transcript_challenges.array_chunks::<4>();
    let lookup_argument_linearization_challenges: [MersenneQuartic<F>;
        NUM_LOOKUP_ARGUMENT_LINEARIZATION_CHALLENGES] = core::array::from_fn(|_| {
        MersenneQuartic::from_coeffs(
            it.next()
                .unwrap()
                .map(|el| MersenneField::from_uint32_with_reduction(cs, el)),
        )
    });
    let lookup_argument_gamma = MersenneQuartic::from_coeffs(
        it.next()
            .unwrap()
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
    let mut transcript_challenges =
        [UInt32::zero(cs); (2usize * 4).next_multiple_of(BLAKE2S_DIGEST_SIZE_U32_WORDS)];
    Blake2sWrappedTranscript::draw_randomness_using_hasher(
        cs,
        &mut transcript_hasher,
        &mut seed,
        &mut transcript_challenges,
    );

    let mut it = transcript_challenges.array_chunks::<4>();
    let quotient_alpha = MersenneQuartic::from_coeffs(
        it.next()
            .unwrap()
            .map(|el| MersenneField::from_uint32_with_reduction(cs, el)),
    );

    let quotient_beta = MersenneQuartic::from_coeffs(
        it.next()
            .unwrap()
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
    let mut transcript_challenges =
        [UInt32::zero(cs); (1usize * 4).next_multiple_of(BLAKE2S_DIGEST_SIZE_U32_WORDS)];
    Blake2sWrappedTranscript::draw_randomness_using_hasher(
        cs,
        &mut transcript_hasher,
        &mut seed,
        &mut transcript_challenges,
    );

    let mut it = transcript_challenges.array_chunks::<4>();
    let z = MersenneQuartic::from_coeffs(
        it.next()
            .unwrap()
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
    let mut transcript_challenges =
        [UInt32::zero(cs); (1usize * 4).next_multiple_of(BLAKE2S_DIGEST_SIZE_U32_WORDS)];
    Blake2sWrappedTranscript::draw_randomness_using_hasher(
        cs,
        &mut transcript_hasher,
        &mut seed,
        &mut transcript_challenges,
    );

    let mut it = transcript_challenges.array_chunks::<4>();
    let deep_poly_alpha = MersenneQuartic::from_coeffs(
        it.next()
            .unwrap()
            .map(|el| MersenneField::from_uint32_with_reduction(cs, el)),
    );

    // now we should draw challenges and commit FRI oracles
    let mut fri_folding_challenges = [MersenneQuartic::<F>::zero(cs); NUM_FRI_STEPS];

    for (caps, challenge) in skeleton
        .transcript_elements_fri_intermediate_oracles()
        .into_iter()
        .zip(fri_folding_challenges.iter_mut())
    {
        Blake2sWrappedTranscript::commit_with_seed_using_hasher(
            cs,
            &mut transcript_hasher,
            &mut seed,
            &caps,
        );

        let mut transcript_challenges =
            [UInt32::zero(cs); (1usize * 4).next_multiple_of(BLAKE2S_DIGEST_SIZE_U32_WORDS)];
        Blake2sWrappedTranscript::draw_randomness_using_hasher(
            cs,
            &mut transcript_hasher,
            &mut seed,
            &mut transcript_challenges,
        );

        let mut it = transcript_challenges.array_chunks::<4>();
        *challenge = MersenneQuartic::from_coeffs(
            it.next()
                .unwrap()
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

        let mut transcript_challenges =
            [UInt32::zero(cs); (1usize * 4).next_multiple_of(BLAKE2S_DIGEST_SIZE_U32_WORDS)];
        Blake2sWrappedTranscript::draw_randomness_using_hasher(
            cs,
            &mut transcript_hasher,
            &mut seed,
            &mut transcript_challenges,
        );

        let mut it = transcript_challenges.array_chunks::<4>();
        *dst = MersenneQuartic::from_coeffs(
            it.next()
                .unwrap()
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
    Blake2sWrappedTranscript::verify_pow_using_hasher::<_, _, POW_BITS>(
        cs,
        &mut transcript_hasher,
        &mut seed,
        skeleton.pow_nonce,
    );

    // now we need to draw enough bits to form query indexes
    let mut indexes_bits = [UInt32::<F>::zero(cs); NUM_REQUIRED_WORDS_FOR_QUERY_INDEXES];
    Blake2sWrappedTranscript::draw_randomness_using_hasher(
        cs,
        &mut transcript_hasher,
        &mut seed,
        &mut indexes_bits,
    );

    // NOTE: when we will use queries below, we MUST check that query set's index is exactly the index we draw from transcript.
    // Indexes in `queries` are already checked to be included in merkle tree caps declared in `skeleton`

    // And NOW we can check algebraic properties of the proof:
    // - quotient evaluation at z
    // - consistency check for DEEP poly
    // - correct folding in FRI

    // quotient evaluation at z

    let omega = MersenneComplex::allocate_constant(
        cs,
        Mersenne31Complex::TWO_ADICITY_GENERATORS[TRACE_LEN_LOG2],
    );
    let omega_inv = MersenneComplex::allocate_constant(
        cs,
        Mersenne31Complex::TWO_ADICITY_GENERATORS_INVERSED[TRACE_LEN_LOG2],
    );

    let mut z_omega = z.mul_by_2nd_ext(cs, &omega);

    {
        // setup, then witness, then memory, then stage 2 base, then stage 2 ext, then quotient
        let (setup, rest) = skeleton.openings_at_z.split_at(NUM_SETUP_OPENINGS);
        let (witness, rest) = rest.split_at(NUM_WITNESS_OPENINGS);
        let (memory, rest) = rest.split_at(NUM_MEMORY_OPENINGS);
        let (stage_2, rest) = rest.split_at(NUM_STAGE2_OPENINGS);
        assert_eq!(rest.len(), 1);
        let quotient_opening = rest[0];

        let mut witness_next_row = [MersenneQuartic::zero(cs); NUM_WITNESS_OPENINGS];
        let mut memory_next_row = [MersenneQuartic::zero(cs); NUM_MEMORY_OPENINGS];
        let mut stage_2_next_row = [MersenneQuartic::zero(cs); NUM_STAGE2_OPENINGS];

        // we only need to update very few places here, so we will overwrite them below

        let mut lookup_argument_two_gamma = lookup_argument_gamma.double(cs);

        let (witness_next_row_set, rest) = skeleton
            .openings_at_z_omega
            .split_at(NUM_WITNESS_OPENING_NEXT_ROW);
        let (memory_next_row_set, rest) = rest.split_at(NUM_MEMORY_OPENING_NEXT_ROW);
        assert_eq!(rest.len(), 1);
        let stage_2_next_row_set = rest;

        debug_assert_eq!(
            witness_next_row_set.len(),
            WITNESS_NEXT_ROW_OPENING_INDEXES.len()
        );
        debug_assert_eq!(
            memory_next_row_set.len(),
            MEMORY_NEXT_ROW_OPENING_INDEXES.len()
        );

        for (index, element) in WITNESS_NEXT_ROW_OPENING_INDEXES
            .iter()
            .zip(witness_next_row_set.iter())
        {
            witness_next_row[*index] = *element;
        }
        for (index, element) in MEMORY_NEXT_ROW_OPENING_INDEXES
            .iter()
            .zip(memory_next_row_set.iter())
        {
            memory_next_row[*index] = *element;
        }

        stage_2_next_row[MEMORY_GRAND_PRODUCT_ACCUMULATOR_POLY_INDEX] = stage_2_next_row_set[0];

        let one_base = MersenneField::one(cs);

        let mut vanishing = z.exp_power_of_2(cs, TRACE_LEN_LOG2);
        vanishing.x.x = vanishing.x.x.sub(cs, &one_base);

        let omega_inv_squared = MersenneComplex::allocate_constant(
            cs,
            Mersenne31Complex::TWO_ADICITY_GENERATORS_INVERSED[TRACE_LEN_LOG2 - 1],
        );

        let mut z_minus_omega_inv = z;
        z_minus_omega_inv.x = z_minus_omega_inv.x.sub(cs, &omega_inv);

        let mut z_minus_omega_inv_squared = z;
        z_minus_omega_inv_squared.x = z_minus_omega_inv_squared.x.sub(cs, &omega_inv_squared);

        // now we should assemble candidates for batch inversion

        // first row is 1 / (x - omega^0)
        let mut first_row_to_inverse = z;
        first_row_to_inverse.x.x = first_row_to_inverse.x.x.sub(cs, &one_base);

        // one before last row is 1/(x - omega^-2)
        let mut one_before_last_row_to_inverse = z;
        one_before_last_row_to_inverse.x =
            one_before_last_row_to_inverse.x.sub(cs, &omega_inv_squared);

        // last row is 1/(x - omega^-1)
        let mut last_row_to_inverse = z;
        last_row_to_inverse.x = last_row_to_inverse.x.sub(cs, &omega_inv);

        let mut to_batch_inverse = [
            z,
            vanishing,
            first_row_to_inverse,
            one_before_last_row_to_inverse,
            last_row_to_inverse,
        ];

        for el in to_batch_inverse.iter_mut() {
            // TODO: can be optimized
            *el = el.inverse_or_zero(cs);

            // low probability here
            let boolean_false = Boolean::allocate_constant(cs, false);
            let is_zero = el.is_zero(cs);

            Boolean::enforce_equal(cs, &is_zero, &boolean_false);
        }

        let [z_inv, vanishing_inv, first_row, one_before_last_row, last_row] = to_batch_inverse;

        // everywhere except last row (x - omega^-1) / (x^n - 1)
        let mut everywhere_except_last = z_minus_omega_inv.mul(cs, &vanishing_inv);

        // everywhere except last two rows
        let mut everywhere_except_last_two_rows =
            everywhere_except_last.mul(cs, &z_minus_omega_inv_squared);

        let mut last_row_and_zero = last_row.mul(cs, &z_inv);

        let divisors = [
            everywhere_except_last,
            everywhere_except_last_two_rows,
            first_row,
            one_before_last_row,
            last_row,
            last_row_and_zero,
        ];

        let delegation_argument_accumulator_sum =
            if skeleton.delegation_argument_accumulator.len() > 0 {
                skeleton.delegation_argument_accumulator[0]
            } else {
                // will be unused, but we do not want to deal with Option
                MersenneQuartic::zero(cs)
            };

        let aux_proof_values = WrappedProofAuxValues {
            memory_grand_product_accumulator_final_value: skeleton.memory_grand_product_accumulator,
            delegation_argument_accumulator_sum,
        };

        let delegation_argument_linearization_challenges =
            if skeleton.delegation_argument_challenges.len() > 0 {
                skeleton.delegation_argument_challenges[0]
            } else {
                WrappedExternalDelegationArgumentChallenges {
                    delegation_argument_linearization_challenges: [MersenneQuartic::zero(cs);
                        NUM_DELEGATION_ARGUMENT_LINEARIZATION_CHALLENGES],
                    delegation_argument_gamma: MersenneQuartic::zero(cs),
                }
            };

        let aux_boundary_values = if skeleton.aux_boundary_values.len() > 0 {
            skeleton.aux_boundary_values[0]
        } else {
            WrappedAuxArgumentsBoundaryValues {
                lazy_init_first_row: [MersenneField::zero(cs); REGISTER_SIZE],
                lazy_init_one_before_last_row: [MersenneField::zero(cs); REGISTER_SIZE],
            }
        };

        let shift = UInt32::allocate_constant(cs, 1 << CIRCUIT_SEQUENCE_BITS_SHIFT as u32);
        let memory_timestamp_high_from_circuit_sequence =
            skeleton.circuit_sequence_idx.non_widening_mul(cs, &shift);
        let memory_timestamp_high_from_circuit_sequence = MersenneField::from_uint32_with_reduction(
            cs,
            memory_timestamp_high_from_circuit_sequence,
        );

        // TODO: should we check if delegation type is valid or should we reduce?
        let delegation_type = MersenneField::from_variable_checked(
            cs,
            skeleton.delegation_type.get_variable(),
            false,
        );

        // we need to show the sum of the values everywhere except the last row,
        // so we show that intermediate poly - interpolant((0, 0), (omega^-1, `value``)) is divisible
        // by our selected divisor, where "value" == negate(our sum over all other domain), and we also require that sum over
        // all the domain is 0

        // interpolant is literaly 1/omega^-1 * value * X (as one can see it's 0 at 0 and `value` at omega^-1)
        let mut delegation_argument_interpolant_linear_coeff =
            delegation_argument_accumulator_sum.mul_by_2nd_ext(cs, &omega);
        delegation_argument_interpolant_linear_coeff =
            delegation_argument_interpolant_linear_coeff.negated(cs);

        // use crate::concrete::evaluate_quotient;
        // let quotient_recomputed_value = evaluate_quotient(
        //     z,
        //     witness,
        //     memory,
        //     setup,
        //     stage_2,
        //     &witness_next_row,
        //     &memory_next_row,
        //     &stage_2_next_row,
        //     quotient_alpha,
        //     quotient_beta,
        //     &divisors,
        //     lookup_argument_linearization_challenges,
        //     lookup_argument_gamma,
        //     lookup_argument_two_gamma,
        //     skeleton
        //         .memory_argument_challenges
        //         .memory_argument_linearization_challenges,
        //     skeleton.memory_argument_challenges.memory_argument_gamma,
        //     delegation_argument_linearization_challenges
        //         .delegation_argument_linearization_challenges,
        //     delegation_argument_linearization_challenges.delegation_argument_gamma,
        //     &skeleton.public_inputs,
        //     &aux_proof_values,
        //     aux_boundary_values,
        //     memory_timestamp_high_from_circuit_sequence,
        //     delegation_type,
        //     delegation_argument_interpolant_linear_coeff,
        // );

        // assert_eq!(
        //     quotient_recomputed_value, quotient_opening,
        //     "quotient evaluation diverged"
        // );
    }

    {
        // For the purposes of FRI below we consider query index as indexing into coset (highest bits) and domain (lowest bits).
        // Both indexes are bitreversed. When we will perform FRI folding we will need to perform an operation like (a - b)/eval_point(a).
        // Since our lowest bits are bitreversed, it means that lowest 3 bits correspond to element arising from 8th root of unity, and at
        // the end of the day we would to precompute 4 elements - 0..=3 powers of 8th root, and every time when our evaluation point is mapped
        // as x -> x^2, we only start to ignore

        assert_eq!(FRI_FACTOR_LOG2, 1);

        // below we will use consistency checks for oracles, where we compute just \sum alpha^i (f(z) - f(x))/(z - x) for few values of x.
        // So we can precompute \sum_i alpha^i f(z) that doesn't change

        let mut extra_factor_for_accumulation_at_z_omega = MersenneQuartic::zero(cs);

        let (precompute_with_evals_at_z, precompute_with_evals_at_z_omega) =
            precompute_for_consistency_checks(
                cs,
                &skeleton,
                &deep_poly_alpha,
                &mut extra_factor_for_accumulation_at_z_omega,
            );

        let omega = MersenneComplex::allocate_constant(
            cs,
            Mersenne31Complex::TWO_ADICITY_GENERATORS[TRACE_LEN_LOG2],
        );
        let tau = MersenneComplex::allocate_constant(
            cs,
            Mersenne31Complex::TWO_ADICITY_GENERATORS[TRACE_LEN_LOG2 + FRI_FACTOR_LOG2],
        );
        let tau_inv = MersenneComplex::allocate_constant(
            cs,
            Mersenne31Complex::TWO_ADICITY_GENERATORS_INVERSED[TRACE_LEN_LOG2 + FRI_FACTOR_LOG2],
        );

        let mut taus = [MersenneComplex::zero(cs); 1 << FRI_FACTOR_LOG2];
        taus[0] = MersenneComplex::one(cs);
        taus[1] = tau;

        let mut taus_inversed = [MersenneComplex::zero(cs); 1 << FRI_FACTOR_LOG2];
        taus_inversed[0] = MersenneComplex::one(cs);
        taus_inversed[1] = tau_inv;

        let mut taus_in_domain_by_half = [MersenneComplex::zero(cs); 1 << FRI_FACTOR_LOG2];
        taus_in_domain_by_half[0] = MersenneComplex::one(cs);
        taus_in_domain_by_half[1] = MersenneComplex::allocate_constant(
            cs,
            Mersenne31Complex::TWO_ADICITY_GENERATORS
                [TRACE_LEN_LOG2 + FRI_FACTOR_LOG2 - (TRACE_LEN_LOG2 - 1)],
        );

        let mut taus_in_domain_by_half_inversed = [MersenneComplex::zero(cs); 1 << FRI_FACTOR_LOG2];
        taus_in_domain_by_half_inversed[0] = MersenneComplex::one(cs);
        taus_in_domain_by_half_inversed[1] = MersenneComplex::allocate_constant(
            cs,
            Mersenne31Complex::TWO_ADICITY_GENERATORS_INVERSED
                [TRACE_LEN_LOG2 + FRI_FACTOR_LOG2 - (TRACE_LEN_LOG2 - 1)],
        );

        // here we will precompute max powers even needed
        let fri_folding_challenges_powers = fri_folding_challenges.map(|el| {
            let squared = el.square(cs);
            let quad = squared.square(cs);
            let eigths = quad.square(cs);
            let sixteens = eigths.square(cs);

            [el, squared, quad, eigths, sixteens]
        });

        // NOTE: here we skip 1 word because PoW is checked over it
        let mut bit_iterator = WrappedBitSource::new(cs, &indexes_bits[1..]);
        let mut inversion_buffer = [MersenneQuartic::zero(cs); 2];
        for query_round in 0..NUM_QUERIES {
            let query = &queries[query_round];
            let query_index_bits = bit_iterator.get_next_bits(cs, BITS_FOR_QUERY_INDEX);

            // assert that our query is at the proper index
            let mut powers = [F::ZERO; BITS_FOR_QUERY_INDEX];
            powers.copy_from_slice(&F::SHIFTS[..BITS_FOR_QUERY_INDEX]);
            boojum::gadgets::impls::lc::linear_combination_collapse(
                cs,
                &mut query_index_bits
                    .iter()
                    .map(|bit| bit.get_variable())
                    .zip(powers),
                Some(query.query_index.get_variable()),
            );

            let mut tree_index_bits = query_index_bits[0..TRACE_LEN_LOG2].to_vec();
            let mut domain_index_bits = tree_index_bits.clone();
            domain_index_bits.reverse();
            let coset_index_bits = query_index_bits[TRACE_LEN_LOG2..].to_vec();

            let mut evaluation_point = omega.pow(cs, &domain_index_bits);

            assert!(coset_index_bits.len() == 1);
            let tau_power =
                MersenneComplex::conditionally_select(cs, coset_index_bits[0], &taus[1], &taus[0]);

            evaluation_point = evaluation_point.mul(cs, &tau_power);

            let mut to_inverse = [z, z_omega];

            for el in to_inverse.iter_mut() {
                el.x = el.x.sub(cs, &evaluation_point);

                // TODO: can be optimized
                *el = el.inverse_or_zero(cs);

                // low probability here
                let boolean_false = Boolean::allocate_constant(cs, false);
                let is_zero = el.is_zero(cs);

                Boolean::enforce_equal(cs, &is_zero, &boolean_false);
            }

            let [divisor_for_z, divisor_for_z_omega] = to_inverse;

            // and can verify FRI. Note that FRI oracle initial leaf is true RS code word, without any adjustments by tau^H/2,
            // and so all next intermediate oracles

            let taus_in_domain_by_half_power = MersenneComplex::conditionally_select(
                cs,
                coset_index_bits[0],
                &taus_in_domain_by_half[1],
                &taus_in_domain_by_half[0],
            );
            let tau_in_domain_by_half_inversed_power = MersenneComplex::conditionally_select(
                cs,
                coset_index_bits[0],
                &taus_in_domain_by_half_inversed[1],
                &taus_in_domain_by_half_inversed[0],
            );

            let mut expected_value = accumulate_over_row_for_consistency_check(
                cs,
                precompute_with_evals_at_z,
                precompute_with_evals_at_z_omega,
                &query,
                deep_poly_alpha,
                extra_factor_for_accumulation_at_z_omega,
                divisor_for_z,
                divisor_for_z_omega,
                taus_in_domain_by_half_power,
                tau_in_domain_by_half_inversed_power,
            );

            let mut domain_size_log_2 = TRACE_LEN_LOG2;
            // let mut domain_index = domain_index as usize;
            // let mut tree_index = tree_index as usize;
            let mut offset_inv = MersenneComplex::conditionally_select(
                cs,
                coset_index_bits[0],
                &taus_inversed[1],
                &taus_inversed[0],
            );
            let mut leaf_src: &[MersenneField<F>] = &query.fri_oracles_leafs;
            for (step, folding_degree_log_2) in FRI_FOLDING_SCHEDULE.iter().enumerate() {
                let leaf_size = (1 << *folding_degree_log_2) * 4;
                let leaf_size_log = folding_degree_log_2 + 2;

                let leaf_projection = if LAST_FRI_STEP_EXPOSE_LEAFS && (step == NUM_FRI_STEPS - 1) {
                    let leaf_size_in_ext4_elements = leaf_size / 4;
                    let leaf_size_in_ext4_elements_log = leaf_size_log - 2;
                    // we should peek into the skeleton for all leaf values
                    let all_leaf_values_in_coset = <[MersenneQuartic<F>;
                        LAST_FRI_STEP_LEAFS_TOTAL_SIZE_PER_COSET]>::conditionally_select(
                        cs,
                        coset_index_bits[0],
                        &skeleton.fri_final_step_leafs[1],
                        &skeleton.fri_final_step_leafs[0],
                    );

                    let leaf_index_bits = &tree_index_bits[leaf_size_in_ext4_elements_log..8];

                    let leaf_projection =
                        get_chunk_with_index_bits(cs, &all_leaf_values_in_coset, leaf_index_bits);

                    leaf_projection
                        .into_iter()
                        .flat_map(|el| el.into_coeffs())
                        .collect()
                } else {
                    let leaf_projection = leaf_src[..leaf_size].to_vec();
                    leaf_src = &leaf_src[leaf_size..];
                    leaf_projection
                };

                assert!(leaf_projection.len() == leaf_size);

                let challenges = fri_folding_challenges_powers[step];

                // NOTE: routine below will check that our expected value is indeed in the leaf at the expected position

                match *folding_degree_log_2 {
                    1 => {
                        fri_fold_by_log_n::<_, _, 1>(
                            cs,
                            &mut expected_value,
                            &mut evaluation_point,
                            &mut domain_size_log_2,
                            &mut domain_index_bits,
                            &mut tree_index_bits,
                            &mut offset_inv,
                            &leaf_projection,
                            &challenges,
                            &SHARED_FACTORS_FOR_FOLDING,
                        );
                    }
                    2 => {
                        fri_fold_by_log_n::<_, _, 2>(
                            cs,
                            &mut expected_value,
                            &mut evaluation_point,
                            &mut domain_size_log_2,
                            &mut domain_index_bits,
                            &mut tree_index_bits,
                            &mut offset_inv,
                            &leaf_projection,
                            &challenges,
                            &SHARED_FACTORS_FOR_FOLDING,
                        );
                    }
                    3 => {
                        fri_fold_by_log_n::<_, _, 3>(
                            cs,
                            &mut expected_value,
                            &mut evaluation_point,
                            &mut domain_size_log_2,
                            &mut domain_index_bits,
                            &mut tree_index_bits,
                            &mut offset_inv,
                            &leaf_projection,
                            &challenges,
                            &SHARED_FACTORS_FOR_FOLDING,
                        );
                    }
                    4 => {
                        fri_fold_by_log_n::<_, _, 4>(
                            cs,
                            &mut expected_value,
                            &mut evaluation_point,
                            &mut domain_size_log_2,
                            &mut domain_index_bits,
                            &mut tree_index_bits,
                            &mut offset_inv,
                            &leaf_projection,
                            &challenges,
                            &SHARED_FACTORS_FOR_FOLDING,
                        );
                    }
                    5 => {
                        fri_fold_by_log_n::<_, _, 5>(
                            cs,
                            &mut expected_value,
                            &mut evaluation_point,
                            &mut domain_size_log_2,
                            &mut domain_index_bits,
                            &mut tree_index_bits,
                            &mut offset_inv,
                            &leaf_projection,
                            &challenges,
                            &SHARED_FACTORS_FOR_FOLDING,
                        );
                    }
                    _ => {
                        unreachable!("too high folding degree");
                    }
                }
            }

            let value_from_monomial_form =
                evaluate_monomial_form(cs, &skeleton.monomial_coeffs, &evaluation_point);

            // FIXME
            //value_from_monomial_form.enforce_equal(cs, &expected_value);
        }
    }

    // NOTE: we will NOT perform any logic about comparison here, and instead we will just write the result back to callee

    // setup caps
    proof_state_dst.setup_caps = skeleton.setup_caps;
    // memory caps
    proof_state_dst.memory_caps = skeleton.memory_caps;
    // - memory challenges
    proof_state_dst.memory_challenges = skeleton.memory_argument_challenges;
    // - delegation challenges
    if NUM_DELEGATION_CHALLENGES > 0 {
        proof_state_dst.delegation_challenges = skeleton.delegation_argument_challenges;
    }
    // - shuffle RAM lazy init first and last values
    if NUM_AUX_BOUNDARY_VALUES > 0 {
        proof_state_dst.lazy_init_boundary_values = skeleton.aux_boundary_values;
    }
    // - memory grand product and delegation accumulators
    proof_state_dst.memory_grand_product_accumulator = skeleton.memory_grand_product_accumulator;
    if NUM_DELEGATION_CHALLENGES > 0 {
        proof_state_dst.delegation_argument_accumulator = skeleton.delegation_argument_accumulator;
    }
    // sequence and delegation types
    proof_state_dst.circuit_sequence = skeleton.circuit_sequence_idx;
    proof_state_dst.delegation_type = skeleton.delegation_type;
    // - input and output state variables
    if NUM_STATE_ELEMENTS > 0 {
        let mut it = skeleton.public_inputs.array_chunks::<NUM_STATE_ELEMENTS>();
        proof_input_dst.input_state_variables = *it.next().unwrap();
        proof_input_dst.output_state_variables = *it.next().unwrap();
    }
}

fn precompute_for_consistency_checks<F: SmallField, CS: ConstraintSystem<F>>(
    cs: &mut CS,
    skeleton: &WrappedProofSkeletonInstance<F>,
    deep_poly_alpha: &MersenneQuartic<F>,
    extra_factor_for_accumulation_at_z_omega: &mut MersenneQuartic<F>,
) -> (MersenneQuartic<F>, MersenneQuartic<F>) {
    // here we should inverse the Horner rule (walk backwards)
    let num_evals_at_z = skeleton.openings_at_z.len();

    let mut i = num_evals_at_z - 1;
    let mut precompute_with_evals_at_z = skeleton.openings_at_z[i];
    while i > 0 {
        i -= 1;
        precompute_with_evals_at_z = deep_poly_alpha.mul_and_add(
            cs,
            &precompute_with_evals_at_z,
            &skeleton.openings_at_z[i],
        );
    }

    let mut i = skeleton.openings_at_z_omega.len() - 1;
    let mut precompute_with_evals_at_z_omega = skeleton.openings_at_z_omega[i];
    while i > 0 {
        i -= 1;
        precompute_with_evals_at_z_omega = deep_poly_alpha.mul_and_add(
            cs,
            &precompute_with_evals_at_z_omega,
            &skeleton.openings_at_z_omega[i],
        );
    }

    // multiply by extra power
    *extra_factor_for_accumulation_at_z_omega = deep_poly_alpha.pow_const(cs, num_evals_at_z);
    precompute_with_evals_at_z_omega =
        precompute_with_evals_at_z_omega.mul(cs, extra_factor_for_accumulation_at_z_omega);

    (precompute_with_evals_at_z, precompute_with_evals_at_z_omega)
}

fn accumulate_over_row_for_consistency_check<F: SmallField, CS: ConstraintSystem<F>>(
    cs: &mut CS,
    precompute_with_evals_at_z: MersenneQuartic<F>,
    precompute_with_evals_at_z_omega: MersenneQuartic<F>,
    query: &WrappedQueryValuesInstance<F>,
    deep_poly_alpha: MersenneQuartic<F>,
    extra_factor_for_accumulation_at_z_omega: MersenneQuartic<F>,
    divisor_for_z: MersenneQuartic<F>,
    divisor_for_z_omega: MersenneQuartic<F>,
    tau_in_domain_by_half: MersenneComplex<F>,
    tau_in_domain_by_half_inversed: MersenneComplex<F>,
) -> MersenneQuartic<F> {
    // let VERIFIER_COMPILED_LAYOUT: VerifierCompiledCircuitArtifact<Mersenne31Field> = {
    //     // IMPORT CONSTANT
    //     todo!()
    // };

    use risc_v_cycles_circuit_layout::VERIFIER_COMPILED_LAYOUT;

    // now we can do consistency check
    let mut accumulated_at_z = MersenneQuartic::zero(cs);

    // setup, then witness, then memory, then stage 2 base, then stage 2 ext, then quotient,
    // but all backwards

    // quotient is just a single value
    {
        let leaf_el = MersenneQuartic::from_coeffs(query.quotient_leaf);
        accumulated_at_z = leaf_el.mul_and_add(cs, &deep_poly_alpha, &accumulated_at_z);
        // NOTE: we compute quotient at non-main domain first, and then LDE, so we do NOT have adjustment
        // there, and we should cancel one below
        accumulated_at_z.x = accumulated_at_z.x.mul(cs, &tau_in_domain_by_half_inversed);
        accumulated_at_z.y = accumulated_at_z.y.mul(cs, &tau_in_domain_by_half_inversed);
    }

    for coeffs in query.stage_2_leaf[VERIFIER_COMPILED_LAYOUT.stage_2_layout.ext4_polys_offset..]
        .array_chunks::<4>()
    {
        let current = MersenneQuartic::from_coeffs(*coeffs);
        accumulated_at_z = accumulated_at_z.mul_and_add(cs, &deep_poly_alpha, &current);
    }

    let mut i = VERIFIER_COMPILED_LAYOUT
        .stage_2_layout
        .num_base_field_polys();
    while i > 0 {
        i -= 1;
        let current = MersenneQuartic::from_base(cs, query.stage_2_leaf[i]);
        accumulated_at_z = accumulated_at_z.mul_and_add(cs, &deep_poly_alpha, &current);
    }

    let mut i = VERIFIER_COMPILED_LAYOUT.memory_layout.total_width;
    while i > 0 {
        i -= 1;
        let current = MersenneQuartic::from_base(cs, query.memory_leaf[i]);
        accumulated_at_z = accumulated_at_z.mul_and_add(cs, &deep_poly_alpha, &current);
    }

    let mut i = VERIFIER_COMPILED_LAYOUT.witness_layout.total_width;
    while i > 0 {
        i -= 1;
        let current = MersenneQuartic::from_base(cs, query.witness_leaf[i]);
        accumulated_at_z = accumulated_at_z.mul_and_add(cs, &deep_poly_alpha, &current);
    }

    let mut i = VERIFIER_COMPILED_LAYOUT.setup_layout.total_width;
    while i > 0 {
        i -= 1;
        let current = MersenneQuartic::from_base(cs, query.setup_leaf[i]);
        accumulated_at_z = accumulated_at_z.mul_and_add(cs, &deep_poly_alpha, &current);
    }

    // all terms are linear over leaf values, so it's enough to scale once
    accumulated_at_z.x = accumulated_at_z.x.mul(cs, &tau_in_domain_by_half);
    accumulated_at_z.y = accumulated_at_z.y.mul(cs, &tau_in_domain_by_half);

    let mut simulated_from_z = precompute_with_evals_at_z.sub(cs, &accumulated_at_z);
    simulated_from_z = simulated_from_z.mul(cs, &divisor_for_z);

    let mut accumulated_at_z_omega = MersenneQuartic::zero(cs);

    for index in WITNESS_NEXT_ROW_OPENING_INDEXES.iter() {
        let current = MersenneQuartic::from_base(cs, query.witness_leaf[*index]);
        accumulated_at_z_omega = accumulated_at_z.mul_and_add(cs, &deep_poly_alpha, &current);
    }
    for index in MEMORY_NEXT_ROW_OPENING_INDEXES.iter() {
        let current = MersenneQuartic::from_base(cs, query.memory_leaf[*index]);
        accumulated_at_z_omega = accumulated_at_z.mul_and_add(cs, &deep_poly_alpha, &current);
    }
    // single element for stage 2
    {
        let leaf_el =
            query.stage_2_leaf[VERIFIER_COMPILED_LAYOUT.stage_2_layout.ext4_polys_offset
                + (MEMORY_GRAND_PRODUCT_ACCUMULATOR_POLY_INDEX
                    - VERIFIER_COMPILED_LAYOUT
                        .stage_2_layout
                        .num_base_field_polys())
                    * 4..]
                .array_chunks::<4>()
                .next()
                .unwrap();
        let current = MersenneQuartic::from_coeffs(*leaf_el);
        accumulated_at_z_omega = accumulated_at_z.mul_and_add(cs, &deep_poly_alpha, &current);
    }

    // all terms are linear over leaf values, so it's enough to scale once
    accumulated_at_z_omega.x = accumulated_at_z_omega.x.mul(cs, &tau_in_domain_by_half);
    accumulated_at_z_omega.y = accumulated_at_z_omega.y.mul(cs, &tau_in_domain_by_half);

    accumulated_at_z_omega =
        accumulated_at_z_omega.mul(cs, &extra_factor_for_accumulation_at_z_omega);

    let mut simulated_from_z_omega = precompute_with_evals_at_z_omega;
    simulated_from_z_omega = simulated_from_z_omega.sub(cs, &accumulated_at_z_omega);
    simulated_from_z_omega = simulated_from_z_omega.mul(cs, &divisor_for_z_omega);

    let mut expected_value = simulated_from_z;
    expected_value = expected_value.add(cs, &simulated_from_z_omega);

    expected_value
}

fn evaluate_monomial_form<F: SmallField, CS: ConstraintSystem<F>>(
    cs: &mut CS,
    monomial_coeffs: &[MersenneQuartic<F>; FRI_FINAL_DEGREE],
    evaluation_point: &MersenneComplex<F>,
) -> MersenneQuartic<F> {
    // now we have our evaluation point, and we can evaluate a result from the monomial form
    // here Horner rule is a little more involved
    // we want to multiply previous by evaluation point, and add to the new one
    let bound = monomial_coeffs.len();
    let mut i = bound - 1;
    let mut value_from_monomial_form = monomial_coeffs[i];
    let evaluation_point = MersenneQuartic::from_complex(cs, *evaluation_point);
    while i > 0 {
        i -= 1;
        value_from_monomial_form =
            value_from_monomial_form.mul_and_add(cs, &evaluation_point, &monomial_coeffs[i]);
    }

    value_from_monomial_form
}

fn get_chunk_with_index_bits<F: SmallField, CS: ConstraintSystem<F>, T: Selectable<F> + Clone>(
    cs: &mut CS,
    src: &[T],
    index_bits: &[Boolean<F>],
) -> Vec<T> {
    assert!(
        src.len() % (1 << index_bits.len()) == 0,
        "src should contain 2^index_bits.len() equal chunks, {} vs {}",
        src.len(),
        1 << index_bits.len()
    );

    let mut result = src.to_vec();

    /*let mut tmp = index_bits.to_vec();
    while src.len() < (1 << tmp.len()) {
        tmp.pop();
        // TODO: assert that what we popped is 0.
    }*/

    for bit in index_bits.iter().rev() {
        let current_length = result.len() / 2;
        let mut current = Vec::with_capacity(current_length);

        for idx in 0..current_length {
            current.push(T::conditionally_select(
                cs,
                bit.clone(),
                &result[idx + current_length],
                &result[idx],
            ));
        }

        result = current;
    }

    result
}

pub fn fri_fold_by_log_n<
    F: SmallField,
    CS: ConstraintSystem<F>,
    const FOLDING_DEGREE_LOG2: usize,
>(
    cs: &mut CS,
    expected_value: &mut MersenneQuartic<F>,
    evaluation_point: &mut MersenneComplex<F>,
    domain_size_log_2: &mut usize,
    domain_index_bits: &mut [Boolean<F>],
    tree_index_bits: &mut [Boolean<F>],
    offset_inv: &mut MersenneComplex<F>,
    leaf: &[MersenneField<F>],
    fri_folding_challenges_powers: &[MersenneQuartic<F>],
    shared_factors_for_folding: &[Mersenne31Complex],
) {
    const MAX_SIZE_FOR_LEAF: usize = 32;
    const MAX_SIZE_FOR_ROOTS: usize = 16;

    assert!(FOLDING_DEGREE_LOG2 > 0);
    assert!(FOLDING_DEGREE_LOG2 <= 5);
    debug_assert_eq!(leaf.len(), (1 << FOLDING_DEGREE_LOG2) * 4);

    let in_leaf_mask: usize = (1 << FOLDING_DEGREE_LOG2) - 1;
    let eval_points_bits_mask = (1 << (*domain_size_log_2 - FOLDING_DEGREE_LOG2)) - 1;
    let generator_inv = MersenneComplex::allocate_constant(
        cs,
        Mersenne31Complex::TWO_ADICITY_GENERATORS_INVERSED[*domain_size_log_2],
    );

    *domain_size_log_2 -= FOLDING_DEGREE_LOG2;

    // here we use worst-case sizes
    let mut leaf_parsed = [MersenneQuartic::zero(cs); MAX_SIZE_FOR_LEAF];
    let mut it = leaf.array_chunks::<4>();
    for i in 0..(1 << FOLDING_DEGREE_LOG2) {
        // NOTE: field elements are reduced in the query already!
        leaf_parsed[i] = MersenneQuartic::from_coeffs(*it.next().unwrap());
    }

    let expected_index_in_rs_code_word_leaf_bits = tree_index_bits[..FOLDING_DEGREE_LOG2].to_vec();
    let value_at_expected_index =
        get_chunk_with_index_bits(cs, &leaf, &expected_index_in_rs_code_word_leaf_bits);
    assert_eq!(value_at_expected_index.len(), 4);

    let value_at_expected_index =
        MersenneQuartic::from_coeffs(value_at_expected_index.try_into().unwrap());
    // check that our simulated value is actually in the leaf
    // FIXME
    //expected_value.enforce_equal(cs, &value_at_expected_index);

    // note that our evaluation points share highest N-3 bits, so we can just precompute additional multiplication
    // factors for lower bits. We only need elements that are not negations of each other

    let shared_bits_in_folding =
        domain_index_bits[..(*domain_size_log_2 - FOLDING_DEGREE_LOG2)].to_vec();
    let mut evaluation_point_shared_factor = generator_inv.pow(cs, &shared_bits_in_folding);
    evaluation_point_shared_factor = evaluation_point_shared_factor.mul(cs, offset_inv);
    // again - worst case size
    let mut folding_evals_points_inversed = [MersenneComplex::zero(cs); MAX_SIZE_FOR_ROOTS];
    for i in 0..(1 << (FOLDING_DEGREE_LOG2 - 1)) {
        let t = MersenneComplex::allocate_constant(cs, shared_factors_for_folding[i]);
        folding_evals_points_inversed[i] = t.mul(cs, &evaluation_point_shared_factor);
    }

    let mut buffer_0 = [MersenneQuartic::zero(cs); MAX_SIZE_FOR_LEAF];
    let mut buffer_1 = [MersenneQuartic::zero(cs); MAX_SIZE_FOR_LEAF];

    for round in 0..FOLDING_DEGREE_LOG2 {
        let roots_stride = 1 << round;
        if round > 0 {
            // we should remap evaluation points
            for i in 0..1 << (FOLDING_DEGREE_LOG2 - round - 1) {
                folding_evals_points_inversed[i * roots_stride] =
                    folding_evals_points_inversed[i * roots_stride].square(cs);
            }
        }
        let (input_buffer, output_buffer) = if round % 2 == 0 {
            (&mut buffer_0, &mut buffer_1)
        } else {
            (&mut buffer_1, &mut buffer_0)
        };
        let challenge = fri_folding_challenges_powers[round];

        for i in 0..1 << (FOLDING_DEGREE_LOG2 - round - 1) {
            let root = folding_evals_points_inversed[i * roots_stride];
            let (a, b) = if round == 0 {
                (leaf_parsed[2 * i], leaf_parsed[2 * i + 1])
            } else {
                (input_buffer[2 * i], input_buffer[2 * i + 1])
            };

            let mut t = a.sub(cs, &b);
            t.x = t.x.mul(cs, &root);
            t.y = t.y.mul(cs, &root);
            let folder = t.mul_and_add(cs, &challenge, &a);

            output_buffer[i] = folder.add(cs, &b);
        }
    }

    *expected_value = if FOLDING_DEGREE_LOG2 % 2 == 0 {
        buffer_0[0]
    } else {
        buffer_1[0]
    };

    for _ in 0..FOLDING_DEGREE_LOG2 {
        *evaluation_point = evaluation_point.square(cs);
        *offset_inv = offset_inv.square(cs);
    }

    for i in 0..(tree_index_bits.len() - FOLDING_DEGREE_LOG2) {
        tree_index_bits[i] = tree_index_bits[i + FOLDING_DEGREE_LOG2].clone();
    }
    for bit in domain_index_bits[(*domain_size_log_2 - FOLDING_DEGREE_LOG2)..].iter_mut() {
        *bit = Boolean::allocate_constant(cs, false);
    }
}
