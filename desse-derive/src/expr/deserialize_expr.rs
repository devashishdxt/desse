use proc_macro2::TokenStream;
use quote::{quote, ToTokens};
use syn::{DataEnum, DataStruct, Fields, Ident, Index};

use crate::expr::SizeExpr;

/// Helper struct for computing deserialize expression for different types
pub struct DeserializeExpr;

impl DeserializeExpr {
    /// Calculates serialize expression for fields
    fn get_deserialize_expr_for_fields<T: ToTokens, C: ToTokens>(
        container_name: T,
        init_counter: C,
        fields: &Fields,
    ) -> TokenStream {
        match fields {
            Fields::Unit => quote! { Ok(#container_name) },
            Fields::Named(named_fields) => {
                if named_fields.named.is_empty() {
                    quote! { Ok(#container_name {}) }
                } else {
                    let mut exprs = Vec::with_capacity(named_fields.named.len());
                    let mut counter = quote! { #init_counter };

                    for field in named_fields.named.iter() {
                        let field_name = match &field.ident {
                            None => unreachable!(),
                            Some(ref ident) => quote! { #ident },
                        };
                        let field_type = &field.ty;

                        exprs.push(quote! {
                            #field_name: <#field_type as DesseStatic>::deserialize_from(&*(bytes[ (#counter) .. ( #counter + <#field_type>::SIZE ) ].as_ptr() as *const [u8; <#field_type>::SIZE]))?
                        });

                        counter = quote! { #counter + <#field_type>::SIZE };
                    }

                    quote! {
                        unsafe {
                            Ok(#container_name {
                                #(#exprs),*
                            })
                        }
                    }
                }
            }
            Fields::Unnamed(unnamed_fields) => {
                if unnamed_fields.unnamed.is_empty() {
                    quote! { Ok(#container_name()) }
                } else {
                    let mut exprs = Vec::with_capacity(unnamed_fields.unnamed.len());
                    let mut counter = quote! { #init_counter };

                    for field in unnamed_fields.unnamed.iter() {
                        let field_type = &field.ty;

                        exprs.push(quote! {
                            <#field_type as DesseStatic>::deserialize_from(&*(bytes[ (#counter) .. ( #counter + <#field_type>::SIZE ) ].as_ptr() as *const [u8; <#field_type>::SIZE]))?
                        });

                        counter = quote! { #counter + <#field_type>::SIZE };
                    }

                    quote! {
                        unsafe {
                            Ok(#container_name(#(#exprs),*))
                        }
                    }
                }
            }
        }
    }

    /// Calculates size expression for [`DataStruct`](syn::DataStruct)
    pub fn for_struct(name: &Ident, struct_data: &DataStruct) -> TokenStream {
        Self::get_deserialize_expr_for_fields(name, quote! { 0 }, &struct_data.fields)
    }

    /// Calculates size expression for [`DataEnum`](syn::DataEnum)
    pub fn for_enum(name: &Ident, enum_data: &DataEnum) -> TokenStream {
        let variant_count = enum_data.variants.len();

        let size_type = SizeExpr::get_variant_count_size_type(variant_count);
        let mut match_exprs = Vec::with_capacity(variant_count);

        let variant_expr = quote! {
            let variant = unsafe { <#size_type as DesseStatic>::deserialize_from(&*(bytes[0..<#size_type>::SIZE].as_ptr() as *const [u8; <#size_type>::SIZE]))? };
        };

        for (i, variant) in enum_data.variants.iter().enumerate() {
            let variant_name = &variant.ident;
            let variant_expr = Self::get_deserialize_expr_for_fields(
                quote! { #name:: #variant_name},
                quote! { <#size_type>::SIZE },
                &variant.fields,
            );
            let index = Index::from(i);
            match_exprs.push(quote! {
                #index => #variant_expr
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
            #variant_expr
            #match_expr
        }
    }
}
