use confgr::prelude::*;

mod common;

#[derive(Config, Default)]
#[config(prefix = "TEST", separator = "_sep_")]
pub struct SeparatorTest {
    #[config(separator = "__")]
    pub field_sep: String,
    pub struct_sep: String,
}

#[test]
fn custom_separator() {
    std::env::set_var("TEST__FIELD_SEP", "field_sep");
    std::env::set_var("TEST_sep_STRUCT_SEP", "struct_sep");

    let config = SeparatorTest::load_config();

    assert_eq!(config.field_sep, common::get_var("TEST__FIELD_SEP"));
    assert_eq!(config.struct_sep, common::get_var("TEST_sep_STRUCT_SEP"));
}
