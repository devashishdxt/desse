use proc_macro2::TokenStream;
use quote::quote;
use syn::punctuated::Punctuated;
use syn::{DataEnum, DataStruct, Field, Fields, Ident};

use crate::expr::SizeExpr;

/// Helper struct for computing deserialize expression for different types
pub struct DeserializeExpr;

impl DeserializeExpr {
    /// Calculates deserialize expression for punctuated fields
    fn get_deserialize_expr_for_punctuated_field<T>(
        punctuated: &Punctuated<Field, T>,
    ) -> TokenStream {
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

    /// Calculates serialize expression for fields
    fn get_deserialize_expr_for_fields(fields: &Fields) -> TokenStream {
        match fields {
            Fields::Unit => quote! { 0 },
            Fields::Named(named_fields) => {
                Self::get_deserialize_expr_for_punctuated_field(&named_fields.named)
            }
            Fields::Unnamed(unnamed_fields) => {
                Self::get_deserialize_expr_for_punctuated_field(&unnamed_fields.unnamed)
            }
        }
    }

    /// Calculates size expression for [`DataStruct`](syn::DataStruct)
    pub fn for_struct(struct_data: &DataStruct) -> TokenStream {
        Self::get_deserialize_expr_for_fields(&struct_data.fields)
    }

    /// Calculates size expression for [`DataEnum`](syn::DataEnum)
    pub fn for_enum(name: &Ident, enum_data: &DataEnum) -> TokenStream {
        let variant_count = enum_data.variants.len();

        let size_type = SizeExpr::get_variant_count_size_type(variant_count);
        let mut match_exprs = Vec::with_capacity(variant_count);

        let variant = quote! {
            let variant = <#size_type>::deserialize_from(bytes) as usize;
        };

        for (i, field) in enum_data.variants.iter().enumerate() {
            let variant_name = &field.ident;
            match_exprs.push(quote! {
                #i => #name:: #variant_name
            });
        }

        match_exprs.push(quote! {
            _ => unreachable!()
        });

        let match_expr = quote! {
            match variant {
                #(#match_exprs),*
            }
        };

        quote! {
            #variant
            #match_expr
        }
    }
}
