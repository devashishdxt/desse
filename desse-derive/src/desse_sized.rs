use proc_macro2::TokenStream;
use quote::quote;
use syn::Data::*;
use syn::Fields::*;
use syn::{DataEnum, DataStruct, DataUnion, DeriveInput, Type};

pub fn get_desse_sized_impl(input: DeriveInput) -> TokenStream {
    let name = input.ident;

    let types: Vec<&Type> = match &input.data {
        Struct(ref struct_data) => get_struct_types(struct_data),
        Enum(ref enum_data) => get_enum_types(enum_data),
        Union(ref union_data) => get_union_types(union_data),
    };

    let expr = get_size_expr(types);

    let output = quote! {
        #[automatically_derived]
        #[allow(unused_qualifications)]
        impl DesseSized for #name {
            const SIZE: usize = #expr;
        }
    };

    output
}

fn get_enum_types(_enum_data: &DataEnum) -> Vec<&Type> {
    panic!("This macro cannot be used on enums!")
}

fn get_union_types(_union_data: &DataUnion) -> Vec<&Type> {
    panic!("This macro cannot be used on unions!")
}

fn get_struct_types(struct_data: &DataStruct) -> Vec<&Type> {
    match &struct_data.fields {
        Named(named_fields) => named_fields.named.iter().map(|field| &field.ty).collect(),
        Unnamed(unnamed_fields) => unnamed_fields
            .unnamed
            .iter()
            .map(|field| &field.ty)
            .collect(),
        Unit => Vec::default(),
    }
}

fn get_size_expr(types: Vec<&Type>) -> TokenStream {
    if types.is_empty() {
        quote! { 0 }
    } else {
        quote! { #(<#types>::SIZE)+* }
    }
}
