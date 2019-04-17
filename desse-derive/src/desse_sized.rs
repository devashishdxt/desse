use proc_macro2::TokenStream;
use quote::quote;
use syn::Data::*;
use syn::DeriveInput;

use crate::expr::SizeExpr;

/// Returns `DesseSized` trait implementation
pub fn get_desse_sized_impl(input: DeriveInput) -> TokenStream {
    let name = input.ident;

    let expr = match &input.data {
        Struct(ref struct_data) => SizeExpr::for_struct(struct_data),
        Enum(ref enum_data) => SizeExpr::for_enum(enum_data),
        Union(_) => panic!("This macro cannot be used on unions!"),
    };

    quote! {
        #[automatically_derived]
        #[allow(unused_qualifications)]
        impl DesseSized for #name {
            const SIZE: usize = #expr;
        }
    }
}
