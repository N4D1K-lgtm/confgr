use confgr::prelude::*;

use std::fs::File;
use std::io::Write;

#[derive(Config, Default, Debug)]
#[config(path = "tests/simple_settings.toml")]
struct AppConfig {
    url: String,
    port: u32,
    enabled: bool,
    #[config(nest, name = "db")]
    database: DatabaseConfig,
}

#[derive(Config, Default, Debug)]
struct DatabaseConfig {
    #[config(skip)]
    host: String,
    #[config(skip)]
    username: String,
    #[config(skip, name = "secret")]
    password: String,
}

fn setup_simple_config_file() {
    let settings = r#"
        url = "https://example.com"
        port = 8080
        enabled = true

        [db]
        host = "localhost"
        username = "admin"
        secret = "securepass"
        "#;

    let mut settings_file = File::create("tests/simple_settings.toml").unwrap();
    writeln!(settings_file, "{}", settings).unwrap();
}

fn cleanup_simple_config_files() {
    std::fs::remove_file("tests/simple_settings.toml").unwrap();
}

#[test]
fn test_simple_config() {
    setup_simple_config_file();

    let config = AppConfig::load_config();
    println!("{:?}", config);

    assert_eq!(config.url, "https://example.com");
    assert_eq!(config.port, 8080);
    assert!(config.enabled);

    assert_eq!(config.database.password, "securepass");

    cleanup_simple_config_files();
}
