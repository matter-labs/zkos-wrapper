use super::quotient_generator::*;

use proc_macro2::TokenStream;
use quote::quote;

use prover::cs::one_row_compiler::ColumnAddress;

pub(crate) fn read_value_expr(
    column: ColumnAddress,
    idents: &Idents,
    use_next_row: bool,
) -> TokenStream {
    match column {
        ColumnAddress::WitnessSubtree(offset) => {
            let ident = if use_next_row == false {
                &idents.witness_values_ident
            } else {
                &idents.witness_values_next_row_ident
            };

            quote! {
                *(#ident.get_unchecked(#offset))
            }
        }
        ColumnAddress::MemorySubtree(offset) => {
            let ident = if use_next_row == false {
                &idents.memory_values_ident
            } else {
                &idents.memory_values_next_row_ident
            };

            quote! {
                *(#ident.get_unchecked(#offset))
            }
        }
        ColumnAddress::SetupSubtree(offset) => {
            assert!(use_next_row == false);
            let ident = &idents.setup_values_ident;

            quote! {
                *(#ident.get_unchecked(#offset))
            }
        }
        ColumnAddress::OptimizedOut(..) => {
            unreachable!("quotient must not use `optimized out` variables");
        }
    }
}

pub(crate) fn read_stage_2_value_expr(
    offset: usize,
    idents: &Idents,
    use_next_row: bool,
) -> TokenStream {
    let ident = if use_next_row == false {
        &idents.stage_2_values_ident
    } else {
        &idents.stage_2_values_next_row_ident
    };

    quote! {
        *(#ident.get_unchecked(#offset))
    }
}

pub(crate) fn accumulate_contributions(
    is_first: &mut bool,
    common_stream_for_terms: Option<TokenStream>,
    individual_term_streams: Vec<TokenStream>,
    idents: &Idents,
) -> TokenStream {
    if individual_term_streams.is_empty() {
        assert!(common_stream_for_terms.is_none());

        return TokenStream::new();
    }

    if let Some(common_stream_for_terms) = common_stream_for_terms {
        // assume not first
        assert!(*is_first == false, "alternative mode is unsupported");
        let mut inner_stream = TokenStream::new();
        for el in individual_term_streams.into_iter() {
            let t = accumulate_contribution(is_first, el, idents);
            inner_stream.extend(t);
        }
        quote! {
            {
                #common_stream_for_terms

                #inner_stream
            }
        }
    } else {
        let mut stream = TokenStream::new();
        for el in individual_term_streams.into_iter() {
            let t = accumulate_contribution(is_first, el, idents);
            stream.extend(t);
        }

        stream
    }
}

fn accumulate_contribution(
    is_first: &mut bool,
    individual_term_stream: TokenStream,
    idents: &Idents,
) -> TokenStream {
    let Idents {
        individual_term_ident,
        terms_accumulator_ident,
        quotient_alpha_ident,
        ..
    } = idents;

    if *is_first {
        *is_first = false;

        quote! {
            let mut #terms_accumulator_ident = {
                #individual_term_stream

                #individual_term_ident
            };
        }
    } else {
        quote! {
            {
                let contribution = {
                    #individual_term_stream

                    #individual_term_ident
                };
                // #terms_accumulator_ident = #terms_accumulator_ident.mul(cs, & #quotient_alpha_ident);
                // #terms_accumulator_ident = #terms_accumulator_ident.add(cs, &contribution);
                #terms_accumulator_ident = #terms_accumulator_ident.mul_and_add(cs, & #quotient_alpha_ident, &contribution);
            }
        }
    }
}
