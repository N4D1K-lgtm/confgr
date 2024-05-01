use confgr::prelude::*;
use std::{env, fs::File, io::Write};

#[derive(Config, Default)]
#[config(env_path = "CONFIG_ENV_PATH", path = "tests/common/path.toml")]
struct TestPathEnvAndValidPath {
    name: String,
}

#[derive(Config, Default)]
#[config(env_path = "CONFIG_ENV_PATH", path = "nonexistent.toml")]
struct TestPathEnvAndInvalidPath {
    name: String,
}

#[derive(Config, Default)]
#[config(env_path = "CONFIG_ENV_PATH", default_path = "test.toml")]
struct TestPathEnvWithDefault {
    name: String,
}

#[derive(Config, Default)]
#[config(env_path = "CONFIG_ENV_PATH")]
struct TestPathEnv {
    name: String,
}

#[derive(Config, Default)]
#[config(default_path = "tests/common/default.toml")]
struct TestDefaultPathValid {
    name: String,
}

#[derive(Config, Default)]
#[config(default_path = "nonexistent.toml")]
struct TestDefaultPathInvalid {
    name: String,
}

fn setup_files(file_name: &str, contents: &str) {
    let mut file = File::create(file_name).expect("Failed to create file");
    writeln!(file, "{}", contents).expect("Failed to write to file");
}

fn cleanup_file(file_name: &str) {
    std::fs::remove_file(file_name).expect("Failed to delete file");
}

#[test]
fn test_env_path_valid() {
    setup_files("tests/common/env_path.toml", r#"name = "EnvPath""#);
    env::set_var("CONFIG_ENV_PATH", "tests/common/env_path.toml");

    let config = TestPathEnvAndValidPath::load_config();
    assert_eq!(config.name, "EnvPath");

    cleanup_env_and_files("CONFIG_ENV_PATH", "tests/common/env_path.toml");
}

#[test]
fn test_invalid_env_path_continues_with_valid_path() {
    setup_files("tests/common/path.toml", r#"name = "Path""#);
    env::set_var("CONFIG_ENV_PATH", "nonexistent_path.toml");

    let config = TestPathEnvAndValidPath::load_config();
    assert_eq!(config.name, "Path");

    cleanup_env_and_files("CONFIG_ENV_PATH", "tests/common/path.toml");
}

#[test]
#[should_panic]
fn test_invalid_env_path_fails_with_invalid_path() {
    env::set_var("CONFIG_ENV_PATH", "nonexistent_path.toml");
    let _config = TestPathEnvAndInvalidPath::load_config();
    env::remove_var("CONFIG_ENV_PATH");
}

#[test]
fn test_default_path_valid() {
    setup_files("tests/common/default.toml", r#"name = "DefaultPath""#);
    let config = TestDefaultPathValid::load_config();
    println!("{:?}", TestDefaultPathValid::get_file_path());
    assert_eq!(config.name, "DefaultPath");
    cleanup_file("tests/common/default.toml");
}

#[test]
fn test_default_path_invalid() {
    let config = TestDefaultPathInvalid::load_config();
    assert_eq!(config.name, "");
}

#[test]
fn test_env_path_with_default_path() {
    setup_files("tests/common/path.toml", r#"name = "DefaultPath""#);
    env::set_var("CONFIG_ENV_PATH", "nonexistent_path.toml");

    let config = TestPathEnv::load_config();
    assert_eq!(config.name, "");

    cleanup_env_and_files("CONFIG_ENV_PATH", "tests/common/path.toml");
}

fn cleanup_env_and_files(env_var: &str, file_path: &str) {
    env::remove_var(env_var);
    cleanup_file(file_path);
}
