use proc_macro2::TokenStream;
use quote::quote;
use syn::Data::*;
use syn::DeriveInput;

use crate::impls::enum_impl::*;
use crate::impls::struct_impl::*;

pub fn get_desse_sized_impl(input: DeriveInput) -> TokenStream {
    let name = input.ident;

    let expr = match &input.data {
        Struct(ref struct_data) => get_struct_desse_sized_expr(struct_data),
        Enum(ref enum_data) => get_enum_desse_sized_expr(enum_data),
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
