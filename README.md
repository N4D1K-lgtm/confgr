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

## Overview

[`confgr`](https://docs.rs/confgr/latest/confgr) is a crate that enables easily managing rust application configuration by automatically deriving functionality to load settings from environment variables, configuration files, and default values. This is done by procedurally parsing struct fields to build environment variable keys as well as deserialization using [`serde`](https://docs.rs/serde/latest/serde/) from a provided config file path. Functionality is customizable through several macro attribute helpers.

The order of priority is Environment Variable -> Config File -> Default Value. If a `config(path = "filepath")` attribute is not present, a config file will not be loaded, and `config(skip)` may be used to skip the environment variable step.

| Attribute      | Functionality                                                                                                                              |
|----------------|--------------------------------------------------------------------------------------------------------------------------------------------|
| prefix         | Sets the prefix for environment variables, can be set at the struct or field level.                                                        |
| path           | Specifies the path to the configuration file, the extension may be omitted.                                                                |
| key            | Overrides the default key name for an attribute, ignores the prefix and field name.                                                        |
| nest           | Necessary for non standard types, these must also derive `Config`                                                                          |
| skip           | Skips loading the attribute from an environment variable.                                                                                  |
| separator      | Sets the separator character that is placed between the prefix and the field name, can be set at the struct or field level, default is "_" |


## Key Features

- **Simplicity**: Minimal boilerplate, as simple as annotating your struct and a struct with named fields and a single method.
- **Flexibility**: Supports loading configuration data from environment variables, a single `toml`, `json`, `yaml`, `xml`, `ini`, `ron` or `json5` configuration file with default trait implementations as a fall-back.
- **Integration**: Integrates conveniently with other macros such as [`smart_default`](https://docs.rs/smart-default/latest/smart_default/derive.SmartDefault.html).
## Usage

The simplest way to use `confgr` is as follows: 
```rust
use confgr::prelude::*;

#[derive(Config)]
#[config(path = "config.toml")]
pub struct AppConfig {
  port: u32,
  address: String,
}

// Default implementations are required.
impl Default for AppConfig {
  fn default() -> Self {
    Self {
      port: 3000,
      address: "127.0.0.1".to_string(),
    }
  }
}
```

Then you can load your settings like so:

```rust
fn main() {
  std::env::set_var("PORT", "4000");

  // AppConfig {
  //  port: 4000,
  //  address: "127.0.0.1"
  // }
  let settings = AppConfig::load_config();
}
```

This is intended to easily be used inside of something like [`std::sync::OnceLock`](https://doc.rust-lang.org/nightly/std/sync/struct.OnceLock.html)

## Considerations
- **Version Flexibility**: This is an initial release (v0.1.0), and as such, it is not fully optimized. The implementation involves some cloning for simplicity, which may impact performance in large-scale applications.
- **Production Use Caution**: This is my first published Rust crate, while it is fully functional and useful for me, it's advisable not to rely heavily on this library in critical production environments without thorough testing, especially where guarantees of stability and performance are required.
- **Contribution**: Contributions are welcome! Whether it's feature requests, bug reports, or pull requests, i'd love some constructive feedback!

> I highly recommend checking out the [`config`](https://docs.rs/config/latest/config/) crate as it is a feature complete non-proc-macro alternative. This crate actually relies on `config` for file parsing.
