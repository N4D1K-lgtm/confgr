use crate::{get_ident_from_type, ConfigAttributes, SUFFIX};
use proc_macro2::TokenStream;
use quote::{format_ident, quote};
use syn::{Ident, Type};

pub fn generate_layer(
    name: &Ident,
    field_data: &[(&Ident, &Type, ConfigAttributes)],
) -> TokenStream {
    let layer_name = format_ident!("{}{}", name, SUFFIX);

    let (field_defs, empty_defs, merges) = field_data.iter().fold(
        (vec![], vec![], vec![]),
        |(mut defs, mut empty, mut merges), (name, ty, attr)| {
            if attr.nest {
                let ty_ident = get_ident_from_type(ty);
                let nested_builder = format_ident!("{}{}", ty_ident, SUFFIX);
                defs.push(quote! {
                #[serde(default)]
                pub #name: #nested_builder });
                empty.push(quote! {
                #name: #nested_builder::empty() });
                merges.push(quote! { #name: self.#name.merge(other.#name) });
            } else {
                defs.push(quote! { pub #name: Option<#ty> });
                empty.push(quote! { #name: None });
                merges.push(quote! { #name: self.#name.or(other.#name) });
            }
            (defs, empty, merges)
        },
    );

    quote! {
        #[derive(::serde::Deserialize, Debug, Clone)]
        pub struct #layer_name {
            #( #field_defs ),*
        }

        impl Default for #layer_name {
           fn default() -> Self {
            #name::default().into()
            }
        }

        impl ::confgr::core::Merge for #layer_name {
            fn merge(self, other: Self) -> Self {
                Self {
                    #( #merges ),*
                }
            }
        }

        impl ::confgr::core::Empty for #layer_name {
            fn empty() -> Self {
               Self {
                #( #empty_defs ),*
               }
            }
        }
    }
}
