use proc_macro2::TokenStream;
use quote::quote;
use syn::{DataEnum, Ident};

pub fn get_enum_desse_sized_expr(enum_data: &DataEnum) -> TokenStream {
    let size_type = get_enum_size_type(enum_data);
    quote! { <#size_type>::SIZE }
}

pub fn get_enum_desse_expr(name: &Ident, enum_data: &DataEnum) -> (TokenStream, TokenStream) {
    if enum_data.variants.is_empty() {
        panic!("This macro cannot be used on empty enums!")
    }

    (
        get_enum_desse_serialize_expr(name, enum_data),
        get_enum_desse_deserialize_expr(name, enum_data),
    )
}

fn get_enum_size_type(enum_data: &DataEnum) -> TokenStream {
    let len = enum_data.variants.len();

    if len <= <u8>::max_value() as usize {
        quote! { u8 }
    } else if len <= <u16>::max_value() as usize {
        quote! { u16 }
    } else if len <= <u32>::max_value() as usize {
        quote! { u32 }
    } else if len <= <u64>::max_value() as usize {
        quote! { u64 }
    } else {
        quote! { u128 }
    }
}

fn get_enum_desse_serialize_expr(name: &Ident, enum_data: &DataEnum) -> TokenStream {
    let size_type = get_enum_size_type(enum_data);
    let mut match_exprs = Vec::with_capacity(enum_data.variants.len());

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

fn get_enum_desse_deserialize_expr(name: &Ident, enum_data: &DataEnum) -> TokenStream {
    let size_type = get_enum_size_type(enum_data);

    let variant = quote! {
        let variant = <#size_type>::deserialize_from(bytes) as usize;
    };

    let mut match_exprs = Vec::with_capacity(enum_data.variants.len());

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

// fn get_variant_expr(enum_data: &DataEnum) -> TokenStream {
//     let len = enum_data.variants.len();

//     if len <= <u8>::max_value() as usize {
//         quote! {
//             let variant = unsafe {
//                 <u8>::deserialize_from(&*(bytes[0..Self::SIZE].as_ptr() as *const [u8; Self::SIZE]))
//             } as usize;
//         }
//     } else if len <= <u16>::max_value() as usize {
//         quote! {
//             let variant = unsafe {
//                 <u16>::deserialize_from(&*(bytes[0..Self::SIZE].as_ptr() as *const [u8; Self::SIZE]))
//             } as usize;
//         }
//     } else if len <= <u32>::max_value() as usize {
//         quote! {
//             let variant = unsafe {
//                 <u32>::deserialize_from(&*(bytes[0..Self::SIZE].as_ptr() as *const [u8; Self::SIZE]))
//             } as usize;
//         }
//     } else if len <= <u64>::max_value() as usize {
//         quote! {
//             let variant = unsafe {
//                 <u64>::deserialize_from(&*(bytes[0..Self::SIZE].as_ptr() as *const [u8; Self::SIZE]))
//             } as usize;
//         }
//     } else {
//         quote! {
//             let variant = unsafe {
//                 <u128>::deserialize_from(&*(bytes[0..Self::SIZE].as_ptr() as *const [u8; Self::SIZE]))
//             } as usize;
//         }
//     }
// }
