use trybuild;

#[test]
#[cfg_attr(not(nightly), ignore)]
fn nightly_ui() {
    let t = trybuild::TestCases::new();
    t.compile_fail("tests/nightly_ui/*.rs");
}
