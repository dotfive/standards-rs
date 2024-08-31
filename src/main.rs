//! An example Rust project.
//! 
//! This project does nothing useful. It is intended to demonstrate the Rust
//! coding standards described in this repository.



//		Global configuration

#![cfg_attr(feature = "reasons", feature(lint_reasons))]

//	Customisations of the standard linting configuration
#![cfg_attr(    feature = "reasons",  allow(unreachable_pub, reason = "Not useful in a binary crate"))]
#![cfg_attr(not(feature = "reasons"), allow(unreachable_pub))]

//	Lints specifically disabled for unit tests
#![cfg_attr(test, allow(
	non_snake_case,
	clippy::cognitive_complexity,
	clippy::exhaustive_enums,
	clippy::exhaustive_structs,
	clippy::expect_used,
	clippy::indexing_slicing,
	clippy::let_underscore_untyped,
	clippy::missing_assert_message,
	clippy::missing_panics_doc,
	clippy::must_use_candidate,
	clippy::panic,
	clippy::print_stdout,
	clippy::unwrap_in_result,
	clippy::unwrap_used,
))]



//		Modules

mod foo;

#[cfg(test)]
mod tests;



//		Packages

use foo::{Foo, Settings, Style};



//		Functions

//		main																	
#[cfg_attr(    feature = "reasons",  allow(clippy::print_stdout, reason = "This is an example app with demo output"))]
#[cfg_attr(not(feature = "reasons"), allow(clippy::print_stdout))]
fn main() {
	let example1 = Foo::new(
		Some(42),
		Settings::default(),
	);
	let example2 = Foo::new(
		None,
		Settings {
			Style: Style::Bar,
		},
	);
	match example1.settings().Style {
		Style::Standard => println!("Hello, number {}!", example1.id.unwrap_or(0)),
		Style::Bar      => println!("Hi number {}.",     example1.id.unwrap_or(0)),
	}
	match example2.settings().Style {
		Style::Standard => println!("Hello, world!"),
		Style::Bar      => println!("Hi world."),
	}
}


