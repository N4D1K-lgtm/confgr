//! # Overview
//!
//! The [`confgr`](self) crate simplifies application configuration by automatically loading
//! settings from environment variables, then from a configuration file, and then default values.
//!
//! As such, your configuration may never fail, without sacrificing flexibility.
//!
//! Several useful helper attributes are provided to customize the behavior of the proc-macro.
//!
//! | Attribute   | Functionality                                                                                                                              |
//! |-------------|--------------------------------------------------------------------------------------------------------------------------------------------|
//! | `prefix`    | Sets a prefix for environment variables. Can be applied at the struct or field level.                                                      |
//! | `path`      | Specifies the path to the configuration file. The file extension may be omitted.                                                           |
//! | `path_env`  | Resolves an environment variable at runtime to determine the configuration file path.                                                      |
//! | `key`       | Overrides the default environment variable name. This ignores the prefix and uses the provided key directly.                               |
//! | `nest`      | Required for non-standard types which must also derive [`Config`](self::derive::Config), useful for nesting configuration structs.         |
//! | `skip`      | Skips loading the attribute from an environment variable. Useful for types that don't implement [`FromStr`](std::str::FromStr) but are present in the configuration file. |
//! | `separator` | Specifies a character to separate the prefix and the field name. The default separator is "_".                                             |
//!
//! ## Key Features
//!
//! - **Simplicity**: Minimal boilerplate. Define your configuration struct, add useful annotations, and you're good to go.
//! - **Flexibility**: Supports a variety of configuration file formats including `toml`, `json`, `yaml`, `ini`, `ron`, and `json5`.
//! - **Integration**: Synergy with other Rust macros and libraries, such as [`smart_default`](https://docs.rs/smart-default/latest/smart_default/).
//!
//! ## Usage Example
//!
//! Below is a simple example demonstrating how to use [`confgr`](self) to load application settings:
//!
//! ```rust
//! use confgr::prelude::*;
//!
//! # use std::fs::File;
//! # use std::io::Write;
//! # let mut file = std::fs::File::create("docs.toml").unwrap();
//! # writeln!(file, "address = '127.0.0.1'");
//! #[derive(Config)]
//! #[config(path = "docs.toml", prefix = "APP")]
//! pub struct AppConfig {
//!     port: u32,
//!     address: String,
//!     #[config(key = "DEBUG_MODE")]
//!     debug: bool,
//! }
//!
//! // A Default implementation is required.
//! impl Default for AppConfig {
//!     fn default() -> Self {
//!         Self {
//!             port: 3000,
//!             address: "127.0.0.1".to_string(),
//!             debug: false
//!         }
//!     }
//! }
//!
//! std::env::set_var("APP_PORT", "4000");
//! std::env::set_var("DEBUG_MODE", "true");
//!
//! let settings = AppConfig::load_config();
//! # std::fs::remove_file("docs.toml").unwrap();
//!
//! assert_eq!(settings.port, 4000);
//! assert_eq!(settings.address, "127.0.0.1");
//! assert!(settings.debug)
//! ```
//!
//! ## Important Information
//!
//! - Nested structs do not load separate files based on their own `path` attributes. If
//! you would like multiple files to be loaded, you must use multiple structs with multiple
//! [`load_config()`](core::Confgr::load_config()) calls.
//! - Types that do not implement [`FromStr`](std::str::FromStr) must use `config(skip)` or `config(nest)`.
//! - The `separator` character is only inserted between the prefix and the field name.
//! - The `prefix` is applied per field or for the entire struct, and it's ignored if `config(key)` is used.
//! - All structs must implement [`Default`].
//!
//! ## Debugging
//!
//! When encountering issues using the macro, the following methods may be of use.
//!
//! ### Verifying Environment Variables
//!
//! The [`get_env_keys()`](core::Confgr::get_env_keys) method can be used to retrieve the
//! resolved environment variable keys based on the struct's configuration.
//!
//! ```rust
//! use std::collections::HashMap;
//! use confgr::prelude::*;
//!
//! #[derive(Config, Default)]
//! #[config(prefix = "APP")]
//! pub struct AppConfig {
//!     port: u32,
//!     #[config(separator = "__")]
//!     address: String,
//!     #[config(key = "DEBUG_MODE")]
//!     debug: bool,
//! }
//!
//! let keys: HashMap<String, String> = AppConfig::get_env_keys();
//!
//! assert_eq!(keys["port"], "APP_PORT");
//! assert_eq!(keys["address"], "APP__ADDRESS");
//! assert_eq!(keys["debug"], "DEBUG_MODE");
//! ```
//!
//! ### Verifying Configuration File Path
//!
//! You can use [`check_file()`](core::Confgr::check_file) to ensure that the configuration file
//! is accessible the path specified or resolved in the `path`, or `path_env` attribute.
//!
//! ```rust
//! use confgr::prelude::*;
//!
//! # use std::fs::File;
//! # use std::io::Write;
//! # let mut file = std::fs::File::create("docs.toml").unwrap();
//! # writeln!(file, "port = 3000\ndebug = false");
//! #[derive(Config, Default)]
//! #[config(path = "docs.toml", path_env = "APP_CONFIG_FILE")]
//! pub struct AppConfig {
//!     port: u32,
//!     debug: bool,
//! }
//!
//! std::env::set_var("APP_CONFIG_FILE", "config.toml");
//! AppConfig::check_file().expect("Failed to open configuration file.");
//!
//! std::env::remove_var("APP_CONFIG_FILE");
//! AppConfig::check_file().expect("Failed to open configuration file.");
//! # std::fs::remove_file("docs.toml").unwrap();
//! ```
//!
//! ### Test Deserialization
//!
//! The [`deserialize_from_file()`](core::Confgr::deserialize_from_file()) method can be used to manually test the config deserialization step. This
//! will give you the parsed configuration struct before default values are applied.
//!
//! ```rust
//! use confgr::prelude::*;
//!
//! # use std::fs::File;
//! # use std::io::Write;
//! # let mut file = std::fs::File::create("docs.toml").unwrap();
//! # writeln!(file, "port = 3000\ndebug = false");
//! #[derive(Config, Default)]
//! #[config(path = "docs.toml")]
//! pub struct AppConfig {
//!     port: u32,
//!     debug: bool,
//! }
//!
//! let config = AppConfig::deserialize_from_file().expect("Failed to deserialize configuration.");
//! println!("Deserialized configuration: {:?}", config);
//!
//! # std::fs::remove_file("docs.toml").unwrap();
//! ```
pub mod core {
    pub use confgr_core::*;
}

pub mod derive {
    pub use confgr_derive::*;
}

pub mod config {
    pub use config::{Config, ConfigError, File};
}

pub mod prelude {
    pub use crate::core::{Confgr, Empty, FromEnv, FromFile, Merge};
    pub use crate::derive::Config;
}
