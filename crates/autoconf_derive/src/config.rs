use crate::SUFFIX;
use proc_macro2::TokenStream;
use quote::{format_ident, quote};
use syn::Ident;

pub fn generate_config_impl(name: &Ident) -> TokenStream {
    let layer_name = format_ident!("{}{}", name, SUFFIX);

    quote! {
        impl ::autoconf_core::Config for #name {
            type Layer = #layer_name;

            fn config() -> Self {
                let empty = Self::Layer::empty();

                let file = Self::Layer::from_file().unwrap_or(Self::Layer::default());

                let merged_defaults = file.merge(Self::Layer::default());

                let env_config = Self::Layer::from_env();

                let final_config = env_config.merge(merged_defaults);

                final_config.into()
            }
        }
    }
}
