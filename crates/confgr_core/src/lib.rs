use std::collections::HashMap;
use thiserror::Error;

/// Error type for configuration operations.
#[derive(Error, Debug)]
pub enum ConfgrError {
    #[error("Config File IO Error: {0}")]
    File(#[from] std::io::Error),
    #[error("Missing 'path' or 'path_env' attribute.")]
    NoFilePath,
    #[error("Config Error: {0}")]
    Config(#[from] config::ConfigError),
}

/// Merges configuration layers. Self takes precedence over other.
pub trait Merge {
    fn merge(self, other: Self) -> Self;
}

/// Creates an empty configuration layer, used to initialize all [`None`]'s, instead of [`Default`].
pub trait Empty {
    fn empty() -> Self;
}

/// Deserializes a configuration layer from environment variables.
pub trait FromEnv {
    fn from_env() -> Self;
    fn get_env_keys() -> HashMap<String, String>;
}

/// Deserializes a configuration layer from a file.
pub trait FromFile: Sized {
    fn from_file() -> Result<Self, ConfgrError>;
    fn check_file() -> Result<(), ConfgrError>;
    fn get_file_path() -> Option<String>;
}

/// Configuration trait that combines [`FromEnv`], [`FromFile`], and [`Merge`] traits. Implemented
/// by the derive macro on the original configuration struct.
pub trait Confgr
where
    Self: Sized,
{
    type Layer: Default + FromEnv + Merge + FromFile + From<Self> + Into<Self>;

    fn load_config() -> Self {
        let file_layer = Self::Layer::from_file().unwrap_or_else(|_| Self::Layer::default());
        let default_layer = Self::Layer::default();
        let env_layer = Self::Layer::from_env();

        env_layer.merge(file_layer.merge(default_layer)).into()
    }

    fn deserialize_from_file() -> Result<Self::Layer, ConfgrError> {
        Self::Layer::from_file()
    }

    fn check_file() -> Result<(), ConfgrError> {
        Self::Layer::check_file()
    }

    fn get_env_keys() -> HashMap<String, String> {
        Self::Layer::get_env_keys()
    }

    fn get_file_path() -> Option<String> {
        Self::Layer::get_file_path()
    }
}
