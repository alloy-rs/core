#[rustversion::attr(nightly, ignore = "type.rs depends way too much on compiler internals")]
#[rustversion::attr(not(nightly), ignore)]
#[cfg_attr(any(target_os = "windows", miri), ignore)]
#[test]
fn ui() {
    let t = trybuild::TestCases::new();
    t.compile_fail("tests/ui/*.rs");

    macro_rules! feature_tests {
        ($($f:literal),* $(,)?) => {$(
            #[cfg(feature = $f)]
            t.compile_fail(concat!("tests/ui/features/", $f, "/*.rs"));
            #[cfg(not(feature = $f))]
            t.compile_fail(concat!("tests/ui/features/not(", $f, ")/*.rs"));
        )*};
    }

    feature_tests!("json");
}
