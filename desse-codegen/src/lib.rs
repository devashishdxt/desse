extern crate proc_macro;
extern crate syn;

use syn::Item;
use proc_macro::TokenStream;

#[proc_macro_attribute]
pub fn desse(attr: TokenStream, item: TokenStream) -> TokenStream {
    let item: Item = syn::parse(item).expect("Failed to parse input");
    
    unimplemented!()
}
