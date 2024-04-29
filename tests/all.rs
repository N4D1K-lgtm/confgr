use confgr::prelude::*;
use std::env::{remove_var, set_var};

#[derive(Config)]
#[config(prefix = "PRIORITY")]
pub struct TestPriority {
    #[config(key = "CUSTOM_KEY")]
    pub name: String,
    pub id: i32,
    pub timeout: u64,
}

impl Default for TestPriority {
    fn default() -> Self {
        Self {
            name: "DefaultName".to_string(),
            id: 1,
            timeout: 100,
        }
    }
}

#[derive(Config, Default)]
#[config(path = "tests/config.json", prefix = "PRIORITY")]
pub struct JsonTestPriority {
    #[config(key = "CUSTOM_KEY")]
    pub name: String,
    pub id: i32,
    pub timeout: u64,
}

#[derive(Config, Default)]
#[config(path = "tests/config.toml", prefix = "PRIORITY")]
pub struct TomlTestPriority {
    #[config(key = "CUSTOM_KEY")]
    pub name: String,
    pub id: i32,
    pub timeout: u64,
}

#[derive(Config, Default)]
#[config(prefix = "TEST")]
pub struct SeperatorTest {
    #[config(separator = "__")]
    pub name: String,
}

#[derive(Config, Default)]
#[config(prefix = "TEST")]
pub struct SkipTest {
    pub id: i32,
    #[config(skip)]
    pub ignored: bool,
}

#[derive(Config, Default)]
#[config(prefix = "TEST_NESTED")]
pub struct Nested {
    pub detail: String,
}

#[derive(Config, Default)]
#[config(prefix = "TEST")]
pub struct NestedTest {
    pub id: i32,
    #[config(nest)]
    pub nested: Nested,
}

#[cfg(test)]
mod tests {
    use super::*;
    use std::fs::File;
    use std::io::Write;

    fn setup_env_vars() {
        set_var("CUSTOM_KEY", "EnvName");
        set_var("PRIORITY_ID", "20");
        set_var("PRIORITY_TIMEOUT", "300");
    }

    fn cleanup_env_vars() {
        remove_var("CUSTOM_KEY");
        remove_var("PRIORITY_ID");
        remove_var("PRIORITY_TIMEOUT");
    }

    fn create_config_file(data: &str, file_path: &str) {
        let mut file = File::create(file_path).unwrap();
        writeln!(file, "{}", data).unwrap();
    }

    #[test]
    fn test_env_over_config_file_and_default() {
        setup_env_vars();
        let config = TestPriority::load_config();
        assert_eq!(config.name, "EnvName");
        assert_eq!(config.id, 20);
        assert_eq!(config.timeout, 300);
        cleanup_env_vars();
    }

    #[test]
    fn test_config_file_over_default_toml() {
        cleanup_env_vars();
        let toml_data = r#"
            name = "TomlName"
            id = 30
            timeout = 400
        "#;
        create_config_file(toml_data, "tests/config.toml");

        let config = TomlTestPriority::load_config();
        assert_eq!(config.name, "TomlName");
        assert_eq!(config.id, 30);
        assert_eq!(config.timeout, 400);

        std::fs::remove_file("tests/config.toml").unwrap();
    }

    #[test]
    fn test_config_file_over_default_config_file_and_json() {
        cleanup_env_vars();
        let json_data = r#"
            {
                "name": "JsonName",
                "id": 40,
                "timeout": 500
            }
        "#;
        create_config_file(json_data, "tests/config.json");

        let config = JsonTestPriority::load_config();
        assert_eq!(config.name, "JsonName");
        assert_eq!(config.id, 40);
        assert_eq!(config.timeout, 500);

        std::fs::remove_file("tests/config.json").unwrap();
    }

    #[test]
    fn test_default_values() {
        let config = TestPriority::default();
        assert_eq!(config.name, "DefaultName");
        assert_eq!(config.id, 1);
        assert_eq!(config.timeout, 100);
    }

    #[test]
    fn test_custom_separator() {
        set_var("TEST__NAME", "SeparatedValue");
        let config = SeperatorTest::load_config();
        assert_eq!(config.name, "SeparatedValue");
        remove_var("TEST__NAME");
    }

    #[test]
    fn test_skip_attribute() {
        set_var("TEST_ID", "10");
        set_var("TEST_IGNORED", "true");
        let config = SkipTest::load_config();
        assert_eq!(config.id, 10);
        assert!(!config.ignored);
        remove_var("TEST_ID");
        remove_var("TEST_IGNORED");
    }

    #[test]
    fn test_nested_config() {
        set_var("TEST_ID", "20");
        set_var("TEST_NESTED_DETAIL", "NestedDetail");
        let config = NestedTest::load_config();
        assert_eq!(config.id, 20);
        assert_eq!(config.nested.detail, "NestedDetail");
        remove_var("TEST_ID");
        remove_var("TEST_NESTED_DETAIL");
    }

    #[test]
    fn test_default_values_for_nested() {
        let config = NestedTest::default();
        assert_eq!(config.id, 0);
        assert_eq!(config.nested.detail, "");
    }

    #[test]
    fn check_file_fail() {
        assert!(TestPriority::check_file().is_err());
    }

    #[test]
    fn get_empty_file_path() {
        assert_eq!(TestPriority::get_file_path(), "");
    }

    #[test]
    fn check_file_success() {
        let toml_data = r#"
            name = "TomlName"
            id = 30
            timeout = 400
        "#;
        create_config_file(toml_data, "tests/config.toml");
        assert!(TomlTestPriority::check_file().is_ok());
        std::fs::remove_file("tests/config.toml").unwrap();
    }

    #[test]
    fn get_file_path() {
        assert_eq!(TomlTestPriority::get_file_path(), "tests/config.toml");
    }
}
