use crate::quotient_generator::*;
use crate::utils::*;

use quote::quote;

use prover::cs::one_row_compiler::{
    BoundaryConstraintLocation, ColumnAddress, LookupAndMemoryArgumentLayout, MemorySubtree,
};

use proc_macro2::TokenStream;

pub(crate) fn transform_first_or_last_rows(
    memory_layout: &MemorySubtree,
    stage_2_layout: &LookupAndMemoryArgumentLayout,
    public_inputs: &[(BoundaryConstraintLocation, ColumnAddress)],
    idents: &Idents,
) -> (TokenStream, TokenStream, TokenStream, TokenStream) {
    // first lazy init, then public inputs

    let Idents {
        random_point_ident,
        individual_term_ident,
        public_inputs_ident,
        aux_proof_values_ident,
        aux_boundary_values_ident,
        delegation_argument_interpolant_linear_coeff_ident,
        ..
    } = idents;

    let mut first_row_boundary_constraints = vec![];
    let mut one_before_last_row_boundary_constraints = vec![];

    if let Some(shuffle_ram_inits_and_teardowns) = memory_layout.shuffle_ram_inits_and_teardowns {
        let lazy_init_address_start = shuffle_ram_inits_and_teardowns
            .lazy_init_addesses_columns
            .start();

        first_row_boundary_constraints.push((
            ColumnAddress::MemorySubtree(lazy_init_address_start),
            quote! {
                #aux_boundary_values_ident.lazy_init_first_row[0]
            },
        ));
        first_row_boundary_constraints.push((
            ColumnAddress::MemorySubtree(lazy_init_address_start + 1),
            quote! {
                #aux_boundary_values_ident.lazy_init_first_row[1]
            },
        ));

        one_before_last_row_boundary_constraints.push((
            ColumnAddress::MemorySubtree(lazy_init_address_start),
            quote! {
                #aux_boundary_values_ident.lazy_init_one_before_last_row[0]
            },
        ));
        one_before_last_row_boundary_constraints.push((
            ColumnAddress::MemorySubtree(lazy_init_address_start + 1),
            quote! {
                #aux_boundary_values_ident.lazy_init_one_before_last_row[1]
            },
        ));
    }

    for (i, (location, column_address)) in public_inputs.iter().enumerate() {
        match location {
            BoundaryConstraintLocation::FirstRow => {
                first_row_boundary_constraints.push((*column_address, quote! {
                    #public_inputs_ident[#i]
                }));
            }
            BoundaryConstraintLocation::OneBeforeLastRow => {
                one_before_last_row_boundary_constraints.push((*column_address, quote! {
                    #public_inputs_ident[#i]
                }));
            }
            BoundaryConstraintLocation::LastRow => {
                panic!("public inputs on the last row are not supported");
            }
        }
    }

    let first_row = {
        let mut streams = vec![];

        for (_i, (place, expected_value)) in first_row_boundary_constraints.iter().enumerate() {
            let value_expr = read_value_expr(*place, idents, false);

            let t = quote! {
                let #individual_term_ident = {
                    let mut #individual_term_ident = #value_expr;
                    let t = #expected_value;
                    #individual_term_ident = #individual_term_ident.sub_base(cs, &t);

                    #individual_term_ident
                };
            };

            streams.push(t);
        }

        // 1 constraint for memory accumulator initial value == 1
        {
            let num_memory_accumulators = stage_2_layout
                .intermediate_polys_for_memory_argument
                .num_elements();
            let offset = stage_2_layout
                .get_intermediate_polys_for_memory_argument_absolute_poly_idx_for_verifier(
                    num_memory_accumulators - 1,
                );
            let value_expr = read_stage_2_value_expr(offset, idents, false);

            let t = quote! {
                let #individual_term_ident = {
                    let one = MersenneField::one(cs);
                    let mut #individual_term_ident = #value_expr;
                    #individual_term_ident = #individual_term_ident.sub_base(cs, &one);

                    #individual_term_ident
                };
            };

            streams.push(t);
        }

        // merge right here
        let mut stream = TokenStream::new();
        let mut is_first = true;

        let contribution = accumulate_contributions(&mut is_first, None, streams, &idents);
        stream.extend(contribution);

        stream
    };

    let one_before_last_row = {
        let mut streams = vec![];

        for (_i, (place, expected_value)) in
            one_before_last_row_boundary_constraints.iter().enumerate()
        {
            let value_expr = read_value_expr(*place, idents, false);

            let t = quote! {
                let #individual_term_ident = {
                    let mut #individual_term_ident = #value_expr;
                    let t = #expected_value;
                    #individual_term_ident = #individual_term_ident.sub_base(cs, &t);

                    #individual_term_ident
                };
            };

            streams.push(t);
        }

        let mut stream = TokenStream::new();
        let mut is_first = true;

        let contribution = accumulate_contributions(&mut is_first, None, streams, &idents);
        stream.extend(contribution);

        stream
    };

    let last_row = {
        let mut streams = vec![];

        let num_memory_accumulators = stage_2_layout
            .intermediate_polys_for_memory_argument
            .num_elements();
        let offset = stage_2_layout
            .get_intermediate_polys_for_memory_argument_absolute_poly_idx_for_verifier(
                num_memory_accumulators - 1,
            );
        let value_expr = read_stage_2_value_expr(offset, idents, false);

        let t = quote! {
            let #individual_term_ident = {
                let mut #individual_term_ident = #value_expr;
                let t = #aux_proof_values_ident.memory_grand_product_accumulator_final_value;
                #individual_term_ident = #individual_term_ident.sub(cs, &t);

                #individual_term_ident
            };
        };

        streams.push(t);

        let mut stream = TokenStream::new();
        let mut is_first = true;

        let contribution = accumulate_contributions(&mut is_first, None, streams, &idents);
        stream.extend(contribution);

        stream
    };

    let last_row_and_zero = {
        let mut streams = vec![];

        // range checks
        {
            {
                let offset = stage_2_layout
                    .range_check_16_intermediate_poly_for_multiplicities_absolute_poly_idx_for_verifier();
                let multiplicities_acc_expr = read_stage_2_value_expr(offset, idents, false);

                let mut substream = quote! {
                    let mut #individual_term_ident = #multiplicities_acc_expr;
                };

                let num_pairs = stage_2_layout
                    .intermediate_polys_for_range_check_16
                    .num_pairs;

                for i in 0..num_pairs {
                    let offset = stage_2_layout
                        .intermediate_polys_for_range_check_16
                        .get_ext4_poly_index_in_openings(i, stage_2_layout);
                    let el_expr = read_stage_2_value_expr(offset, idents, false);

                    let t = quote! {
                        let t = #el_expr;
                        #individual_term_ident = #individual_term_ident.sub(cs, &t);
                    };
                    substream.extend(t);
                }

                if let Some(lazy_init_address_range_check_16) =
                    stage_2_layout.lazy_init_address_range_check_16
                {
                    let offset = lazy_init_address_range_check_16
                        .get_ext4_poly_index_in_openings(0, stage_2_layout);
                    let el_expr = read_stage_2_value_expr(offset, idents, false);

                    let t = quote! {
                        let t = #el_expr;
                        #individual_term_ident = #individual_term_ident.sub(cs, &t);
                    };
                    substream.extend(t);
                }

                if let Some(_remainder) = stage_2_layout.remainder_for_range_check_16 {
                    todo!();
                }

                let t = quote! {
                    let #individual_term_ident = {
                        #substream

                        #individual_term_ident
                    };
                };

                streams.push(t);
            }
        }

        // generic lookup
        if stage_2_layout
            .intermediate_polys_for_generic_multiplicities
            .num_elements()
            > 0
        {
            let bound = stage_2_layout
                .intermediate_polys_for_generic_multiplicities
                .num_elements();
            let offset = stage_2_layout
                .generic_width_3_lookup_intermediate_polys_for_multiplicities_absolute_poly_idx_for_verifier(0);
            let multiplicities_acc_expr = read_stage_2_value_expr(offset, idents, false);

            let mut substream = quote! {
                let mut #individual_term_ident = #multiplicities_acc_expr;
            };

            for i in 1..bound {
                let offset = stage_2_layout
                    .generic_width_3_lookup_intermediate_polys_for_multiplicities_absolute_poly_idx_for_verifier(i);
                let multiplicities_acc_expr = read_stage_2_value_expr(offset, idents, false);
                let t = quote! {
                    let t = #multiplicities_acc_expr;
                    #individual_term_ident = #individual_term_ident.add(cs, &t);
                };
                substream.extend(t);
            }

            for i in 0..stage_2_layout
                .intermediate_polys_for_generic_lookup
                .num_elements()
            {
                let offset = stage_2_layout
                    .get_intermediate_polys_for_generic_lookup_absolute_poly_idx_for_verifier(i);
                let el_expr = read_stage_2_value_expr(offset, idents, false);

                let t = quote! {
                    let t = #el_expr;
                    #individual_term_ident = #individual_term_ident.sub(cs, &t);
                };
                substream.extend(t);
            }

            let t = quote! {
                let #individual_term_ident = {
                    #substream

                    #individual_term_ident
                };
            };

            streams.push(t);
        }

        // and delegation creation/processing
        if memory_layout.delegation_request_layout.is_some()
            || memory_layout.delegation_processor_layout.is_some()
        {
            // we need to show the sum of the values everywhere except the last row,
            // so we show that intermediate poly - interpolant((0, 0), (omega^-1, `value``)) is divisible
            // by our selected divisor

            // interpolant is literaly 1/omega^-1 * value * X (as one can see it's 0 at 0 and `value` at omega^-1)
            let acc = stage_2_layout
                .get_aux_polys_for_gelegation_argument_absolute_poly_idx_for_verifier()
                .expect("must exist");
            let accumulator_expr = read_stage_2_value_expr(acc, idents, false);

            let t = quote! {
                let #individual_term_ident = {
                    let mut #individual_term_ident = #accumulator_expr;
                    // coeff should be accumulator value / omega^-1
                    let mut t = #random_point_ident;

                    // TODO: can be optimized (mul + sub)
                    t = t.mul(cs, & #delegation_argument_interpolant_linear_coeff_ident);
                    #individual_term_ident = #individual_term_ident.sub(cs, &t);

                    #individual_term_ident
                };
            };

            streams.push(t);
        }

        let mut stream = TokenStream::new();
        let mut is_first = true;

        let contribution = accumulate_contributions(&mut is_first, None, streams, &idents);
        stream.extend(contribution);

        stream
    };

    (first_row, one_before_last_row, last_row, last_row_and_zero)
}
