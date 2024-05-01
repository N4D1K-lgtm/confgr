use confgr::prelude::*;

#[derive(Config, Default)]
#[config(prefix = "SKIP")]
pub struct SkipTest {
    pub id: i32,
    #[config(skip)]
    pub ignored: bool,
}

#[test]
fn skipped() {
    std::env::set_var("SKIP_IGNORED", "true");
    let config = SkipTest::load_config();

    assert!(!config.ignored);
}
