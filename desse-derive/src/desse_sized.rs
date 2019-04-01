use proc_macro2::TokenStream;
use quote::quote;
use syn::Data::*;
use syn::Fields::*;
use syn::{DataEnum, DataStruct, DataUnion, DeriveInput, Type};

pub fn get_desse_sized_impl(input: DeriveInput) -> TokenStream {
    let name = input.ident;

    let expr = match &input.data {
        Struct(ref struct_data) => get_struct_expr(struct_data),
        Enum(ref enum_data) => get_enum_expr(enum_data),
        Union(ref union_data) => get_union_expr(union_data),
    };

    quote! {
        #[automatically_derived]
        #[allow(unused_qualifications)]
        impl DesseSized for #name {
            const SIZE: usize = #expr;
        }
    }
}

fn get_enum_expr(_enum_data: &DataEnum) -> TokenStream {
    panic!("This macro cannot be used on enums!")
}

fn get_union_expr(_union_data: &DataUnion) -> TokenStream {
    panic!("This macro cannot be used on unions!")
}

fn get_struct_expr(struct_data: &DataStruct) -> TokenStream {
    let types = get_struct_types(struct_data);
    get_size_expr(types)
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
