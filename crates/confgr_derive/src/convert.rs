#![allow(non_snake_case)]
use crate::{get_ident_from_type, ConfigAttributes, SUFFIX};
use proc_macro2::TokenStream;
use quote::{format_ident, quote};
use syn::{Ident, Type};

pub fn generate_conversion_impl(
    name: &Ident,
    field_data: &[(&Ident, &Type, ConfigAttributes)],
) -> TokenStream {
    let BASE_PARAMETER: Ident = format_ident!("{}", "base");
    let LAYER_PARAMETER: Ident = format_ident!("{}", "layer");

    let layer_name = format_ident!("{}{}", name, SUFFIX);

    let from_layer_conversions = field_data.iter().map(|(field_name, ty, attr)| {
        if attr.nest {
            let nested_type = get_ident_from_type(ty);
            let nested_layer_name = format_ident!("{}{}", nested_type, SUFFIX);
            quote! {
                #field_name: <#nested_type as ::core::convert::From<#nested_layer_name>>::from(#LAYER_PARAMETER.#field_name),
            }
        } else {
            quote! {
                #field_name: #LAYER_PARAMETER.#field_name.unwrap_or_default(),
            }
        }
    });

    let from_base_conversions = field_data.iter().map(|(field_name, ty, attr)| {
        if attr.nest {
            let nested_type = get_ident_from_type(ty);
            let nested_layer_name = format_ident!("{}{}", nested_type, SUFFIX);
            quote! {
                #field_name: <#nested_layer_name as ::core::convert::From<#nested_type>>::from(#BASE_PARAMETER.#field_name),
            }
        } else {
            quote! {
                #field_name: Some(#BASE_PARAMETER.#field_name),
            }
        }
    });

    quote! {
        impl ::core::convert::From<#layer_name> for #name {
            fn from(#LAYER_PARAMETER: #layer_name) -> Self {
                Self {
                    #( #from_layer_conversions )*
                }
            }
        }

        impl ::core::convert::From<#name> for #layer_name {
            fn from(#BASE_PARAMETER: #name) -> Self {
                Self {
                    #( #from_base_conversions )*
                }
            }
        }


    }
}
