use crate::{get_ident_from_type, ConfigAttributes, SUFFIX};
use proc_macro2::TokenStream;
use quote::{format_ident, quote};
use syn::{Ident, Type};

pub fn generate_layer(
    name: &Ident,
    attributes: &ConfigAttributes,
    field_data: &[(&Ident, &Type, ConfigAttributes)],
) -> TokenStream {
    let layer_name = format_ident!("{}{}", name, SUFFIX);

    let struct_rename = if let Some(attr_name) = &attributes.name {
        quote! { #[serde(rename = #attr_name)] }
    } else {
        quote! {}
    };

    let (field_defs, empty_defs, merges) = field_data.iter().fold(
        (vec![], vec![], vec![]),
        |(mut defs, mut empty, mut merges), (field_name, ty, attr)| {
            let field_rename = if let Some(rename) = &attr.name {
                quote! { #[serde(rename = #rename)] }
            } else {
                quote! {}
            };

            if attr.nest {
                let ty_ident = get_ident_from_type(ty);
                let nested_builder = format_ident!("{}{}", ty_ident, SUFFIX);
                defs.push(quote! {
                    #field_rename
                    #[serde(default)]
                    pub #field_name: #nested_builder
                });
                empty.push(quote! {
                    #field_name: #nested_builder::empty()
                });
                merges.push(quote! {
                    #field_name: self.#field_name.merge(other.#field_name)
                });
            } else {
                defs.push(quote! {
                    #field_rename
                    pub #field_name: Option<#ty>
                });
                empty.push(quote! {
                    #field_name: None
                });
                merges.push(quote! {
                    #field_name: self.#field_name.or(other.#field_name)
                });
            }
            (defs, empty, merges)
        },
    );

    quote! {
        #[derive(::serde::Deserialize, Debug, Clone)]
        #[doc(hidden)]
        #struct_rename
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

