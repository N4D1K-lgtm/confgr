use quote::{quote, ToTokens};
use syn::{
    parse_macro_input, punctuated::Punctuated, Attribute, Data, DeriveInput, Error, Expr, ExprLit,
    Fields, Ident, Lit, Meta, Token, Type,
};

mod config;
mod convert;
mod env;
mod file;
mod merge;

const SUFFIX: &str = "ConfgrLayer";
const AUTOCONF_ATTRIBUTE: &str = "config";
const PATH_ATTRIBUTE: &str = "path";
const DEFAULT_PATH_ATTRIBUTE: &str = "default_path";
const ENV_PATH_ATTRIBUTE: &str = "env_path";
const KEY_ATTRIBUTE: &str = "key";
const PREFIX_ATTRIBUTE: &str = "prefix";
const SEPARATOR_ATTRIBUTE: &str = "separator";
const NEST_ATTRIBUTE: &str = "nest";
const SKIP_ATTRIBUTE: &str = "skip";
const NAME_ATTRIBUTE: &str = "name";

#[proc_macro_derive(Config, attributes(config))]
pub fn config_derive(input: proc_macro::TokenStream) -> proc_macro::TokenStream {
    let ast = parse_macro_input!(input as DeriveInput);
    match impl_config_derive(&ast) {
        Ok(expanded) => expanded.into(),
        Err(errors) => to_compile_errors(errors).into(),
    }
}

fn impl_config_derive(ast: &DeriveInput) -> Result<proc_macro2::TokenStream, Vec<syn::Error>> {
    let name = &ast.ident;
    let struct_attributes =
        parse_config_field_attributes(&ast.attrs).unwrap_or_else(|_| ConfigAttributes::default());

    let field_data = extract_fields(ast)?;

    let layer_impl = merge::generate_layer(name, &struct_attributes, &field_data);
    let config_impl = config::generate_config_impl(name);
    let from_impl = convert::generate_conversion_impl(name, &field_data);
    let env_impl = env::generate_from_env(name, &struct_attributes, &field_data);
    let file_impl = file::generate_from_file(name, &struct_attributes);

    let expanded = quote! {
        #layer_impl
        #from_impl
        #env_impl
        #file_impl
        #config_impl
    };

    Ok(expanded)
}

pub(crate) fn extract_fields(
    input: &DeriveInput,
) -> Result<Vec<(&Ident, &Type, ConfigAttributes)>, Vec<syn::Error>> {
    if let Data::Struct(data) = &input.data {
        if let Fields::Named(fields) = &data.fields {
            let mut field_data = Vec::new();
            let mut errors = Vec::new();

            for f in fields.named.iter() {
                match parse_config_field_attributes(&f.attrs) {
                    Ok(attributes) => {
                        field_data.push((f.ident.as_ref().unwrap(), &f.ty, attributes))
                    }
                    Err(mut errs) => errors.append(&mut errs),
                }
            }

            if errors.is_empty() {
                Ok(field_data)
            } else {
                Err(errors)
            }
        } else {
            Err(vec![syn::Error::new_spanned(
                &data.fields,
                "Config derive macro only supports structs with named fields.",
            )])
        }
    } else {
        Err(vec![syn::Error::new_spanned(
            input,
            "Config derive macro only supports structs.",
        )])
    }
}

pub(crate) fn parse_config_field_attributes(
    attrs: &[Attribute],
) -> Result<ConfigAttributes, Vec<syn::Error>> {
    let mut attributes = ConfigAttributes::new();
    let mut errors = Vec::new();

    for attr in attrs
        .iter()
        .filter(|a| a.path().is_ident(AUTOCONF_ATTRIBUTE))
    {
        let result = attr.parse_args_with(Punctuated::<Meta, Token![,]>::parse_terminated);
        match result {
            Ok(list) => {
                for meta in list {
                    match meta {
                        Meta::Path(path) if path.is_ident(SKIP_ATTRIBUTE) => attributes.skip = true,
                        Meta::Path(path) if path.is_ident(NEST_ATTRIBUTE) => attributes.nest = true,
                        Meta::NameValue(named_value)
                            if named_value.path.is_ident(PATH_ATTRIBUTE) =>
                        {
                            if let Expr::Lit(ExprLit {
                                lit: Lit::Str(path),
                                ..
                            }) = &named_value.value
                            {
                                attributes.path = Some(path.value());
                            } else {
                                errors.push(Error::new_spanned(
                                    named_value.into_token_stream(),
                                    "Expected a valid path for 'path'",
                                ));
                            }
                        }
                        Meta::NameValue(named_value)
                            if named_value.path.is_ident(PREFIX_ATTRIBUTE) =>
                        {
                            if let Expr::Lit(ExprLit {
                                lit: Lit::Str(prefix),
                                ..
                            }) = &named_value.value
                            {
                                attributes.prefix = Some(prefix.value());
                            } else {
                                errors.push(Error::new_spanned(
                                    named_value.into_token_stream(),
                                    "Expected a string for 'prefix'",
                                ));
                            }
                        }
                        Meta::NameValue(named_value)
                            if named_value.path.is_ident(KEY_ATTRIBUTE) =>
                        {
                            if let Expr::Lit(ExprLit {
                                lit: Lit::Str(key), ..
                            }) = &named_value.value
                            {
                                attributes.key = Some(key.value());
                            } else {
                                errors.push(Error::new_spanned(
                                    named_value.into_token_stream(),
                                    "Expected a string for 'key'",
                                ));
                            }
                        }
                        Meta::NameValue(named_value)
                            if named_value.path.is_ident(SEPARATOR_ATTRIBUTE) =>
                        {
                            if let Expr::Lit(ExprLit {
                                lit: Lit::Str(seperator),
                                ..
                            }) = &named_value.value
                            {
                                attributes.separator = Some(seperator.value());
                            } else {
                                errors.push(Error::new_spanned(
                                    named_value.into_token_stream(),
                                    "Expected a string for 'seperator'",
                                ));
                            }
                        }
                        Meta::NameValue(named_value)
                            if named_value.path.is_ident(ENV_PATH_ATTRIBUTE) =>
                        {
                            if let Expr::Lit(ExprLit {
                                lit: Lit::Str(path_env),
                                ..
                            }) = &named_value.value
                            {
                                attributes.path_env = Some(path_env.value());
                            } else {
                                errors.push(Error::new_spanned(
                                    named_value.into_token_stream(),
                                    "Expected a string for 'path_env'",
                                ));
                            }
                        }
                        Meta::NameValue(named_value)
                            if named_value.path.is_ident(DEFAULT_PATH_ATTRIBUTE) =>
                        {
                            if let Expr::Lit(ExprLit {
                                lit: Lit::Str(default_path),
                                ..
                            }) = &named_value.value
                            {
                                attributes.default_path = Some(default_path.value());
                            } else {
                                errors.push(Error::new_spanned(
                                    named_value.into_token_stream(),
                                    "Expected a string for 'default_path'",
                                ));
                            }
                        }
                        Meta::NameValue(named_value)
                            if named_value.path.is_ident(NAME_ATTRIBUTE) =>
                        {
                            if let Expr::Lit(ExprLit {
                                lit: Lit::Str(name),
                                ..
                            }) = &named_value.value
                            {
                                attributes.name = Some(name.value());
                            } else {
                                errors.push(Error::new_spanned(
                                    named_value.into_token_stream(),
                                    "Expected a string for 'name'",
                                ));
                            }
                        }
                        _ => {
                            errors.push(Error::new_spanned(
                                meta.into_token_stream(),
                                "Unsupported attribute in 'config'",
                            ));
                        }
                    }
                }
            }
            Err(e) => errors.push(e),
        }
    }

    if errors.is_empty() {
        Ok(attributes)
    } else {
        Err(errors)
    }
}

pub(crate) fn get_ident_from_type(ty: &Type) -> proc_macro2::Ident {
    if let Type::Path(type_path) = ty {
        type_path.path.segments.last().unwrap().ident.clone()
    } else {
        panic!("Type is not a Type::Path, which is required for nested builder patterns")
    }
}

#[derive(Debug, Default)]
pub(crate) struct ConfigAttributes {
    skip: bool,
    nest: bool,
    prefix: Option<String>,
    key: Option<String>,
    separator: Option<String>,
    path: Option<String>,
    path_env: Option<String>,
    default_path: Option<String>,
    name: Option<String>,
}

impl ConfigAttributes {
    fn new() -> Self {
        Self::default()
    }
}

fn to_compile_errors(errors: Vec<syn::Error>) -> proc_macro2::TokenStream {
    let compile_errors = errors.iter().map(syn::Error::to_compile_error);
    quote!(#(#compile_errors)*)
}
