use crate::{ConfigAttributes, SUFFIX};
use proc_macro2::TokenStream;
use quote::{format_ident, quote};
use syn::Ident;

pub fn generate_config_impl(name: &Ident, attributes: &ConfigAttributes) -> TokenStream {
    let layer_name = format_ident!("{}{}", name, SUFFIX);

    let file_loading_code = if let Some(_path) = &attributes.path {
        quote! {
            let file = Self::Layer::from_file().unwrap_or(Self::Layer::default());
        }
    } else {
        quote! {
            let file = Self::Layer::default();
        }
    };

    // Generate the final implementation
    quote! {
        impl ::confgr::core::Load for #name {
            type Layer = #layer_name;

            fn load_config() -> Self {
                let empty = Self::Layer::empty();

                #file_loading_code

                let merged_defaults = file.merge(Self::Layer::default());

                let env_config = Self::Layer::from_env();

                let final_config = env_config.merge(merged_defaults);

                final_config.into()
            }
        }
    }
}
