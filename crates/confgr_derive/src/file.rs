use crate::{ConfigAttributes, SUFFIX};
use proc_macro2::TokenStream;
use quote::{format_ident, quote};
use syn::Ident;

pub(crate) fn generate_from_file(name: &Ident, attributes: &ConfigAttributes) -> TokenStream {
    let layer_name = format_ident!("{}{}", name, SUFFIX);
    let file_path = attributes.path.clone();

    quote! {

        impl ::confgr::core::FromFile for #layer_name {
            fn from_file() -> Result<Self, String> {
                let config = ::confgr::config::Config::builder()
                    .add_source(::confgr::config::File::with_name(#file_path))
                    .build()
                    .map_err(|e| format!("Error building config: {}", e))?;

                config
                    .try_deserialize::<Self>()
                    .map_err(|e| format!("Error deserializing config: {}", e))
            }
        }
    }
}
