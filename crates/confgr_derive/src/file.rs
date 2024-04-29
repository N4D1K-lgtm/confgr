use crate::{ConfigAttributes, SUFFIX};
use proc_macro2::TokenStream;
use quote::{format_ident, quote};
use syn::Ident;

pub(crate) fn generate_from_file(name: &Ident, attributes: &ConfigAttributes) -> TokenStream {
    let layer_name = format_ident!("{}{}", name, SUFFIX);

    let mut file_path = String::new();

    if let Some(path_env) = &attributes.path_env {
        if let Ok(path) = std::env::var(path_env) {
            file_path = path;
        }
    } else if let Some(path) = &attributes.path {
        file_path = path.to_string();
    }

    quote! {

        impl ::confgr::core::FromFile for #layer_name {
            fn from_file() -> Result<Self, ::confgr::core::ConfgrError> {
                Self::check_file()?;

                let config = ::confgr::config::Config::builder()
                    .add_source(::confgr::config::File::with_name(#file_path))
                    .build()?;

                config
                    .try_deserialize::<Self>().map_err(|e| ::confgr::core::ConfgrError::Config(e))
            }
            fn check_file() -> Result<(), ::confgr::core::ConfgrError> {
                use ::std::fs::File;
                use ::std::io::Read;

                let mut file = File::open(#file_path)
                    .map_err(|e| ::confgr::core::ConfgrError::File(e))?;

                let mut contents = String::new();
                file.read_to_string(&mut contents)
                    .map_err(|e| ::confgr::core::ConfgrError::File(e))?;

                Ok(())
            }
            fn get_file_path() -> String {
                #file_path.to_string()
            }
        }
    }
}
