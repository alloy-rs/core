#[test]
fn ui() {
    let t = trybuild::TestCases::new();
    t.compile_fail("tests/ui/*.rs");

    macro_rules! feature_tests {
        ($($f:literal),* $(,)?) => {$(
            #[cfg(feature = $f)]
            t.compile_fail(concat!("tests/ui/feature/", $f, "/*.rs"));
            #[cfg(not(feature = $f))]
            t.compile_fail(concat!("tests/ui/feature/not(", $f, ")/*.rs"));
        )*};
    }

    feature_tests!("json");
}
