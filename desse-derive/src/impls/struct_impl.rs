use proc_macro2::TokenStream;
use quote::quote;
use syn::punctuated::Punctuated;
use syn::token::Comma;
use syn::Fields::*;
use syn::{DataStruct, Field, Type};

pub fn get_struct_desse_sized_expr(struct_data: &DataStruct) -> TokenStream {
    let types = get_struct_types(struct_data);
    get_size_expr(types)
}

fn get_struct_types(struct_data: &DataStruct) -> Vec<&Type> {
    match &struct_data.fields {
        Named(ref named_fields) => named_fields.named.iter().map(|field| &field.ty).collect(),
        Unnamed(ref unnamed_fields) => unnamed_fields
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

pub fn get_struct_desse_expr(struct_data: &DataStruct) -> (TokenStream, TokenStream) {
    let punctuated = match &struct_data.fields {
        Named(ref named_fields) => &named_fields.named,
        Unnamed(ref unnamed_fields) => &unnamed_fields.unnamed,
        Unit => panic!("Cannot implement serialize for unit struct"),
    };

    let serialize = get_serialize_expr_for_punctuated(&punctuated);
    let deserialize = get_deserialize_expr_for_punctuated(&punctuated);

    (serialize, deserialize)
}

fn get_serialize_expr_for_punctuated(punctuated: &Punctuated<Field, Comma>) -> TokenStream {
    if punctuated.is_empty() {
        panic!("Cannot implement serialize for struct with no fields");
    }

    let mut exprs = Vec::with_capacity(punctuated.len());
    let mut counter = quote! { 0 };

    for (i, field) in punctuated.iter().enumerate() {
        let name = match &field.ident {
            None => quote! { #i },
            Some(ref ident) => quote! { #ident },
        };

        let ty = &field.ty;

        exprs.push(
            quote! { (&mut bytes[ (#counter)..( #counter + <#ty>::SIZE ) ]).copy_from_slice(&Desse::serialize(&self. #name)); },
        );

        counter = quote! { #counter + <#ty>::SIZE };
    }

    quote! {
        let mut bytes: Self::Output = [0; Self::SIZE];
        #(#exprs)*
        bytes
    }
}

fn get_deserialize_expr_for_punctuated(punctuated: &Punctuated<Field, Comma>) -> TokenStream {
    if punctuated.is_empty() {
        panic!("Cannot implement serialize for struct with no fields");
    }

    let mut exprs = Vec::with_capacity(punctuated.len());
    let mut counter = quote! { 0 };

    for (i, field) in punctuated.iter().enumerate() {
        let name = match &field.ident {
            None => quote! { #i },
            Some(ref ident) => quote! { #ident },
        };

        let ty = &field.ty;

        exprs.push(
            quote! { object. #name = <#ty>::deserialize_from(&*(bytes[ (#counter) .. ( #counter + <#ty>::SIZE ) ].as_ptr() as *const [u8; <#ty>::SIZE])); },
        );

        counter = quote! { #counter + <#ty>::SIZE };
    }

    quote! {
        let mut object: Self = unsafe { std::mem::zeroed() };
        unsafe { #(#exprs)* }
        object
    }
}
