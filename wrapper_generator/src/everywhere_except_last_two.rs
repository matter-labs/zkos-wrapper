use crate::quotient_generator::Idents;
use crate::utils::read_value_expr;

use proc_macro2::TokenStream;
use quote::quote;

use prover::cs::one_row_compiler::{
    ColumnAddress, ShuffleRamAuxComparisonSet, ShuffleRamInitAndTeardownLayout,
};

// use crate::utils::*;

pub(crate) fn transform_shuffle_ram_lazy_init(
    shuffle_ram_inits_and_teardowns: ShuffleRamInitAndTeardownLayout,
    lazy_init_address_aux_vars: &ShuffleRamAuxComparisonSet,
    idents: &Idents,
) -> Vec<TokenStream> {
    let Idents {
        individual_term_ident,
        ..
    } = idents;

    let mut streams = vec![];

    // first lazy init from read set / lazy teardown

    let lazy_init_address_start = shuffle_ram_inits_and_teardowns
        .lazy_init_addesses_columns
        .start();

    // two constraints to compare sorting of lazy init
    {
        let comparison_aux_vars = lazy_init_address_aux_vars;
        let lazy_init_address_low = lazy_init_address_start;
        let lazy_init_address_high = lazy_init_address_start + 1;
        let lazy_init_address_low_place = ColumnAddress::MemorySubtree(lazy_init_address_low);
        let lazy_init_address_high_place = ColumnAddress::MemorySubtree(lazy_init_address_high);

        let ShuffleRamAuxComparisonSet {
            aux_low_high: [address_aux_low, address_aux_high],
            borrow,
        } = *comparison_aux_vars;
        // first we do low: this - next with borrow
        let this_low_expr = read_value_expr(lazy_init_address_low_place, idents, false);
        let next_low_expr = read_value_expr(lazy_init_address_low_place, idents, true);
        let aux_low_expr = read_value_expr(address_aux_low, idents, false);
        let borrow_value_expr = read_value_expr(borrow, idents, false);

        let this_high_expr = read_value_expr(lazy_init_address_high_place, idents, false);
        let next_high_expr = read_value_expr(lazy_init_address_high_place, idents, true);
        let aux_high_expr = read_value_expr(address_aux_high, idents, false);

        let t = quote! {
            let #individual_term_ident = {
                let borrow_value = #borrow_value_expr;
                let this_low = #this_low_expr;
                let next_low = #next_low_expr;
                let aux_low = #aux_low_expr;
                let shift = MersenneField::allocate_constant(cs, Mersenne31Field(1 << 16));

                let mut #individual_term_ident = borrow_value;
                #individual_term_ident = #individual_term_ident.mul_by_base_and_add(cs, &shift, &this_low);
                #individual_term_ident = #individual_term_ident.sub(cs, &next_low);
                #individual_term_ident = #individual_term_ident.sub(cs, &aux_low);

                #individual_term_ident
            };
        };

        streams.push(t);

        let t = quote! {
            let #individual_term_ident = {
                let borrow_value = #borrow_value_expr;
                let this_high = #this_high_expr;
                let next_high = #next_high_expr;
                let aux_high = #aux_high_expr;
                let shift = MersenneField::allocate_constant(cs, Mersenne31Field(1 << 16));

                let mut #individual_term_ident = this_high;
                #individual_term_ident = #individual_term_ident.add_base(cs, &shift);
                #individual_term_ident = #individual_term_ident.sub(cs, &borrow_value);
                #individual_term_ident = #individual_term_ident.sub(cs, &next_high);
                #individual_term_ident = #individual_term_ident.sub(cs, &aux_high);

                #individual_term_ident
            };
        };

        streams.push(t);
    }

    streams
}

pub(crate) fn transform_linking_constraints(
    state_linkage_constraints: &[(ColumnAddress, ColumnAddress)],
    idents: &Idents,
) -> Vec<TokenStream> {
    let Idents {
        individual_term_ident,
        ..
    } = idents;

    let mut streams = vec![];

    // linking constraints
    for (src, dst) in state_linkage_constraints.iter() {
        let this_row_expr = read_value_expr(*src, idents, false);
        let next_row_expr = read_value_expr(*dst, idents, true);

        let t = quote! {
            let #individual_term_ident = {
                let mut #individual_term_ident = #this_row_expr;
                let t = #next_row_expr;
                #individual_term_ident = #individual_term_ident.sub(cs, &t);

                #individual_term_ident
            };
        };

        streams.push(t);
    }

    streams
}
