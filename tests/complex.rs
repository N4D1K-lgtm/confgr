use confgr::prelude::*;
use serde::Deserialize;
use smart_default::SmartDefault;
use std::collections::HashMap;

use std::env;
use std::fs::File;
use std::io::Write;

#[derive(Config, SmartDefault)]
#[config(path = "tests/complex_settings.toml")]
struct AppConfig {
    #[config(nest)]
    service: ServiceConfig,
    #[config(skip, name = "secret")]
    secret_key: Option<String>,
    #[config(skip)]
    features: Vec<String>,
    #[config(skip)]
    parameters: HashMap<String, String>,
}

#[derive(Config, SmartDefault)]
struct ServiceConfig {
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
#[config(env_path = "OPTIONAL_ENV_CONFIG")]
struct OptionalEnvConfig {
    #[config(skip)]
    debug_mode: Option<bool>,
    #[config(nest, name = "db")]
    db_settings: DatabaseConfig,
}

#[derive(Config, SmartDefault)]
struct DatabaseConfig {
    #[default = "localhost"]
    host: String,
    port: u32,
    #[config(skip)]
    #[default(Some(Credentials::default()))]
    credentials: Option<Credentials>,
}

#[derive(SmartDefault, Debug, Clone, Deserialize)]
struct Credentials {
    #[default = "admin"]
    username: String,
    #[default = "password"]
    #[allow(dead_code)]
    password: String,
}

fn setup_complex_config_files() {
    let complex_settings = r#"
        secret = "sup3rs3cr3t"

        [service]
        url = "http://example.com/api"
        enabled = true
        ports = [8080, 8081]

        [service.metadata]
        environment = "production"
        version = "1.0.0"


        [parameters]
        retries = "3"
        timeout = "100"
        "#;

    let mut settings_file =
        File::create("tests/complex_settings.toml").expect("Failed to create file");
    writeln!(settings_file, "{}", complex_settings).expect("Failed to write to file");
}

fn cleanup_complex_config_files() {
    std::fs::remove_file("tests/complex_settings.toml").expect("Failed to cleanup config file");
}

#[test]
fn test_complex_config_loads_correctly() {
    setup_complex_config_files();

    let config = AppConfig::load_config();

    assert_eq!(config.service.url, "http://example.com/api");
    assert!(config.service.enabled);
    assert_eq!(config.service.ports.len(), 2);
    assert_eq!(config.secret_key, Some("sup3rs3cr3t".to_string()));
    assert_eq!(config.features.len(), 0);
    assert_eq!(config.parameters.get("retries").unwrap(), "3");

    cleanup_complex_config_files();
}

#[test]
fn test_optional_env_config_loads_with_defaults() {
    env::set_var("OPTIONAL_ENV_CONFIG", "tests/complex_settings.toml");

    let config = OptionalEnvConfig::load_config();

    assert!(config.debug_mode.is_none());
    assert_eq!(config.db_settings.host, "localhost");
    assert_eq!(
        config
            .db_settings
            .credentials
            .expect("Credentials not found")
            .username,
        "admin"
    );

    env::remove_var("OPTIONAL_ENV_CONFIG");
}
