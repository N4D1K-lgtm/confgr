use std::collections::HashMap;
use thiserror::Error;

/// Shared error type for configuration-related errors.
#[derive(Error, Debug)]
pub enum ConfgrError {
    #[error("Config File IO Error: {0}")]
    File(#[from] std::io::Error),
    #[error("Configured filepath does not exist.")]
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

/// Provides a unified approach to load configurations from environment variables,
/// files, and default settings. This trait is typically derived using a macro to automate
/// implementations based on struct field names and annotations.
pub trait Confgr
where
    Self: Sized,
{
    type Layer: Default + FromEnv + Merge + FromFile + From<Self> + Into<Self>;

    /// Loads and merges configurations from files, environment variables, and default values.
    /// Order of precedence: Environment variables, file configurations, default values.
    ///
    /// # Examples
    ///
    /// ```rust no_run
    /// let config = AppConfig::load_config();
    /// assert_eq!(config.port, 8080);
    /// ```
    fn load_config() -> Self {
        let file_layer = match Self::deserialize_from_file() {
            Ok(file_layer) => file_layer,
            Err(_e) => Self::Layer::default(),
        };

        let default_layer = Self::Layer::default();
        let env_layer = Self::Layer::from_env();

        env_layer.merge(file_layer.merge(default_layer)).into()
    }

    /// Attempts to deserialize configuration from a file.
    /// This method is a part of the file loading phase of the configuration process.
    ///
    /// # Errors
    ///
    /// Returns [`ConfgrError`] if the file cannot be read.
    ///
    /// # Examples
    ///
    /// ``` rust no_run
    /// let file_layer = AppConfig::deserialize_from_file();
    /// match file_layer {
    ///     Ok(layer) => println!("Configuration loaded from file."),
    ///     Err(e) => eprintln!("Failed to load configuration: {}", e),
    /// }
    /// ```
    fn deserialize_from_file() -> Result<Self::Layer, ConfgrError> {
        Self::Layer::from_file()
    }

    /// Checks the accessibility of the specified configuration file.
    ///
    /// # Returns
    ///
    /// [`Ok`] if the file is accessible, otherwise an [`Err`]\([`ConfgrError`]) if the file cannot be found or opened.
    ///
    /// # Examples
    ///
    /// ```
    /// if AppConfig::check_file().is_ok() {
    ///     println!("Configuration file is accessible.");
    /// } else {
    ///     println!("Cannot access configuration file.");
    /// }
    /// ```
    fn check_file() -> Result<(), ConfgrError> {
        Self::Layer::check_file()
    }

    /// Retrieves the map of environment variable keys associated with the configuration properties.
    ///
    /// # Returns
    ///
    /// A [`HashMap`] where the keys are property names and the values are the corresponding environment variable names.
    ///
    /// # Examples
    ///
    /// ```
    /// let env_keys = AppConfig::get_env_keys();
    /// assert_eq!(env_keys["port"], "APP_PORT");
    /// ```
    fn get_env_keys() -> HashMap<String, String> {
        Self::Layer::get_env_keys()
    }

    /// Gets the file path used for loading the configuration, if specified.
    ///
    /// # Returns
    ///
    /// An [`Option<String>`] which is `Some(path)` if a path is set, otherwise `None`.
    ///
    /// # Examples
    ///
    /// ```
    /// if let Some(path) = AppConfig::get_file_path() {
    ///     println!("Configuration file used: {}", path);
    /// } else {
    ///     println!("No specific configuration file used.");
    /// }
    /// ```
    fn get_file_path() -> Option<String> {
        Self::Layer::get_file_path()
    }
}
