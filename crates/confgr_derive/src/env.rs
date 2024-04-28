use crate::{get_ident_from_type, ConfigAttributes, SUFFIX};
use proc_macro2::TokenStream;
use quote::{format_ident, quote};
use syn::{Ident, Type};

const DEFAULT_PREFIX: &str = "";
const DEFAULT_SEPARATOR: &str = "_";

pub fn generate_from_env(
    name: &Ident,
    struct_attributes: &ConfigAttributes,
    field_data: &[(&Ident, &Type, ConfigAttributes)],
) -> TokenStream {
    let layer_name = format_ident!("{}{}", name, SUFFIX);

    let env_items = field_data.iter().map(|(field_name, ty, attr)| {
        let mut env_var_name = field_name.to_string().to_uppercase();

        if let Some(ref key) = attr.key {
            env_var_name = key.clone().to_uppercase();
        } else if attr.prefix.is_some() || struct_attributes.prefix.is_some() {
            let separator = attr
                .separator
                .as_ref()
                .unwrap_or(
                    struct_attributes
                        .separator
                        .as_ref()
                        .unwrap_or(&String::from(DEFAULT_SEPARATOR)),
                )
                .clone();

            let prefix = attr
                .prefix
                .as_ref()
                .unwrap_or(
                    struct_attributes
                        .prefix
                        .as_ref()
                        .unwrap_or(&String::from(DEFAULT_PREFIX)),
                )
                .to_uppercase();

            env_var_name = format!(
                "{}{}{}",
                prefix,
                separator,
                field_name.to_string().to_uppercase()
            );
        }

        if attr.skip {
            quote! { #field_name: None }
        } else if attr.nest {
            let ty_ident = get_ident_from_type(ty);
            let nested_builder = format_ident!("{}{}", ty_ident, SUFFIX);
            quote! { #field_name: #nested_builder::from_env() }
        } else {
            quote! {
               #field_name: match ::std::env::var(#env_var_name) {
                    Ok(val) => {
                        val.parse::<#ty>().ok()
                    },
                    Err(_) => {
                        None
                    }
                }
            }
        }
    });

    quote! {
        impl ::confgr::core::FromEnv for #layer_name {
            fn from_env() -> Self {
                Self {
                    #( #env_items ),*
                }
            }
        }
    }
}
