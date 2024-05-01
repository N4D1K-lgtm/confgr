<h1 align="center">Confgr</h1>
<div align="center">
 <strong>
  A simple rust application configuration derive macro.
 </strong>
</div>

<br />

<div align="center">
  <!-- Github Actions -->
  <a href="https://github.com/N4D1K-lgtm/confgr/actions/workflows/rust.yml?query=branch%3Amaster">
    <img src="https://img.shields.io/github/actions/workflow/status/N4D1K-lgtm/confgr/rust.yml?branch=master&style=flat-square" alt="actions status" /></a>
  <!-- Version -->
  <a href="https://crates.io/crates/confgr">
    <img src="https://img.shields.io/crates/v/confgr.svg?style=flat-square"
    alt="Crates.io version" /></a>
  <!-- Docs -->
  <a href="https://docs.rs/confgr">
  <img src="https://img.shields.io/badge/docs-latest-blue.svg?style=flat-square" alt="docs.rs docs" /></a>
  <!-- Downloads -->
  <a href="https://crates.io/crates/confgr">
    <img src="https://img.shields.io/crates/d/confgr.svg?style=flat-square" alt="Download" />
  </a>
</div>

<div align="center">
  <h4>
    <a href="#install">
      Overview
    </a>
    <span> | </span>
    <a href="#usage">
      Usage
    </a>
    <span> | </span>
    <a href="#considerations">
      Considerations
    </a>
  </h4>
</div>

<br />

<div align="center">
  <small>Built with ❤️ by Kidan Nelson</small>
</div>

# Overview

The [`Config`](https://docs.rs/confgr/latest/confgr/prelude/derive.Config.html) derive macro simplifies application configuration by automatically loading
settings from various sources in the following order:

1.  **Environment Variables**.
2.  **Configuration File** (e.g., `toml`, `json`, `yaml`, `ini`, `ron`, `json5`).
3.  **Default Values**.

## Key Features

- **Simplicity**: Minimal boilerplate. Define your configuration struct, customize the macro, and you're good to go.
- **Flexibility**: Supports a variety of configuration file formats including `toml`, `json`, `yaml`, `ini`, `ron`, and `json5`.
- **Integration**: Synergy with other crates, such as [`smart_default`](https://docs.rs/smart_default/latest/smart_default/).

There are also several useful helper attributes for customizing the behavior of the derive macro.

| Attribute      | Functionality                                                                                                                                                                                                   |
| -------------- | --------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------- |
| `prefix`       | Sets a prefix for environment variables. Can be applied at the struct or field level.                                                                                                                           |
| `path`         | Specifies the static path to a configuration file. The file extension may (though probably shouldn't) be omitted.                                                                                               |
| `env_path`     | Resolves an environment variable at runtime to determine the configuration file path.                                                                                                                           |
| `default_path` | Specifies a fallback path used if the path determined by `env_path` does not exist.                                                                                                                             |
| `key`          | Overrides the default environment variable name. This ignores the prefix and uses the provided key directly.                                                                                                    |
| `name`         | forwards to `#[serde(rename = "_")]` to rename fields during serialization/deserialization. It does not affect environment variable names.                                                                      |
| `nest`         | Required for non-standard types which must also derive [`Config`](https://docs.rs/confgr/latest/confgr/prelude/derive.Config.html), used for nesting configuration structs.                                     |
| `skip`         | Skips loading the attribute from an environment variable. Necessary for types that don't implement [`FromStr`](https://doc.rust-lang.org/std/str/trait.FromStr.html) but are present in the configuration file. |
| `separator`    | Specifies a character to separate the prefix and the field name. The default separator is "\_".                                                                                                                 |

## Path Attribute Behavior

- **`env_path`**: Resolves the provided environment variable into configuration filepath. This
  takes precedence over `path` and `default_path`, but will not panic if the file or environment
  does not exist.
- **`path`**: Directly sets the path to the configuration file. When set, `default_path` may not be used. Panics if the file does not exist.
- **`default_path`**: Identical to `path`, but does not panic if the file does not exist.

## Usage

[`serde`](https://docs.rs/serde) is a required dependency.

```toml
 [dependencies]
 confgr = "0.2.0"
 serde = { version = "1.0", features = ["derive"] }
```

Then define your configuration like so:

```rust
use confgr::prelude::*;

#[derive(Config)]
#[config(path = "docs.toml", prefix = "APP")]
pub struct AppConfig {
  port: u32,
  address: String,
  #[config(key = "DEBUG_MODE")]
  debug: bool,
}

// Default implementation is required.
impl Default for AppConfig {
  fn default() -> Self {
    Self {
      port: 3000,
      address: "127.0.0.1".to_string(),
      debug: false
    }
  }
}

std::env::set_var("APP_PORT", "4000");
std::env::set_var("DEBUG_MODE", "true");

let settings = AppConfig::load_config();

assert_eq!(settings.port, 4000);
assert_eq!(settings.address, "127.0.0.1");
assert!(settings.debug)
```

> Check out the [examples](/examples) directory for more.

## Warnings/Pitfalls

- Nested structs do not load separate files based on their own `path` attributes. If
  you would like multiple files to be loaded, you must use multiple structs with multiple
  [`load_config()`](<core::Confgr::load_config()>) calls. This may change in a future version.
- Types that do not implement [`FromStr`](std::str::FromStr) must use `#[config(skip)]` or `#[config(nest)]`.
- The `separator` character is only inserted between the prefix and the field name, not in any
  part of the parsed field name.
- The `prefix` is applied per field or for the entire struct, but is ignored if `#[config(key = "_")]` is used.
- All configuration structs must implement [`Default`](https://doc.rust-lang.org/std/default/trait.Default.html).
- Types used in configuration structs must implement [`Deserialize`](https://docs.rs/serde/latest/serde/trait.Deserialize.html),
  [`Clone`](https://doc.rust-lang.org/std/clone/trait.Clone.html),
  [`Debug`](https://doc.rust-lang.org/std/fmt/trait.Debug.html) and
  [`Default`](https://doc.rust-lang.org/std/default/trait.Default.html).
- [`Option`](https://doc.rust-lang.org/std/option/enum.Option.html) is not currently compatible with `#[config(nest)]`
  on types that implement [`Confgr`](https://docs.rs/confgr/latest/confgr/core/trait.Confgr.html).

## Debugging

When encountering issues using the macro, the following methods may be of use.

### Verifying Environment Variables

The [`get_env_keys()`](core::Confgr::get_env_keys) method can be used to retrieve the
resolved environment variable keys based on the struct's configuration.

```rust
use std::collections::HashMap;
use confgr::prelude::*;

#[derive(Config, Default)]
#[config(prefix = "APP")]
pub struct AppConfig {
    port: u32,
    #[config(separator = "__")]
    address: String,
    #[config(key = "DEBUG_MODE")]
    debug: bool,
}

let keys: HashMap<String, String> = AppConfig::get_env_keys();

assert_eq!(keys["port"], "APP_PORT");
assert_eq!(keys["address"], "APP__ADDRESS");
assert_eq!(keys["debug"], "DEBUG_MODE");
```

### Verifying Configuration File Path

You can use [`check_file()`](core::Confgr::check_file) to ensure that the configuration file
is accessible the path specified or resolved in the `path`, or `env_path` or `default_path` attributes.

```rust
use confgr::prelude::*;

#[derive(Config, Default)]
#[config(path = "docs.toml", env_path = "APP_CONFIG_FILE")]
pub struct AppConfig {
    port: u32,
    debug: bool,
}

std::env::set_var("APP_CONFIG_FILE", "env_config.toml");
AppConfig::check_file().expect("Failed to open configuration file.");

std::env::remove_var("APP_CONFIG_FILE");
AppConfig::check_file().expect("Failed to open configuration file.");

```

### Test Deserialization

The [`deserialize_from_file()`](<core::Confgr::deserialize_from_file()>) method can be used to manually test the config deserialization step. This
will give you the parsed configuration struct before default values are applied.

```rust
use confgr::prelude::\*;

#[derive(Config, Default)] #[config(path = "docs.toml")]
pub struct AppConfig {
port: u32,
debug: bool,
}

let config = AppConfig::deserialize_from_file().expect("Failed to deserialize configuration.");
println!("Deserialized configuration: {:?}", config);
```

## Considerations

- **Version Instability**: As of now, this crate is in an unstable development phase and I reserve the right to make breaking changes in future versions
  without obligation to maintain backwards compatibility.
- **Production Use Caution**: This is my first published Rust crate, while it is fully functional and useful for me, it's advisable not to rely heavily on this library in critical production environments without thorough testing, especially where guarantees of stability and performance are required.
- **Contribution**: Contributions are welcome! Whether it's feature requests, bug reports, or pull requests, I'd love some constructive feedback!

> I highly recommend checking out the [`config`](https://docs.rs/config/latest/config/) crate as it is a feature complete non-proc-macro alternative. This crate actually relies on `config` for file parsing.
