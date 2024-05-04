use confgr::prelude::*;
use serde::Deserialize;

#[derive(Debug, Deserialize, Clone)]
struct NonDefaultType {
    value: String,
}

fn default_non_default_type() -> NonDefaultType {
    NonDefaultType {
        value: "default_value".to_string(),
    }
}

#[derive(Config)]
struct TestConfig {
    #[config(skip)]
    foreign_type: NonDefaultType,
}

impl Default for TestConfig {
    fn default() -> Self {
        Self {
            foreign_type: default_non_default_type(),
        }
    }
}

#[test]
fn test_non_default_foreign_type_is_valid() {
    let config = TestConfig::load_config();
    assert_eq!(config.foreign_type.value, "default_value");
}
