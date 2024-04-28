use confgr::prelude::*;
use smart_default::SmartDefault;

#[derive(Confgr, Clone, SmartDefault)]
#[config(prefix = "PREFIX", path = "tests/config")]
pub struct Test {
    #[config(key = "CUSTOM_KEY")]
    #[default = "World"]
    name: String,
    #[config(prefix = "APP")]
    #[default = 3]
    id: i32,
    #[config(nest)]
    nested: Nested,
    #[config(key = "TIMEOUT_MS")]
    #[default = 1000]
    timeout: u64,
    #[config(key = "FEATURE_ENABLED")]
    #[default = false]
    feature_enabled: bool,
    #[default = 1.5]
    ratio: f64,
    #[config(nest)]
    metadata: Metadata,
    #[config(skip)]
    #[default(Some("Unused".to_string()))]
    unused_field: Option<String>,
}

#[derive(Confgr, Default, Clone)]
pub struct Nested {
    name: String,
}

#[derive(Confgr, Default, Clone)]
#[config(prefix = "META", separator = "__")]
pub struct Metadata {
    description: String,
    version: i32,
}

#[test]
fn main() {
    dotenv::from_path("tests/test.env").ok();

    let config = Test::load_config();

    assert_eq!(config.name, "World");
    assert_eq!(config.id, 10);
    assert_eq!(config.timeout, 2000);
    assert!(config.feature_enabled);
    assert_eq!(config.ratio, 2.5);
    assert_eq!(config.metadata.description, "Example Metadata");
    assert_eq!(config.metadata.version, 1);
    assert_eq!(config.nested.name, "Nested From Toml");
}
