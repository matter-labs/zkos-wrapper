use super::*;

use proc_macro2::{Span, TokenStream};
use quote::quote;
use syn::Ident;

use crate::everywhere_except_last::*;
use crate::everywhere_except_last_two::{
    transform_linking_constraints, transform_shuffle_ram_lazy_init,
};
use crate::first_or_last_rows::transform_first_or_last_rows;
use crate::memory_accumulators::transform_batch_ram_memory_accumulators;
use crate::memory_accumulators::transform_shuffle_ram_memory_accumulators;
use crate::utils::*;

// mod everywhere_except_last;
// mod memory_accumulators;
// mod utils;

// use self::everywhere_except_last::*;
// use self::memory_accumulators::*;
// use self::utils::*;

// mod everywhere_except_last_two;
// use self::everywhere_except_last_two::*;

// mod first_or_last_rows;
// use self::first_or_last_rows::*;

#[derive(Clone)]
pub(crate) struct Idents {
    pub(crate) cs: Ident,
    pub(crate) random_point_ident: Ident,
    pub(crate) witness_values_ident: Ident,
    pub(crate) memory_values_ident: Ident,
    pub(crate) setup_values_ident: Ident,
    pub(crate) stage_2_values_ident: Ident,
    pub(crate) witness_values_next_row_ident: Ident,
    pub(crate) memory_values_next_row_ident: Ident,
    pub(crate) stage_2_values_next_row_ident: Ident,
    pub(crate) quotient_alpha_ident: Ident,
    pub(crate) quotient_beta_ident: Ident,
    pub(crate) individual_term_ident: Ident,
    pub(crate) terms_accumulator_ident: Ident,
    pub(crate) divisors_ident: Ident,
    pub(crate) memory_argument_linearization_challenges_ident: Ident,
    pub(crate) memory_argument_gamma_ident: Ident,
    pub(crate) lookup_argument_linearization_challenges_ident: Ident,
    pub(crate) lookup_argument_gamma_ident: Ident,
    pub(crate) lookup_argument_two_gamma_ident: Ident,
    pub(crate) delegation_argument_linearization_challenges_ident: Ident,
    pub(crate) delegation_argument_gamma_ident: Ident,
    pub(crate) memory_timestamp_high_from_sequence_idx_ident: Ident,
    pub(crate) public_inputs_ident: Ident,
    #[allow(dead_code)]
    pub(crate) external_values_ident: Ident,
    pub(crate) aux_proof_values_ident: Ident,
    pub(crate) aux_boundary_values_ident: Ident,
    pub(crate) delegation_type_ident: Ident,
    pub(crate) delegation_argument_interpolant_linear_coeff_ident: Ident,
}

pub(crate) fn produce_boolean_constraint(witness_column: usize, idents: &Idents) -> TokenStream {
    let place = prover::cs::one_row_compiler::ColumnAddress::WitnessSubtree(witness_column);
    let read_expr = read_value_expr(place, idents, false);

    let Idents {
        individual_term_ident,
        ..
    } = idents;

    quote! {
        let #individual_term_ident = {
            let value = #read_expr;
            let one = MersenneField::one(cs);
            let mut t = value;
            t = t.sub_base(cs, &one);
            t = t.mul(cs, &value);

            t
        };
    }
}

pub fn generate_inlined(compiled_circuit: CompiledCircuitArtifact<Mersenne31Field>) -> TokenStream {
    // // we need to prepare a description for quotient evaluator, so we will assign the layout to the constant, and will also
    // // will transform a description of the constraints to the literals

    let CompiledCircuitArtifact {
        witness_layout,
        memory_layout,
        setup_layout,
        stage_2_layout,
        degree_2_constraints,
        degree_1_constraints,
        state_linkage_constraints,
        public_inputs,
        lazy_init_address_aux_vars,
        // memory_queries_timestamp_comparison_aux_vars,
        // batched_memory_access_timestamp_comparison_aux_vars,
        ..
    } = compiled_circuit;

    let num_public_inputs = public_inputs.len();

    let cs = Ident::new("cs", Span::call_site());
    let random_point_ident = Ident::new("random_point", Span::call_site());
    let witness_values_ident = Ident::new("witness", Span::call_site());
    let memory_values_ident = Ident::new("memory", Span::call_site());
    let setup_values_ident = Ident::new("setup", Span::call_site());
    let stage_2_values_ident = Ident::new("stage_2", Span::call_site());
    let witness_values_next_row_ident = Ident::new("witness_next_row", Span::call_site());
    let memory_values_next_row_ident = Ident::new("memory_next_row", Span::call_site());
    let stage_2_values_next_row_ident = Ident::new("stage_2_next_row", Span::call_site());
    let quotient_alpha_ident = Ident::new("quotient_alpha", Span::call_site());
    let quotient_beta_ident = Ident::new("quotient_beta", Span::call_site());
    let terms_accumulator_ident = Ident::new("accumulated_contribution", Span::call_site());
    let individual_term_ident = Ident::new("individual_term", Span::call_site());
    let divisors_ident = Ident::new("divisors", Span::call_site());
    let memory_argument_linearization_challenges_ident = Ident::new(
        "memory_argument_linearization_challenges",
        Span::call_site(),
    );
    let memory_argument_gamma_ident = Ident::new("memory_argument_gamma", Span::call_site());
    let lookup_argument_linearization_challenges_ident = Ident::new(
        "lookup_argument_linearization_challenges",
        Span::call_site(),
    );
    let lookup_argument_gamma_ident = Ident::new("lookup_argument_gamma", Span::call_site());
    let lookup_argument_two_gamma_ident =
        Ident::new("lookup_argument_two_gamma", Span::call_site());
    let delegation_argument_linearization_challenges_ident = Ident::new(
        "delegation_argument_linearization_challenges",
        Span::call_site(),
    );
    let delegation_argument_gamma_ident =
        Ident::new("delegation_argument_gamma", Span::call_site());
    let memory_timestamp_high_from_sequence_idx_ident =
        Ident::new("memory_timestamp_high_from_sequence_idx", Span::call_site());
    let public_inputs_ident = Ident::new("public_inputs", Span::call_site());
    let external_values_ident = Ident::new("external_values", Span::call_site());
    let aux_proof_values_ident = Ident::new("aux_proof_values", Span::call_site());
    let aux_boundary_values_ident = Ident::new("aux_boundary_values", Span::call_site());
    let delegation_type_ident = Ident::new("delegation_type", Span::call_site());
    let delegation_argument_interpolant_linear_coeff_ident = Ident::new(
        "delegation_argument_interpolant_linear_coeff",
        Span::call_site(),
    );

    let idents = Idents {
        cs,
        random_point_ident,
        witness_values_ident,
        memory_values_ident,
        setup_values_ident,
        stage_2_values_ident,
        witness_values_next_row_ident,
        memory_values_next_row_ident,
        stage_2_values_next_row_ident,
        terms_accumulator_ident,
        quotient_alpha_ident,
        quotient_beta_ident,
        divisors_ident,
        individual_term_ident,
        memory_argument_linearization_challenges_ident,
        memory_argument_gamma_ident,
        lookup_argument_linearization_challenges_ident,
        lookup_argument_gamma_ident,
        lookup_argument_two_gamma_ident,
        delegation_argument_linearization_challenges_ident,
        delegation_argument_gamma_ident,
        memory_timestamp_high_from_sequence_idx_ident,
        public_inputs_ident,
        external_values_ident,
        aux_proof_values_ident,
        aux_boundary_values_ident,
        delegation_type_ident,
        delegation_argument_interpolant_linear_coeff_ident,
    };

    let num_boolean_constraints = witness_layout.boolean_vars_columns_range.num_elements();

    // we also use Horner rule, so we reduce multiplication complexity

    // first all the constraints for the case of every row except last
    let mut every_low_except_last_subexprs: Vec<(Option<TokenStream>, Vec<TokenStream>)> = vec![];

    let mut common_constraints = vec![];
    // specialized boolean constraints, that can be degraded to single multiplication effectively
    for i in 0..num_boolean_constraints {
        let column_index = witness_layout.boolean_vars_columns_range.get_range(i).start;
        let expr = produce_boolean_constraint(column_index, &idents);
        common_constraints.push(expr);
    }
    // constraints themselves, skipping boolean
    for el in degree_2_constraints
        .into_iter()
        .skip(num_boolean_constraints)
    {
        let expr = transform_degree_2_constraint(el, &idents);
        common_constraints.push(expr);
    }

    for el in degree_1_constraints.into_iter() {
        let expr = transform_degree_1_constraint(el, &idents);
        common_constraints.push(expr);
    }
    every_low_except_last_subexprs.push((None, common_constraints));

    // special compiler-defined constraints. Note that all timestamp comparisons are effectively
    // merged with width-16 range checks

    // if we process delegations - we should process checks in case if processing doesn't happen
    if memory_layout.delegation_processor_layout.is_some() {
        let (common, exprs) = transform_batch_ram_conventions(&memory_layout, &idents);
        every_low_except_last_subexprs.push((Some(common), exprs));
    }

    // now lookup width 1
    {
        // range check 16
        {
            let bound = stage_2_layout
                .intermediate_polys_for_range_check_16
                .num_pairs;
            assert_eq!(
                bound,
                witness_layout.range_check_16_lookup_expressions.len() / 2
            );
            let num_shuffle_ram_accesses = memory_layout.shuffle_ram_access_sets.len();
            let shuffle_ram_special_case_bound = bound - num_shuffle_ram_accesses;
            assert!(witness_layout.range_check_16_lookup_expressions.len() % 2 == 0);
            for (i, pair) in witness_layout
                .range_check_16_lookup_expressions
                .array_chunks::<2>()
                .enumerate()
            {
                let (common, t) = transform_width_1_range_checks_pair(
                    pair,
                    i,
                    stage_2_layout.intermediate_polys_for_range_check_16,
                    &idents,
                    &stage_2_layout,
                    false,
                );

                every_low_except_last_subexprs.push((Some(common), t));
            }
        }

        // special case for range check over lazy init address columns
        if let Some(shuffle_ram_inits_and_teardowns) = memory_layout.shuffle_ram_inits_and_teardowns
        {
            let lazy_init_address_range_check_16 = stage_2_layout
                .lazy_init_address_range_check_16
                .expect("must exist if we do lazy init");
            let t = transform_shuffle_ram_lazy_init_range_checks(
                lazy_init_address_range_check_16,
                shuffle_ram_inits_and_teardowns,
                &idents,
                &stage_2_layout,
            );

            every_low_except_last_subexprs.push((None, t));
        }

        // now remainders
        // Acc(x) * (witness(x) + gamma) - 1
        if let Some(_remainder_for_range_check_16) = stage_2_layout.remainder_for_range_check_16 {
            todo!();
        }
    }

    // timestamp range checks
    {
        let bound = stage_2_layout
            .intermediate_polys_for_timestamp_range_checks
            .num_pairs;
        assert_eq!(
            bound,
            witness_layout
                .timestamp_range_check_lookup_expressions
                .len()
                / 2
        );
        let num_shuffle_ram_accesses = memory_layout.shuffle_ram_access_sets.len();
        let shuffle_ram_special_case_bound = bound - num_shuffle_ram_accesses;
        assert!(
            witness_layout
                .timestamp_range_check_lookup_expressions
                .len()
                % 2
                == 0
        );
        for (i, pair) in witness_layout
            .timestamp_range_check_lookup_expressions
            .array_chunks::<2>()
            .enumerate()
        {
            if i < shuffle_ram_special_case_bound {
                let (common, t) = transform_width_1_range_checks_pair(
                    pair,
                    i,
                    stage_2_layout.intermediate_polys_for_timestamp_range_checks,
                    &idents,
                    &stage_2_layout,
                    false,
                );

                every_low_except_last_subexprs.push((Some(common), t));
            } else {
                let (common, t) = transform_width_1_range_checks_pair(
                    pair,
                    i,
                    stage_2_layout.intermediate_polys_for_timestamp_range_checks,
                    &idents,
                    &stage_2_layout,
                    true,
                );

                every_low_except_last_subexprs.push((Some(common), t));
            }
        }
    }

    // now generic lookup
    {
        let t = transform_generic_lookup(&witness_layout, &stage_2_layout, &setup_layout, &idents);
        every_low_except_last_subexprs.push((None, t));
    }

    // multiplicities
    {
        let t = transform_multiplicities(&witness_layout, &stage_2_layout, &setup_layout, &idents);
        every_low_except_last_subexprs.push((None, t));
    }

    // if we work with delegation argument - then transform them

    // creating of requests
    if memory_layout.delegation_request_layout.is_some() {
        let t = transform_delegation_requests_creation(
            &memory_layout,
            &stage_2_layout,
            &setup_layout,
            &idents,
        );
        every_low_except_last_subexprs.push((None, t));
    }

    // processing of requests
    if memory_layout.delegation_processor_layout.is_some() {
        let t = transform_delegation_requests_processing(&memory_layout, &stage_2_layout, &idents);
        every_low_except_last_subexprs.push((None, t));
    }

    // check padding of lazy-init
    if let Some(shuffle_ram_inits_and_teardowns) = memory_layout.shuffle_ram_inits_and_teardowns {
        let lazy_init_address_aux_vars = lazy_init_address_aux_vars.as_ref().expect("exists");
        let (common, exprs) = transform_shuffle_ram_lazy_init_padding(
            shuffle_ram_inits_and_teardowns,
            &lazy_init_address_aux_vars,
            &idents,
        );
        every_low_except_last_subexprs.push((Some(common), exprs));
    }

    // shuffle RAM memory accumulators

    if memory_layout.shuffle_ram_access_sets.len() > 0 {
        assert!(memory_layout.shuffle_ram_inits_and_teardowns.is_some());

        let t = transform_shuffle_ram_memory_accumulators(
            &memory_layout,
            &stage_2_layout,
            &setup_layout,
            &idents,
        );
        every_low_except_last_subexprs.push((None, t));
    }

    // batch RAM memory accumulators

    if memory_layout.batched_ram_accesses.len() > 0 {
        assert!(memory_layout.shuffle_ram_inits_and_teardowns.is_none());
        assert!(memory_layout.shuffle_ram_access_sets.len() == 0);

        let (common, exprs) =
            transform_batch_ram_memory_accumulators(&memory_layout, &stage_2_layout, &idents);
        every_low_except_last_subexprs.push((Some(common), exprs));
    }

    let divisor_idx = prover::DIVISOR_EVERYWHERE_EXCEPT_LAST_ROW_INDEX;

    let mut stream = TokenStream::new();
    let mut is_first = true;

    for (common, contribution) in every_low_except_last_subexprs.into_iter() {
        let contribution = accumulate_contributions(&mut is_first, common, contribution, &idents);
        stream.extend(contribution);
    }

    let divisors_ident = &idents.divisors_ident;
    let terms_accumulator_ident = &idents.terms_accumulator_ident;

    let every_row_except_last = if stream.is_empty() {
        quote! {
            let every_row_except_last_contribution = MersenneQuartic::zero(cs);
        }
    } else {
        quote! {
            let every_row_except_last_contribution = {
                #stream

                // now divide
                let divisor = #divisors_ident[#divisor_idx];
                #terms_accumulator_ident = #terms_accumulator_ident.mul(cs, &divisor);

                #terms_accumulator_ident
            };
        }
    };

    // now evert row except the last two

    let mut stream = TokenStream::new();

    // shuffle RAM lazy init

    let mut every_low_except_last_two_subexprs = vec![];
    // linking constraints
    if state_linkage_constraints.len() > 0 {
        let t = transform_linking_constraints(&state_linkage_constraints, &idents);
        every_low_except_last_two_subexprs.push((None, t));
    }

    // and shuffle RAM lazy init if it exists
    if let Some(shuffle_ram_inits_and_teardowns) = memory_layout.shuffle_ram_inits_and_teardowns {
        let lazy_init_address_aux_vars = lazy_init_address_aux_vars.as_ref().expect("exists");
        let (common, exprs) = transform_shuffle_ram_lazy_init(
            shuffle_ram_inits_and_teardowns,
            &lazy_init_address_aux_vars,
            &idents,
        );
        every_low_except_last_two_subexprs.push((Some(common), exprs));
    }

    let mut is_first = true;

    for (common, contribution) in every_low_except_last_two_subexprs.into_iter() {
        let contribution = accumulate_contributions(&mut is_first, common, contribution, &idents);
        stream.extend(contribution);
    }

    let divisor_idx = prover::DIVISOR_EVERYWHERE_EXCEPT_LAST_TWO_ROWS_INDEX;

    let every_row_except_two_last = if stream.is_empty() == false {
        quote! {
            let every_row_except_two_last_contribution = {
                #stream

                // now divide
                let divisor = #divisors_ident[#divisor_idx];
                #terms_accumulator_ident = #terms_accumulator_ident.mul(cs, &divisor);

                #terms_accumulator_ident
            };
        }
    } else {
        quote! {
            let every_row_except_two_last_contribution = MersenneQuartic::zero(cs);
        }
    };

    // first, one before last, last and last+0 cases

    let (first_row, one_before_last_row, last_row, last_row_and_zero) =
        transform_first_or_last_rows(&memory_layout, &stage_2_layout, &public_inputs, &idents);

    let divisor_idx = prover::DIVISOR_FIRST_ROW_INDEX;

    let first_row = if first_row.is_empty() == false {
        quote! {
            let first_row_contribution = {
                #first_row

                // now divide
                let divisor = #divisors_ident[#divisor_idx];
                #terms_accumulator_ident = #terms_accumulator_ident.mul(cs, &divisor);

                #terms_accumulator_ident
            };
        }
    } else {
        quote! {
            let first_row_contribution = MersenneQuartic::zero(cs);
        }
    };

    let divisor_idx = prover::DIVISOR_ONE_BEFORE_LAST_ROW_INDEX;

    let one_before_last_row = if one_before_last_row.is_empty() == false {
        quote! {
            let one_before_last_row_contribution = {
                #one_before_last_row

                // now divide
                let divisor = #divisors_ident[#divisor_idx];
                #terms_accumulator_ident = #terms_accumulator_ident.mul(cs, &divisor);

                #terms_accumulator_ident
            };
        }
    } else {
        quote! {
            let one_before_last_row_contribution = MersenneQuartic::zero(cs);
        }
    };

    let divisor_idx = prover::DIVISOR_LAST_ROW_INDEX;

    let last_row = if last_row.is_empty() == false {
        quote! {
            let last_row_contribution = {
                #last_row

                // now divide
                let divisor = #divisors_ident[#divisor_idx];
                #terms_accumulator_ident = #terms_accumulator_ident.mul(cs, &divisor);

                #terms_accumulator_ident
            };
        }
    } else {
        quote! {
            let last_row_contribution = MersenneQuartic::zero(cs);
        }
    };

    let divisor_idx = prover::DIVISOR_LAST_ROW_AND_ZERO_INDEX;

    let last_row_and_zero = if last_row_and_zero.is_empty() == false {
        quote! {
            let last_row_and_zero_contribution = {
                #last_row_and_zero

                // now divide
                let divisor = #divisors_ident[#divisor_idx];
                #terms_accumulator_ident = #terms_accumulator_ident.mul(cs, &divisor);

                #terms_accumulator_ident
            };
        }
    } else {
        quote! {
            let last_row_and_zero_contribution = MersenneQuartic::zero(cs);
        }
    };

    let Idents {
        cs,
        random_point_ident,
        witness_values_ident,
        memory_values_ident,
        setup_values_ident,
        stage_2_values_ident,
        witness_values_next_row_ident,
        memory_values_next_row_ident,
        stage_2_values_next_row_ident,
        quotient_alpha_ident,
        quotient_beta_ident,
        // individual_term_ident,
        // terms_accumulator_ident,
        divisors_ident,
        memory_argument_linearization_challenges_ident,
        memory_argument_gamma_ident,
        lookup_argument_linearization_challenges_ident,
        lookup_argument_gamma_ident,
        lookup_argument_two_gamma_ident,
        delegation_argument_linearization_challenges_ident,
        delegation_argument_gamma_ident,
        memory_timestamp_high_from_sequence_idx_ident,
        public_inputs_ident,
        // external_values_ident,
        aux_proof_values_ident,
        aux_boundary_values_ident,
        delegation_type_ident,
        delegation_argument_interpolant_linear_coeff_ident,
        ..
    } = idents;

    let num_different_divisors = prover::NUM_DIFFERENT_DIVISORS;

    quote! {

        #[allow(unused_braces, unused_mut, unused_variables, unsafe_op_in_unsafe_fn, unused_assignments)]
        unsafe fn evaluate_every_row_except_last<F: SmallField, CS: ConstraintSystem<F>>(
            #cs: &mut CS,
            #random_point_ident: MersenneQuartic<F>,
            #witness_values_ident: &[MersenneQuartic<F>],
            #memory_values_ident: &[MersenneQuartic<F>],
            #setup_values_ident: &[MersenneQuartic<F>],
            #stage_2_values_ident: &[MersenneQuartic<F>],
            #witness_values_next_row_ident: &[MersenneQuartic<F>],
            #memory_values_next_row_ident: &[MersenneQuartic<F>],
            #stage_2_values_next_row_ident: &[MersenneQuartic<F>],
            #quotient_alpha_ident: MersenneQuartic<F>,
            #quotient_beta_ident: MersenneQuartic<F>,
            #divisors_ident: &[MersenneQuartic<F>; #num_different_divisors],
            #lookup_argument_linearization_challenges_ident: [MersenneQuartic<F>; NUM_LOOKUP_ARGUMENT_LINEARIZATION_CHALLENGES],
            #lookup_argument_gamma_ident: MersenneQuartic<F>,
            #lookup_argument_two_gamma_ident: MersenneQuartic<F>,
            #memory_argument_linearization_challenges_ident: [MersenneQuartic<F>; NUM_MEM_ARGUMENT_LINEARIZATION_CHALLENGES],
            #memory_argument_gamma_ident: MersenneQuartic<F>,
            #delegation_argument_linearization_challenges_ident: [MersenneQuartic<F>; NUM_DELEGATION_ARGUMENT_LINEARIZATION_CHALLENGES],
            #delegation_argument_gamma_ident: MersenneQuartic<F>,
            #public_inputs_ident: &[MersenneField<F>; #num_public_inputs],
            #aux_proof_values_ident: &WrappedProofAuxValues<F>,
            #aux_boundary_values_ident: WrappedAuxArgumentsBoundaryValues<F>,
            #memory_timestamp_high_from_sequence_idx_ident: MersenneField<F>,
            #delegation_type_ident: MersenneField<F>,
            #delegation_argument_interpolant_linear_coeff_ident: MersenneQuartic<F>,
        ) -> MersenneQuartic<F> {
            #every_row_except_last

            every_row_except_last_contribution
        }

        #[allow(unused_braces, unused_mut, unused_variables, unsafe_op_in_unsafe_fn, unused_assignments)]
        unsafe fn evaluate_every_row_except_two<F: SmallField, CS: ConstraintSystem<F>>(
            #cs: &mut CS,
            #random_point_ident: MersenneQuartic<F>,
            #witness_values_ident: &[MersenneQuartic<F>],
            #memory_values_ident: &[MersenneQuartic<F>],
            #setup_values_ident: &[MersenneQuartic<F>],
            #stage_2_values_ident: &[MersenneQuartic<F>],
            #witness_values_next_row_ident: &[MersenneQuartic<F>],
            #memory_values_next_row_ident: &[MersenneQuartic<F>],
            #stage_2_values_next_row_ident: &[MersenneQuartic<F>],
            #quotient_alpha_ident: MersenneQuartic<F>,
            #quotient_beta_ident: MersenneQuartic<F>,
            #divisors_ident: &[MersenneQuartic<F>; #num_different_divisors],
            #lookup_argument_linearization_challenges_ident: [MersenneQuartic<F>; NUM_LOOKUP_ARGUMENT_LINEARIZATION_CHALLENGES],
            #lookup_argument_gamma_ident: MersenneQuartic<F>,
            #lookup_argument_two_gamma_ident: MersenneQuartic<F>,
            #memory_argument_linearization_challenges_ident: [MersenneQuartic<F>; NUM_MEM_ARGUMENT_LINEARIZATION_CHALLENGES],
            #memory_argument_gamma_ident: MersenneQuartic<F>,
            #delegation_argument_linearization_challenges_ident: [MersenneQuartic<F>; NUM_DELEGATION_ARGUMENT_LINEARIZATION_CHALLENGES],
            #delegation_argument_gamma_ident: MersenneQuartic<F>,
            #public_inputs_ident: &[MersenneField<F>; #num_public_inputs],
            #aux_proof_values_ident: &WrappedProofAuxValues<F>,
            #aux_boundary_values_ident: WrappedAuxArgumentsBoundaryValues<F>,
            #memory_timestamp_high_from_sequence_idx_ident: MersenneField<F>,
            #delegation_type_ident: MersenneField<F>,
            #delegation_argument_interpolant_linear_coeff_ident: MersenneQuartic<F>,
        ) -> MersenneQuartic<F> {
            #every_row_except_two_last

            every_row_except_two_last_contribution
        }

        #[allow(unused_braces, unused_mut, unused_variables, unsafe_op_in_unsafe_fn, unused_assignments)]
        unsafe fn evaluate_last_row_and_zero<F: SmallField, CS: ConstraintSystem<F>>(
            #cs: &mut CS,
            #random_point_ident: MersenneQuartic<F>,
            #witness_values_ident: &[MersenneQuartic<F>],
            #memory_values_ident: &[MersenneQuartic<F>],
            #setup_values_ident: &[MersenneQuartic<F>],
            #stage_2_values_ident: &[MersenneQuartic<F>],
            #witness_values_next_row_ident: &[MersenneQuartic<F>],
            #memory_values_next_row_ident: &[MersenneQuartic<F>],
            #stage_2_values_next_row_ident: &[MersenneQuartic<F>],
            #quotient_alpha_ident: MersenneQuartic<F>,
            #quotient_beta_ident: MersenneQuartic<F>,
            #divisors_ident: &[MersenneQuartic<F>; #num_different_divisors],
            #lookup_argument_linearization_challenges_ident: [MersenneQuartic<F>; NUM_LOOKUP_ARGUMENT_LINEARIZATION_CHALLENGES],
            #lookup_argument_gamma_ident: MersenneQuartic<F>,
            #lookup_argument_two_gamma_ident: MersenneQuartic<F>,
            #memory_argument_linearization_challenges_ident: [MersenneQuartic<F>; NUM_MEM_ARGUMENT_LINEARIZATION_CHALLENGES],
            #memory_argument_gamma_ident: MersenneQuartic<F>,
            #delegation_argument_linearization_challenges_ident: [MersenneQuartic<F>; NUM_DELEGATION_ARGUMENT_LINEARIZATION_CHALLENGES],
            #delegation_argument_gamma_ident: MersenneQuartic<F>,
            #public_inputs_ident: &[MersenneField<F>; #num_public_inputs],
            #aux_proof_values_ident: &WrappedProofAuxValues<F>,
            #aux_boundary_values_ident: WrappedAuxArgumentsBoundaryValues<F>,
            #memory_timestamp_high_from_sequence_idx_ident: MersenneField<F>,
            #delegation_type_ident: MersenneField<F>,
            #delegation_argument_interpolant_linear_coeff_ident: MersenneQuartic<F>,
        ) -> MersenneQuartic<F> {
            #last_row_and_zero

            last_row_and_zero_contribution
        }

        #[allow(unused_braces, unused_mut, unused_variables, unsafe_op_in_unsafe_fn, unused_assignments)]
        pub unsafe fn evaluate_quotient<F: SmallField, CS: ConstraintSystem<F>>(
            #cs: &mut CS,
            #random_point_ident: MersenneQuartic<F>,
            #witness_values_ident: &[MersenneQuartic<F>],
            #memory_values_ident: &[MersenneQuartic<F>],
            #setup_values_ident: &[MersenneQuartic<F>],
            #stage_2_values_ident: &[MersenneQuartic<F>],
            #witness_values_next_row_ident: &[MersenneQuartic<F>],
            #memory_values_next_row_ident: &[MersenneQuartic<F>],
            #stage_2_values_next_row_ident: &[MersenneQuartic<F>],
            #quotient_alpha_ident: MersenneQuartic<F>,
            #quotient_beta_ident: MersenneQuartic<F>,
            #divisors_ident: &[MersenneQuartic<F>; #num_different_divisors],
            #lookup_argument_linearization_challenges_ident: [MersenneQuartic<F>; NUM_LOOKUP_ARGUMENT_LINEARIZATION_CHALLENGES],
            #lookup_argument_gamma_ident: MersenneQuartic<F>,
            #lookup_argument_two_gamma_ident: MersenneQuartic<F>,
            #memory_argument_linearization_challenges_ident: [MersenneQuartic<F>; NUM_MEM_ARGUMENT_LINEARIZATION_CHALLENGES],
            #memory_argument_gamma_ident: MersenneQuartic<F>,
            #delegation_argument_linearization_challenges_ident: [MersenneQuartic<F>; NUM_DELEGATION_ARGUMENT_LINEARIZATION_CHALLENGES],
            #delegation_argument_gamma_ident: MersenneQuartic<F>,
            #public_inputs_ident: &[MersenneField<F>; #num_public_inputs],
            #aux_proof_values_ident: &WrappedProofAuxValues<F>,
            #aux_boundary_values_ident: WrappedAuxArgumentsBoundaryValues<F>,
            #memory_timestamp_high_from_sequence_idx_ident: MersenneField<F>,
            #delegation_type_ident: MersenneField<F>,
            #delegation_argument_interpolant_linear_coeff_ident: MersenneQuartic<F>,
        ) -> MersenneQuartic<F> {
            let every_row_except_last_contribution = unsafe{ evaluate_every_row_except_last(
                #cs,
                #random_point_ident,
                #witness_values_ident,
                #memory_values_ident,
                #setup_values_ident,
                #stage_2_values_ident,
                #witness_values_next_row_ident,
                #memory_values_next_row_ident,
                #stage_2_values_next_row_ident,
                #quotient_alpha_ident,
                #quotient_beta_ident,
                #divisors_ident,
                #lookup_argument_linearization_challenges_ident,
                #lookup_argument_gamma_ident,
                #lookup_argument_two_gamma_ident,
                #memory_argument_linearization_challenges_ident,
                #memory_argument_gamma_ident,
                #delegation_argument_linearization_challenges_ident,
                #delegation_argument_gamma_ident,
                #public_inputs_ident,
                #aux_proof_values_ident,
                #aux_boundary_values_ident,
                #memory_timestamp_high_from_sequence_idx_ident,
                #delegation_type_ident,
                #delegation_argument_interpolant_linear_coeff_ident,
            )};

            let every_row_except_two_last_contribution = unsafe{ evaluate_every_row_except_two(
                #cs,
                #random_point_ident,
                #witness_values_ident,
                #memory_values_ident,
                #setup_values_ident,
                #stage_2_values_ident,
                #witness_values_next_row_ident,
                #memory_values_next_row_ident,
                #stage_2_values_next_row_ident,
                #quotient_alpha_ident,
                #quotient_beta_ident,
                #divisors_ident,
                #lookup_argument_linearization_challenges_ident,
                #lookup_argument_gamma_ident,
                #lookup_argument_two_gamma_ident,
                #memory_argument_linearization_challenges_ident,
                #memory_argument_gamma_ident,
                #delegation_argument_linearization_challenges_ident,
                #delegation_argument_gamma_ident,
                #public_inputs_ident,
                #aux_proof_values_ident,
                #aux_boundary_values_ident,
                #memory_timestamp_high_from_sequence_idx_ident,
                #delegation_type_ident,
                #delegation_argument_interpolant_linear_coeff_ident,
            )};

            let last_row_and_zero_contribution = unsafe{ evaluate_last_row_and_zero(
                #cs,
                #random_point_ident,
                #witness_values_ident,
                #memory_values_ident,
                #setup_values_ident,
                #stage_2_values_ident,
                #witness_values_next_row_ident,
                #memory_values_next_row_ident,
                #stage_2_values_next_row_ident,
                #quotient_alpha_ident,
                #quotient_beta_ident,
                #divisors_ident,
                #lookup_argument_linearization_challenges_ident,
                #lookup_argument_gamma_ident,
                #lookup_argument_two_gamma_ident,
                #memory_argument_linearization_challenges_ident,
                #memory_argument_gamma_ident,
                #delegation_argument_linearization_challenges_ident,
                #delegation_argument_gamma_ident,
                #public_inputs_ident,
                #aux_proof_values_ident,
                #aux_boundary_values_ident,
                #memory_timestamp_high_from_sequence_idx_ident,
                #delegation_type_ident,
                #delegation_argument_interpolant_linear_coeff_ident,
            )};

            #first_row

            #one_before_last_row

            #last_row

            let mut quotient = every_row_except_last_contribution;
            quotient = quotient.mul_and_add(#cs, &#quotient_beta_ident, &every_row_except_two_last_contribution);
            quotient = quotient.mul_and_add(#cs, &#quotient_beta_ident, &first_row_contribution);
            quotient = quotient.mul_and_add(#cs, &#quotient_beta_ident, &one_before_last_row_contribution);
            quotient = quotient.mul_and_add(#cs, &#quotient_beta_ident, &last_row_contribution);
            quotient = quotient.mul_and_add(#cs, &#quotient_beta_ident, &last_row_and_zero_contribution);

            quotient
        }
    }
}
