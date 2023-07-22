#[test]
fn trybuild() {
    let t = trybuild::TestCases::new();
    t.compile_fail("tests/fail/derive/dependencies/enum.rs");
    t.compile_fail("tests/fail/derive/dependencies/fewer_than_2_fields.rs");
    // annoyingly the test below fails on stable due to a bug affecting the
    // generic's span. Otherwise, we could just:
    // t.compile_fail("tests/fail/**/*.rs");
    #[cfg(feature = "stable")]
    t.compile_fail("tests/fail/derive/dependencies/generics.rs");
    t.compile_fail("tests/fail/derive/dependencies/more_than_26_fields.rs");
    t.compile_fail("tests/fail/derive/dependencies/union.rs");
    t.compile_fail("tests/fail/derive/dependencies/unnamed_fields.rs");
    t.compile_fail("tests/fail/derive/graph/*.rs");
    t.compile_fail("tests/fail/derive/operation/*.rs");
    t.compile_fail("tests/fail/derive/value/*.rs");
    t.pass("tests/pass/**/*.rs");
}
