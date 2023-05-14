//! TESTING ONLY - DO NOT USE.

#[test]
#[cfg(test)]
fn ui_warm() {
	std::env::set_var("RUSTFLAGS", "--deny warnings");
	let t = trybuild::TestCases::new();

	t.compile_fail("src/warn/*.rs");
}

#[test]
#[cfg(test)]
fn ui_no_warn() {
	std::env::set_var("RUSTFLAGS", "--deny warnings");
	let t = trybuild::TestCases::new();

	t.pass("src/no-warn/*.rs");
}
