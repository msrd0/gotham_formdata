use trybuild::TestCases;

#[test]
fn trybuild_ui() {
	let t = TestCases::new();
	t.compile_fail("tests/ui/*.rs");
}
