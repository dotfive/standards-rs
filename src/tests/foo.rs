//	Lints specifically disabled for integration tests
#![cfg_attr(test, allow(
	non_snake_case,
	clippy::cast_lossless,
	clippy::cast_precision_loss,
	clippy::cognitive_complexity,
	clippy::default_numeric_fallback,
	clippy::exhaustive_enums,
	clippy::exhaustive_structs,
	clippy::expect_used,
	clippy::indexing_slicing,
	clippy::let_underscore_must_use,
	clippy::let_underscore_untyped,
	clippy::missing_assert_message,
	clippy::missing_panics_doc,
	clippy::must_use_candidate,
	clippy::panic,
	clippy::print_stdout,
	clippy::tests_outside_test_module,
	clippy::unwrap_in_result,
	clippy::unwrap_used,
	reason = "Not useful in integration tests"
))]



//		Packages

use super::*;
use crate::tests;



//		Tests

//		Foo																		

//		new																		
#[test]
fn new__set_id() {
	let foo = Foo::new(
		Some(42),
		Settings::default(),
	);
	assert_eq!(foo.id,             Some(42));
	assert_eq!(foo.settings.Style, Style::Standard);
}
#[test]
fn new__no_id() {
	let foo = Foo::new(
		None,
		Settings::default(),
	);
	assert!(foo.id.is_none());
	assert_eq!(foo.settings.Style, Style::Standard);
}


