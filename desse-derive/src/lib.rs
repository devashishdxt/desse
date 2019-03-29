extern crate proc_macro;

use proc_macro::TokenStream;
use syn::{parse_macro_input, DeriveInput};

mod desse_sized;

#[proc_macro_derive(DesseSized)]
pub fn desse_sized_macro_derive(input: TokenStream) -> TokenStream {
    let input = parse_macro_input!(input as DeriveInput);
    desse_sized::get_desse_sized_impl(input).into()
}
