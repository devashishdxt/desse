use proc_macro2::TokenStream;
use quote::quote;
use syn::Data::*;
use syn::DeriveInput;

use crate::expr::{DeserializeExpr, SerializeExpr};

/// Returns `Desse` trait implementation
pub fn get_desse_impl(input: DeriveInput) -> TokenStream {
    let name = input.ident;

    let (serialize, deserialize) = match &input.data {
        Struct(ref struct_data) => (
            SerializeExpr::for_struct(&struct_data),
            DeserializeExpr::for_struct(&struct_data),
        ),
        Enum(ref enum_data) => (
            SerializeExpr::for_enum(&name, &enum_data),
            DeserializeExpr::for_enum(&name, &enum_data),
        ),
        Union(_) => panic!("This macro cannot be used on unions!"),
    };

    quote! {
        #[automatically_derived]
        #[allow(unused_qualifications)]
        impl Desse for #name {
            type Output = [u8; Self::SIZE];

            #[inline]
            fn serialize(&self) -> Self::Output {
                #serialize
            }

            #[inline]
            fn deserialize_from(bytes: &Self::Output) -> Self {
                #deserialize
            }
        }
    }
}
