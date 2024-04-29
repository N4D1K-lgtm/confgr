use crate::SUFFIX;
use proc_macro2::TokenStream;
use quote::{format_ident, quote};
use syn::Ident;

pub fn generate_config_impl(name: &Ident) -> TokenStream {
    let layer_name = format_ident!("{}{}", name, SUFFIX);

    quote! {
        impl ::confgr::core::Confgr for #name {
            type Layer = #layer_name;
        }
    }
}
