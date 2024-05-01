use crate::{ConfigAttributes, SUFFIX};
use proc_macro2::TokenStream;
use quote::{format_ident, quote};
use syn::Ident;

pub(crate) fn generate_from_file(name: &Ident, attributes: &ConfigAttributes) -> TokenStream {
    let layer_name = format_ident!("{}{}", name, SUFFIX);

    if attributes.path.is_some() && attributes.default_path.is_some() {
        panic!("'path' and 'default_path' attributes cannot be used alongside eachother");
    }

    let get_file_path_def = if let Some(path_env) = &attributes.path_env {
        match (&attributes.path, &attributes.default_path) {
            (Some(path), None) => {
                quote! {
                    fn get_file_path() -> Option<String> {
                        match std::env::var(#path_env) {
                            Ok(env_val) => if std::path::Path::new(&env_val).exists() { Some(env_val) }
                                           else if std::path::Path::new(#path).exists() { Some(#path.to_string()) }
                                           else { panic!("'path_env' and 'path' attributes resolve to non-existent or invalid files.") },
                            Err(_) => if std::path::Path::new(#path).exists() { Some(#path.to_string()) }
                                      else { panic!("'path_env' variable is not set and the provided 'path' attribute is invalid or references a non-existent file.") }
                        }
                    }
                }
            }
            (None, Some(default_path)) => {
                quote! {
                    fn get_file_path() -> Option<String> {
                        match std::env::var(#path_env) {
                            Ok(env_val) => if std::path::Path::new(&env_val).exists() { Some(env_val) }
                                           else if std::path::Path::new(#default_path).exists() { Some(#default_path.to_string()) }
                                           else { None },
                            Err(_) => if std::path::Path::new(#default_path).exists() { Some(#default_path.to_string()) }
                                      else { None }
                        }
                    }
                }
            }
            _ => {
                quote! {
                    fn get_file_path() -> Option<String> {
                        std::env::var(#path_env)
                            .map(|env_val| {
                                if std::path::Path::new(&env_val).exists() {
                                    Some(env_val)
                                } else {
                                    None
                                }
                        })
                        .ok()
                        .flatten()
                    }
                }
            }
        }
    } else if let Some(path) = &attributes.path {
        quote! {
            fn get_file_path() -> Option<String> {
                if std::path::Path::new(#path).exists() {
                    Some(#path.to_string())
                } else {
                    panic!("The provided 'path' attribute value '{}' is invalid or references a non-existent file.", #path);
                }
            }
        }
    } else if let Some(default_path) = &attributes.default_path {
        quote! {
            fn get_file_path() -> Option<String> {
                if std::path::Path::new(#default_path).exists() {
                    Some(#default_path.to_string())
                } else {
                    None
                }
            }
        }
    } else {
        quote! {
            fn get_file_path() -> Option<String> { None }
        }
    };

    quote! {
        impl ::confgr::core::FromFile for #layer_name {
            #get_file_path_def

            fn from_file() -> Result<Self, ::confgr::core::ConfgrError> {
                let file_path = Self::get_file_path().ok_or(::confgr::core::ConfgrError::NoFilePath)?;

                Self::check_file()?;

                let config = ::confgr::config::Config::builder()
                    .add_source(::confgr::config::File::with_name(&file_path))
                    .build()?;

                config.try_deserialize::<Self>().map_err(|e| ::confgr::core::ConfgrError::Config(e))
            }

            fn check_file() -> Result<(), ::confgr::core::ConfgrError> {
                use ::std::fs::File;
                use ::std::io::Read;

                let file_path = Self::get_file_path().ok_or(::confgr::core::ConfgrError::NoFilePath)?;

                let mut file = File::open(&file_path)
                    .map_err(|e| ::confgr::core::ConfgrError::File(e))?;

                let mut contents = String::new();
                file.read_to_string(&mut contents)
                    .map_err(|e| ::confgr::core::ConfgrError::File(e))?;


                Ok(())
            }
        }
    }
}
