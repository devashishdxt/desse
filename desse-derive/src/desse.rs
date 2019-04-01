use proc_macro2::TokenStream;
use quote::quote;
use syn::Data::*;
use syn::{DataEnum, DataStruct, DataUnion, DeriveInput};

pub fn get_desse_impl(input: DeriveInput) -> TokenStream {
    let name = input.ident;

    let (serialize, deserialize) = match &input.data {
        Struct(ref struct_data) => get_struct_expr(struct_data),
        Enum(ref enum_data) => get_enum_expr(enum_data),
        Union(ref union_data) => get_union_expr(union_data),
    };

    quote! {
        #[automatically_derived]
        #[allow(unused_qualifications)]
        impl Desse for #name {
            type Output = [u8; <#name>::SIZE];

            fn serialize(&self) -> Self::Output {
                #serialize
            }

            fn deserialize_from(bytes: &Self::Output) -> Self {
                #deserialize
            }
        }
    }
}

fn get_enum_expr(_enum_data: &DataEnum) -> (TokenStream, TokenStream) {
    panic!("This macro cannot be used on enums!")
}

fn get_union_expr(_union_data: &DataUnion) -> (TokenStream, TokenStream) {
    panic!("This macro cannot be used on unions!")
}

fn get_struct_expr(_struct_data: &DataStruct) -> (TokenStream, TokenStream) {
    unimplemented!()
}
