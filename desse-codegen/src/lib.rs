extern crate proc_macro;
extern crate syn;

use proc_macro::TokenStream;
use quote::quote;
use syn::Item;

#[proc_macro_attribute]
pub fn desse(_attr: TokenStream, item: TokenStream) -> TokenStream {
    let item: Item = syn::parse(item).expect("Failed to parse input");
    let output = quote! { #item };
    output.into()
}
