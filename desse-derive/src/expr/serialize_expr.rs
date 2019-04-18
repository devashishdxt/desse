use std::fmt::Display;
use std::str::FromStr;

use proc_macro2::{Span, TokenStream};
use quote::{quote, ToTokens};
use syn::{DataEnum, DataStruct, Fields, Ident, Index};

use crate::expr::SizeExpr;

/// Helper struct for computing serialize expression for different types
pub struct SerializeExpr;

impl SerializeExpr {
    /// Calculates serialize expression for fields
    fn get_serialize_expr_for_fields<T: ToTokens + Display, C: ToTokens>(
        container_prefix: T,
        init_counter: C,
        fields: &Fields,
    ) -> TokenStream {
        match fields {
            Fields::Unit => quote! {},
            Fields::Named(named_fields) => {
                let mut exprs = Vec::with_capacity(named_fields.named.len());
                let mut counter = quote! { #init_counter };

                for field in named_fields.named.iter() {
                    let field_name = match &field.ident {
                        None => unreachable!(),
                        Some(ref ident) => quote! { #ident },
                    };
                    let field_type = &field.ty;

                    let field_ref =
                        TokenStream::from_str(&format!("{}{}", container_prefix, field_name))
                            .unwrap();

                    exprs.push(quote! {
                        (&mut bytes[ (#counter)..( #counter + <#field_type>::SIZE ) ]).copy_from_slice(&Desse::serialize(#field_ref));
                    });

                    counter = quote! { #counter + <#field_type>::SIZE };
                }

                quote! { #(#exprs)* }
            }
            Fields::Unnamed(unnamed_fields) => {
                let mut exprs = Vec::with_capacity(unnamed_fields.unnamed.len());
                let mut counter = quote! { #init_counter };

                for (i, field) in unnamed_fields.unnamed.iter().enumerate() {
                    let field_type = &field.ty;

                    let field_ref =
                        TokenStream::from_str(&format!("{}{}", container_prefix, i)).unwrap();

                    exprs.push(quote! {
                        (&mut bytes[ (#counter)..( #counter + <#field_type>::SIZE ) ]).copy_from_slice(&Desse::serialize(#field_ref));
                    });

                    counter = quote! { #counter + <#field_type>::SIZE };
                }

                quote! { #(#exprs)* }
            }
        }
    }

    /// Calculates  expression for [`DataStruct`](syn::DataStruct)
    pub fn for_struct(_: &Ident, struct_data: &DataStruct) -> TokenStream {
        Self::get_serialize_expr_for_fields(quote! { &self. }, quote! { 0 }, &struct_data.fields)
    }

    /// Calculates serialize expression for [`DataEnum`](syn::DataEnum)
    pub fn for_enum(name: &Ident, enum_data: &DataEnum) -> TokenStream {
        let variant_count = enum_data.variants.len();

        let size_type = SizeExpr::get_variant_count_size_type(variant_count);
        let mut match_exprs = Vec::with_capacity(variant_count);

        for (i, variant) in enum_data.variants.iter().enumerate() {
            let index = Index::from(i);

            let field_prefix = match variant.fields {
                Fields::Unit => quote! {},
                Fields::Named(_) => quote! {},
                Fields::Unnamed(_) => quote! { __desse_ },
            };

            let fields_expr = match &variant.fields {
                Fields::Unit => quote! {},
                Fields::Named(named_fields) => {
                    let mut exprs = Vec::with_capacity(named_fields.named.len());

                    for field in named_fields.named.iter() {
                        let field_name = match &field.ident {
                            None => unreachable!(),
                            Some(ref ident) => quote! { #ident },
                        };

                        exprs.push(quote! { ref #field_name })
                    }

                    quote! { { #(#exprs),* } }
                }
                Fields::Unnamed(unnamed_fields) => {
                    let len = unnamed_fields.unnamed.len();
                    let mut exprs = Vec::with_capacity(len);

                    for j in 0..len {
                        let desse_name =
                            Ident::new(&format!("{}{}", field_prefix, j), Span::call_site());
                        exprs.push(quote! { ref #desse_name });
                    }

                    quote! { ( #(#exprs),* ) }
                }
            };

            let variant_name = &variant.ident;
            let variant_init_expr = quote! {
                (&mut bytes[0..<#size_type>::SIZE]).copy_from_slice(&Desse::serialize(&(#index as #size_type)));
            };
            let variant_impl_expr = Self::get_serialize_expr_for_fields(
                field_prefix,
                quote! { <#size_type>::SIZE },
                &variant.fields,
            );

            let variant_expr = quote! {
                #variant_init_expr
                #variant_impl_expr
            };

            match_exprs.push(quote! {
                #name:: #variant_name #fields_expr => { #variant_expr }
            });
        }

        quote! {
            match self {
                #(#match_exprs),*
            }
        }
    }
}
