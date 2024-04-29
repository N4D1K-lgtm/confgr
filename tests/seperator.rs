use confgr::prelude::*;
use std::env;

#[derive(Config, Default)]
#[config(prefix = "TEST")]
pub struct SeparatorFieldTest {
    #[config(separator = "__")]
    pub custom_seperator: String,
    pub default_seperator: String,
}

fn get_var(key: &str) -> String {
    let msg = format!("{} must be set in .env", key);
    env::var(key).expect(&msg)
}

#[test]
fn test_custom_separator() {
    dotenv::from_path("tests/test.env").ok();

    let config = SeparatorFieldTest::load_config();
    assert_eq!(config.custom_seperator, get_var("TEST__CUSTOM_SEPERATOR"));
}
