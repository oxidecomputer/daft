//! UI tests.
//!
//! This is a separate binary from `integration_test.rs` because
//! `integration_test.rs` actually uses the macro under consideration -- it
//! might fail to compile, but we still want to see what the UI tests say.
//!
//! (It might make sense to move integration_test.rs to the daft crate in the
//! future.)

#[test]
fn ui() {
    let t = trybuild::TestCases::new();
    t.pass("tests/fixtures/valid/*.rs");
    t.compile_fail("tests/fixtures/invalid/*.rs");
}
