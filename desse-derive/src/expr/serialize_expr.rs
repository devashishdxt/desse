use proc_macro2::TokenStream;
use quote::quote;
use syn::punctuated::Punctuated;
use syn::{DataEnum, DataStruct, Field, Fields, Ident};

use crate::expr::SizeExpr;

/// Helper struct for computing serialize expression for different types
pub struct SerializeExpr;

impl SerializeExpr {
    /// Calculates serialize expression for punctuated fields
    fn get_serialize_expr_for_punctuated_field<T>(
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

    /// Calculates serialize expression for fields
    fn get_serialize_expr_for_fields(fields: &Fields) -> TokenStream {
        match fields {
            Fields::Unit => quote! { 0 },
            Fields::Named(named_fields) => {
                Self::get_serialize_expr_for_punctuated_field(&named_fields.named)
            }
            Fields::Unnamed(unnamed_fields) => {
                Self::get_serialize_expr_for_punctuated_field(&unnamed_fields.unnamed)
            }
        }
    }

    /// Calculates  expression for [`DataStruct`](syn::DataStruct)
    pub fn for_struct(struct_data: &DataStruct) -> TokenStream {
        Self::get_serialize_expr_for_fields(&struct_data.fields)
    }

    /// Calculates serialize expression for [`DataEnum`](syn::DataEnum)
    ///
    /// # Todo
    ///
    /// Currently, this function takes `name` as input for referencing different variants of enum, for example,
    /// `MyEnum::Variant1`. The need for this will go away in future once
    /// [`type_alias_enum_variants`](https://github.com/rust-lang/rust/issues/49683) lands in stable which will enable
    /// us to write `Self::Variant1` instead.
    pub fn for_enum(name: &Ident, enum_data: &DataEnum) -> TokenStream {
        let variant_count = enum_data.variants.len();

        let size_type = SizeExpr::get_variant_count_size_type(variant_count);
        let mut match_exprs = Vec::with_capacity(variant_count);

        for (i, field) in enum_data.variants.iter().enumerate() {
            let variant_name = &field.ident;
            match_exprs.push(quote! {
                #name:: #variant_name  => (#i as #size_type).serialize()
            });
        }

        quote! {
            match self {
                #(#match_exprs),*
            }
        }
    }
}
