use confgr::prelude::*;
use serde::Deserialize;
use smart_default::SmartDefault;
use std::collections::HashMap;

use std::env;
use std::fs::File;
use std::io::Write;

#[derive(Config, SmartDefault)]
#[config(path = "tests/complex_settings.toml", name = "config")]
struct Config {
    #[config(nest)]
    service: Service,
    #[config(skip)]
    secret_key: Option<String>,
    #[config(skip)]
    features: Vec<String>,
    #[config(skip)]
    parameters: HashMap<String, String>,
}

#[derive(Config, SmartDefault)]
#[config(name = "service")]
struct Service {
    #[default = "https://localhost:3000"]
    url: String,
    #[default = true]
    enabled: bool,
    #[config(skip)]
    ports: Vec<u32>,
    #[config(skip)]
    metadata: HashMap<String, String>,
}

#[derive(Config, Default)]
#[config(env_path = "OPTIONAL_ENV_CONFIG", name = "optional_env")]
struct OptionalEnvConfig {
    #[config(skip)]
    debug_mode: Option<bool>,
    #[config(nest)]
    db_settings: DbSettings,
}

#[derive(Config, SmartDefault)]
#[config(name = "db")]
struct DbSettings {
    host: String,
    port: u32,
    #[config(skip)]
    #[default(Some(Credentials::default()))]
    credentials: Option<Credentials>,
}

#[derive(Config, SmartDefault, Debug, Clone, Deserialize)]
#[config(name = "credentials")]
struct Credentials {
    #[default = "admin"]
    username: String,
    #[default = "password"]
    password: String,
}

fn setup_complex_config_files() {
    let complex_settings = r#"
        [service]
        url = "http://example.com/api"
        enabled = true
        ports = [8080, 8081]
        [service.metadata]
        environment = "production"
        version = "1.0.0"

        features = ["alpha", "beta"]
        [parameters]
        retries = "3"
        timeout = "100"
        "#;

    let mut settings_file = File::create("tests/complex_settings.toml").unwrap();
    writeln!(settings_file, "{}", complex_settings).unwrap();
}

fn cleanup_complex_config_files() {
    std::fs::remove_file("tests/complex_settings.toml").unwrap();
}

// #[test]
// fn test_complex_config_loads_correctly() {
//     setup_complex_config_files();
//
//     let config = Config::load_config();
//     assert_eq!(config.service.url, "http://example.com/api");
//     assert!(config.service.enabled);
//     assert_eq!(config.service.ports.len(), 2);
//     assert_eq!(config.features.len(), 2);
//     assert_eq!(config.parameters.get("retries").unwrap(), "3");
//
//     cleanup_complex_config_files();
// }
//
// #[test]
// fn test_optional_env_config_loads_with_defaults() {
//     env::set_var("OPTIONAL_ENV_CONFIG", "tests/complex_settings.toml");
//
//     let config = OptionalEnvConfig::load_config();
//     assert!(config.debug_mode.is_none()); // Assuming not set in config
//     assert_eq!(config.db_settings.host, "localhost"); // Assuming default values
//     assert!(config.db_settings.credentials.is_none()); // Assuming not provided
//
//     env::remove_var("OPTIONAL_ENV_CONFIG");
// }
