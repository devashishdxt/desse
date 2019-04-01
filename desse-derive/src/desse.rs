use proc_macro2::TokenStream;
use quote::quote;
use syn::punctuated::Punctuated;
use syn::token::Comma;
use syn::Data::*;
use syn::Fields::*;
use syn::{DataEnum, DataStruct, DataUnion, DeriveInput, Field};

pub fn get_desse_impl(input: DeriveInput) -> TokenStream {
    let name = input.ident;

    let (serialize, deserialize) = match &input.data {
        Struct(ref struct_data) => get_struct_expr(&struct_data),
        Enum(ref enum_data) => get_enum_expr(&enum_data),
        Union(ref union_data) => get_union_expr(&union_data),
    };

    quote! {
        #[automatically_derived]
        #[allow(unused_qualifications)]
        impl Desse for #name {
            type Output = [u8; Self::SIZE];

            #[inline(always)]
            fn serialize(&self) -> Self::Output {
                let mut bytes: Self::Output = [0; Self::SIZE];

                #serialize

                bytes
            }

            #[inline(always)]
            fn deserialize_from(bytes: &Self::Output) -> Self {
                let mut object: Self = unsafe { std::mem::zeroed() };

                #deserialize

                object
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

fn get_struct_expr(struct_data: &DataStruct) -> (TokenStream, TokenStream) {
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
            quote! { (&mut bytes[ (#counter)..( #counter + <#ty>::SIZE ) ]).copy_from_slice(&self. #name .serialize()); },
        );

        counter = quote! { #counter + <#ty>::SIZE };
    }

    quote! { #(#exprs)* }
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

    quote! { unsafe { #(#exprs)* } }
}
