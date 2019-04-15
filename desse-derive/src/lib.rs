extern crate proc_macro;

use proc_macro::TokenStream;
use syn::{parse_macro_input, DeriveInput};

mod desse;
mod desse_sized;

mod impls;

#[proc_macro_derive(DesseSized)]
pub fn desse_sized_macro_derive(input: TokenStream) -> TokenStream {
    let input = parse_macro_input!(input as DeriveInput);
    desse_sized::get_desse_sized_impl(input).into()
}

#[proc_macro_derive(Desse)]
pub fn desse_macro_derive(input: TokenStream) -> TokenStream {
    let input = parse_macro_input!(input as DeriveInput);
    desse::get_desse_impl(input).into()
}
