use confgr::prelude::*;

mod common;

#[derive(Config, Default)]
#[config(prefix = "NESTED")]
struct Nested {
    #[config(prefix = "inner_override")]
    name: String,
    detail: String,
}

#[derive(Config, Default)]
#[config(prefix = "TEST")]
struct NestedTest {
    id: i32,
    #[config(nest)]
    nested: Nested,
}

#[test]
fn test_nested() {
    std::env::set_var("TEST_ID", "10");
    std::env::set_var("NESTED_DETAIL", "NestedDetail");
    std::env::set_var("INNER_OVERRIDE_NAME", "Inner");

    let config = NestedTest::load_config();

    assert_eq!(
        config.id,
        common::get_var("TEST_ID").parse::<i32>().unwrap()
    );
    assert_eq!(config.nested.detail, common::get_var("NESTED_DETAIL"));
    assert_eq!(config.nested.name, common::get_var("INNER_OVERRIDE_NAME"));
}
