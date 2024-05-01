use confgr::prelude::*;
use std::env;

use std::fs::File;
use std::io::Write;

#[derive(Config, Default)]
#[config(env_path = "CONFIG_PATH_ENV", path = "tests/common/path.toml")]
struct TestPathEnvAndValidPath {
    name: String,
}

#[derive(Config, Default)]
#[config(env_path = "CONFIG_PATH_ENV", path = "nonexistent.toml")]
struct TestPathEnvAndInvalidPath {
    name: String,
}

#[derive(Config, Default)]
#[config(env_path = "CONFIG_PATH_ENV", default_path = "test.toml")]
struct TestPathEnvWithDefault {
    name: String,
}

#[derive(Config, Default)]
#[config(env_path = "CONFIG_PATH_ENV")]
struct TestPathEnv {
    name: String,
}

#[derive(Config, Default)]
#[config(default_path = "tests/common/path.toml")]
struct TestDefaultPathValid {
    name: String,
}

#[derive(Config, Default)]
#[config(default_path = "nonexistent.toml")]
struct TestDefaultPathInvalid {
    name: String,
}

fn setup_test_config_files() {
    let path_contents = r#"
            name = "Path"
        "#;

    let path_env_contents = r#"
            name = "PathEnv"
        "#;

    let mut path_file = File::create("tests/common/path.toml").unwrap();
    writeln!(path_file, "{}", path_contents).unwrap();

    let mut path_env_file = File::create("tests/common/path_env.toml").unwrap();
    writeln!(path_env_file, "{}", path_env_contents).unwrap();
}

fn setup_default_config_file() {
    let default_path_contents = r#"
            name = "DefaultPath"
        "#;

    let mut default_path_file = File::create("tests/common/default.toml").unwrap();
    writeln!(default_path_file, "{}", default_path_contents).unwrap();
}

fn cleanup_test_config_files() {
    std::fs::remove_file("tests/common/path.toml").unwrap();
    std::fs::remove_file("tests/common/path_env.toml").unwrap();
}

#[test]
fn path_env_valid() {
    setup_test_config_files();
    env::set_var("CONFIG_PATH_ENV", "tests/common/path_env.toml");

    let config = TestPathEnvAndValidPath::load_config();

    assert_eq!(config.name, "PathEnv");

    env::remove_var("CONFIG_PATH_ENV");
    cleanup_test_config_files();
}

#[test]
fn invalid_path_env_continues_with_valid_path() {
    setup_test_config_files();
    env::set_var("CONFIG_PATH_ENV", "nonexistent_path.toml");

    let config = TestPathEnvAndValidPath::load_config();

    assert_eq!(config.name, "Path");

    env::remove_var("CONFIG_PATH_ENV");
    cleanup_test_config_files();
}

#[test]
#[should_panic]
fn invalid_path_env_fails_with_invalid_path() {
    env::set_var("CONFIG_PATH_ENV", "nonexistent_path.toml");

    let _config = TestPathEnvAndInvalidPath::load_config();

    env::remove_var("CONFIG_PATH_ENV");
}

#[test]
fn invalid_path_env_continues_without_path() {
    env::set_var("CONFIG_PATH_ENV", "nonexistent_path.toml");
    env::set_var("NAME", "EnvName");

    let config = TestPathEnv::load_config();

    assert_eq!(config.name, "EnvName");

    env::remove_var("CONFIG_PATH_ENV");
    env::remove_var("NAME");
}

#[test]
fn default_path_valid() {
    setup_default_config_file();

    let config = TestDefaultPathValid::load_config();

    assert_eq!(config.name, "");

    std::fs::remove_file("tests/common/default.toml").unwrap();
}

#[test]
fn default_path_invalid() {
    let config = TestDefaultPathInvalid::load_config();

    assert_eq!(config.name, "");
}

#[test]
fn env_path_with_default_path() {
    setup_test_config_files();
    setup_default_config_file();
    env::set_var("CONFIG_PATH_ENV", "nonexistent_path.toml");

    let config = TestPathEnv::load_config();

    assert_eq!(config.name, "");

    env::remove_var("CONFIG_PATH_ENV");
    cleanup_test_config_files();
    std::fs::remove_file("tests/common/default.toml").unwrap();
}
