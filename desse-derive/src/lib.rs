extern crate proc_macro;

use proc_macro::TokenStream;

#[proc_macro_derive(Desse)]
pub fn desse_macro_derive(_input: TokenStream) -> TokenStream {
    panic!("This is not yet implemented");
}
