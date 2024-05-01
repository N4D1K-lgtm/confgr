use confgr::prelude::*;

use std::fs;
use std::io::Write;

mod common;

#[derive(Config, Debug)]
#[config(prefix = "PRIORITY")]
pub struct TestPriority {
    pub name: String,
    pub id: i32,
}

impl Default for TestPriority {
    fn default() -> Self {
        Self {
            name: "DefaultName".to_string(),
            id: 1,
        }
    }
}

#[derive(Config, Debug)]
#[config(path = "tests/common/priority.toml", prefix = "PRIORITY")]
pub struct TestFilePriority {
    pub name: String,
    #[config(skip)]
    pub skipped: bool,
    pub timeout: u64,
}

impl Default for TestFilePriority {
    fn default() -> Self {
        Self {
            name: "DefaultName".to_string(),
            skipped: false,
            timeout: 100,
        }
    }
}

fn setup_env_vars() {
    std::env::set_var("PRIORITY_NAME", "EnvName");
    std::env::set_var("PRIORITY_ID", "20");
    std::env::set_var("PRIORITY_TIMEOUT", "500");
    std::env::set_var("PRIORITY_SKIPPED", "false");
}

fn cleanup_env_vars() {
    std::env::remove_var("PRIORITY_NAME");
    std::env::remove_var("PRIORITY_ID");
    std::env::remove_var("PRIORITY_TIMEOUT");
    std::env::remove_var("PRIORITY_SKIPPED");
}

fn create_config_file() {
    let data = r#"
            name = "TomlName"
            timeout = 400
            skipped = true
        "#;
    let mut file = fs::File::create("tests/common/priority.toml").unwrap();
    writeln!(file, "{}", data).unwrap();
}

fn cleanup_config_file() {
    let _ = fs::remove_file("tests/common/priority.toml");
}

#[test]
fn env_overrides_config_and_default() {
    setup_env_vars();
    create_config_file();

    let config = TestFilePriority::load_config();

    assert_eq!(config.name, "EnvName");
    assert_eq!(config.timeout, 500);

    cleanup_env_vars();
    cleanup_config_file();
}

#[test]
fn file_overrides_default() {
    cleanup_env_vars();
    create_config_file();

    let config = TestFilePriority::load_config();

    assert_eq!(config.name, "TomlName");
    assert_eq!(config.timeout, 400);

    cleanup_config_file();
}

#[test]
fn default_without_config() {
    cleanup_env_vars();
    cleanup_config_file();

    let config = TestPriority::load_config();

    assert_eq!(config.name, "DefaultName");
    assert_eq!(config.id, 1);
}

#[test]
fn skip_env_with_file() {
    setup_env_vars();
    create_config_file();

    let config = TestFilePriority::load_config();
    assert!(config.skipped);

    cleanup_env_vars();
    cleanup_config_file();
}
