#[test]
fn tests() {
    let t = trybuild::TestCases::new();
    t.pass("tests/01-parse-header.rs");
    t.pass("tests/02-parse-macro.rs");
}