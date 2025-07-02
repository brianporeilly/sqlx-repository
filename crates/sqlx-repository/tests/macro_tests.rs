//! Macro compilation tests using trybuild
//!
//! These tests ensure that the RepositoryDerive macro compiles correctly
//! for valid inputs and produces appropriate error messages for invalid inputs.

#[test]
fn compile_pass_tests() {
    let test_cases = trybuild::TestCases::new();
    test_cases.pass("tests/macro_tests/compile_pass/*.rs");
}

#[test]
fn compile_fail_tests() {
    let test_cases = trybuild::TestCases::new();
    test_cases.compile_fail("tests/macro_tests/compile_fail/*.rs");
}
