# Coding standards for Rust

This document describes the coding standards chosen by [Dotfive](https://dotfive.co.uk)
for [Rust](https://www.rust-lang.org/) code, which are **highly opinionated**.
We choose to follow these standards instead of formatting with `rustfmt`, mainly
for reasons of readability.

It is intended to be a living document, and will be updated as necessary. It is
not intended to be a comprehensive guide to Rust, but rather a set of standards
that should be followed when writing code for Dotfive projects. Notably, when
contributing to other codebases, such as client or community projects, we follow
the rules they have defined rather than ones in this document.

For the most part, these standards are based on common Rust community standards,
but there are some key differences. Additionally, this document clarifies our
approach in areas that are non-standardised and subjective.

This document is accompanied by example Rust files, and so this whole repository
is a working example of the standards described here, in the form of a
compilable, testable Rust project.

**Table of contents**

  - [Variation and evolution](#variation-and-evolution)
  - [IDEs and editors](#ides-and-editors)
      - [Fonts](#fonts)
      - [Highlighting](#highlighting)
  - [Code formatting](#code-formatting)
      - [Indentation](#indentation)
      - [Alignment](#alignment)
      - [Line breaks](#line-breaks)
      - [Line length](#line-length)
      - [Whitespace](#whitespace)
      - [Trailing commas](#trailing-commas)
      - [Comments](#comments)
          - [Structural comments](#structural-comments)
          - [Whole-line comments](#whole-line-comments)
          - [Inline comments](#inline-comments)
  - [Code linting](#code-linting)
      - [Complete configuration](#complete-configuration)
      - [Standard Rust compiler lints](#standard-rust-compiler-lints)
          - [Future compatibility lints](#future-compatibility-lints)
          - [Deprecated approach lints](#deprecated-approach-lints)
          - [Unused code lints](#unused-code-lints)
          - [Cherry-picked lints](#cherry-picked-lints)
      - [Clippy lints](#clippy-lints)
          - [Clippy lint categories](#clippy-lint-categories)
          - [Clippy cargo lints](#clippy-cargo-lints)
          - [Clippy nursery lints](#clippy-nursery-lints)
          - [Clippy pedantic lints](#clippy-pedantic-lints)
          - [Clippy restriction lints](#clippy-restriction-lints)
          - [Clippy configuration](#clippy-configuration)
      - [Giving reasons](#giving-reasons)
  - [Filesystem layout](#filesystem-layout)
  - [Code documentation](#code-documentation)
      - [Structs](#structs)
      - [Enums](#enums)
      - [Traits](#traits)
      - [Functions and methods](#functions-and-methods)
      - [Macros](#macros)
      - [Modules](#modules)
      - [Tests](#tests)
  - [General documentation](#general-documentation)
  - [Testing](#testing)
      - [Unit tests](#unit-tests)
      - [Integration tests](#integration-tests)
      - [Compilation tests](#compilation-tests)
  - [Version control](#version-control)
  - [Development process](#development-process)
  - [Templates and examples](#templates-and-examples)
      - [Root directory](#root-directory)
      - [`src`](#src)


## Variation and evolution

The standards described in this document are not set in stone. They are intended
to be a starting point, and will evolve over time. There will be times when
deviating from the standards is the right thing to do, and that is fine,
providing there is a justifiable reason for doing so. The standards should be
followed unless there is a good reason not to.

If you are working on one of our projects and feel that the standards should be
changed, then please raise a question describing the change you would like to
see. If you feel strongly about it, then please also provide a pull request for
this repository. We will review the changes, and if we agree with them then we
will merge them in.

However, until the point that any suggested changes are approved and merged, the
current standards described in this document should be followed. You may
personally dislike some of them, and that is absolutely fine — it is impossible
to please everyone. It is important that we have a consistent approach to
writing code, and so we must all follow the same standards. You are free to use
your own preferred standards in your own projects, but **when contributing to
Dotfive projects, you must follow the standards described here**.

If any suggestions are rejected then we will explain why, and we will be happy
to discuss the reasons for the decision. We will not reject suggestions without
good reason, but ultimately it is a very opinionated process, and someone has to
make the final decision. If we do reject a suggestion, we expect that to be
accepted with good grace and not subject to argument or further debate, unless
there is some further information that has not been considered.


## IDEs and editors

Which IDE or editor is used will be fairly important in order to avoid
unnecessary friction in following certain aspects of the standards. It does not
matter exactly which tool is chosen, but it should at the very least be able to
be configured to support the indentation and whitespace requirements. Preferably
it will support Rust syntax and be able to validate Rust code, and ideally it
will allow for the configuration of our custom comment headers.

Our recommended IDE is [RustRover](https://www.jetbrains.com/rust/), which is
available from [JetBrains](https://www.jetbrains.com/). It is a commercial
product, but is currently available as a free preview until full release.

We do not currently recommend [Visual Studio Code](https://code.visualstudio.com/),
simply because its interaction with Rust is not as smooth as it could be, and it
misses out on many of the advantages provided by JetBrains IDEs. However, it is
a perfectly viable alternative, and is free.

A worthy mention is [Sublime Text](https://www.sublimetext.com/), which is a
commercial product, but is free to evaluate. It is a very powerful editor, and
is highly configurable, but it does not have the same level of support for Rust
as the JetBrains IDEs.

Some of the standards described in this document have settings provided for
JetBrains IDEs. If you choose to use a different tool then you will need to
configure it to match the settings described here, or otherwise perform the
necessary actions manually.

*Note: There are some screenshots showing our standards in action in the
[`screenshots`](screenshots/) directory. See the [Highlighting](#highlighting)
section for more details.*

### Fonts

Our custom comment headers use symbols from [Font Awesome](https://fontawesome.com/),
which is available in the [Nerd Font](https://www.nerdfonts.com/) range. We
therefore suggest that you pick one of the Nerd Fonts for your editor, so that
you are able to see the symbols correctly.

As an aside, it's worth also configuring your terminal with a Nerd Font too, so
that you can see the symbols correctly in any command-line interactions.

### Highlighting

If you are using a JetBrains IDE, then you can import the provided settings from
the `IDEs/JetBrains` directory in this repository. This will configure the IDE
to use the correct highlighting for the custom comment headers.

Note that the settings rely upon you having the [Better Highlights
plugin](https://plugins.jetbrains.com/plugin/12895-better-highlights) installed.
This is a free plugin, and is available from the JetBrains plugin repository. It
is not necessary to use this plugin, but if you do not then you will not see the
custom comment headers highlighted correctly.

There are some screenshots showing our standards in action with this
highlighting applied in the [`screenshots`](screenshots/) directory:

  - [Main code routine](screenshots/main.png)
  - [Enums](screenshots/foo-enums.png)
  - [Structs](screenshots/foo-structs.png)
  - [Methods](screenshots/foo-methods.png)
  - [All styles](screenshots/styles.png)


## Code formatting

This section refers to the formatting of Rust code, rather than Markdown files.
The formatting of Markdown files is covered in a [later section](#general-documentation).

### Indentation

We use **tabs for indentation**, with a tab width of 4 spaces. Indentation is
considered to be any whitespace that occurs at the start of a line. Once a
non-whitespace character has occurred, indentation is considered to have ended
(but see the notes on [comments](#comments) for some specific rules relating to
them).

Of particular note here is that example code in inline Rustdoc documentation
should be indented by 4 spaces, and should not use tabs.

### Alignment

We use **spaces for alignment**. Alignment is considered to be the vertical
positioning of elements of code, such as assignment operators, that occur
part-way through a line of code.

We use alignment to improve readability, and to make it easier to spot
differences between lines of code. We align assignment operators, and the fields
in struct definitions. If there are similar elements on consecutive lines of
code, then we align them to make it easier to read.

Please line up assignment operators in nearby lines of code, unless there is a
good reason not to.

*Note: Remember that the intent is to **improve** readability, and not impede
it. If aligning certain elements makes the code harder to read, then that might
imply that the alignments should be constrained to a more localised area.*

The one major downside of alignment is that it can cause lines to change due to
whitespace adjustments when they have no functional differences. This can make
it harder to see what has actually changed in a commit. However, we feel that
the benefits of alignment outweigh this downside, the reason being that
maintaining readable code is more important than avoiding the slight pollution
of commits.

##### Examples

This is an example of how to align assignment operators:

```rust
let data    = vec![1, 2, 3];
let average = 0.0;
```

This is an example of how to align the `=>` in match arms:

```rust
match result {
    Ok(value)  => {},
    Err(error) => {},
}
```

This is an example of how to align the fields in struct initialisation:

```rust
Point {
    x:        0,
    y:        0,
    settings: Settings::default(),
};
```

This is an example of how to align similar terms in consecutive lines of code:

```rust
let a = foo + bar * baz / 100.0;
let b = a   + bar * baz /  10.0;
```

### Line breaks

We use **Unix-style line breaks** (LF, `\n`) rather than Windows-style line
breaks (CRLF, `\r\n`). This is the default for Rust, so you should not need to
do anything special to achieve this.

### Line length

We recommend column guides at 80 and 120 characters. For the most part, you
should aim to keep lines of code to 80 characters or less, but 120 is considered
to be the maximum. In very rare cases it might be that readability is improved
by going beyond 120 characters, but this should be avoided if possible.

### Whitespace

You should configure your editor to NOT remove trailing whitespace. You should
not have any at the end of lines of code, but comment headers rely on trailing
whitespace, and we prefer to maintain indentation on blank lines.

### Trailing commas

As is the standard convention in Rust, we use trailing commas in lists of items.
This means that you should always have a comma after the last item in the list.
This improves compatibility with version control systems, as a new line can be
added without modifying the previous one by adding a comma.

The exception to this rule is where a new item is impossible, which is deemed to
be the case when using the `..` operator, for instance to provide defaults. In
this case, the trailing comma is omitted, as this will always be the last item.

### Comments

Broadly speaking, there are structural comments, non-structural comments, and
Rustdoc documentation comments. Structural comments are used to create sections,
and non-structural comments are used for programmer notes. Rustdoc documentation
comments are used to create browsable documentation for the code.

This section deals with the strucural and non-structural comments (using `//`),
rather than Rustdoc documentation comments (using `///` and `//!`). For Rustdoc
documentation comments, see the [Code documentation](#code-documentation)
section.

Non-structural comments in code fall into two categories: whole-line comments,
and inline comments. Whole-line comments are comments that occur on a line by
themselves, and inline comments are comments that occur on the same line as
code.

#### Structural comments

Structural comments are used for creating sections, and have special formatting
symbols to allow colouring to be applied to make them stand out. They all start
with a comment symbol (`//`), followed by a special symbol, followed by *two*
tabs, followed by the comment text. The special symbols are:

  - `` - Divider
  - `` - Section																	
  - `` - Sub-section																
  - `` - Function																
  - `§` - Trait																	
  - `` - Struct																	
  - `` - Macro																	
  - `` - Enum																	
  - `` - Error
  - `` - Test																
  - `` - Method																
  - `` - Properties															
  - `` - Logic section									
  - `` - Logic/properties sub-section					
  - `` - Line of interest								
  - `` - Query		
  - `` - Unsafe		

*Note that you may need to view these in the Markdown source code, using an
appropriate Nerd Font, to see the symbols correctly. A [screenshot of the
symbols](screenshots/symbols.png) is also available.*

These each belong to a category, and the category determines the trailing
whitespace requirements.

  - Divider: No trailing whitespace.
  - Query, unsafe: Tabs to make the block 30 characters wide.
  - Everything else: Tabs to align to the 80th column.

Some of the structural comments are mandatory, such as those included before
core constructs. The logic sections are optional, and are useful for breaking up
longer sections of code to make them easier to read.

##### Examples

You can see these demonstrated in the `examples/styles.rs` file, and there is a
[screenshot showing all the styles in action](screenshots/styles.png) in the
[`screenshots`](screenshots/) directory.

*Note: For more screenshots see the [Highlighting](#highlighting) section.*

#### Whole-line comments

Whole-line comments should be composed of the comment symbol (`//`) followed by
a tab, followed by the comment text. They will quite often span multiple lines,
in which case each line repeats the same prefix. The rationale is that the
comment symbol is "invisible" as it denotes the nature of the line, and so the
indentation rules are still in effect for the comment text.

Whole-line comments are dealt with as a block for line-length purposes. The rule
is that if the comment symbol is located at the zeroth or first tab stop, then
the comment text should be wrapped at the 80th column. If the comment symbol is
located at the second tab stop or beyond, then the comment text should be
wrapped at whichever column would be the 80th column if the comment symbol was
at the first tab stop. The best way to do this is to write your comment text
when the comment symbol is at the first tab stop, and then indent the comment
block to the correct tab stop afterwards. The rationale behind this is that code
is often refactored, and things move around. It is annoying to have to reflow
comments every time this happens, plus for readability a set width is nicest for
the comment blocks. The only other consideration is that the comment text should
go beyond the 120th column — if it will do so by following the above rules, then
this is likely an indication that your code is nested too deeply, and you should
consider refactoring it.

Comments should be used sparingly, and only when necessary, to help explain code
that is otherwise difficult to understand, or, more likely, the reason for a
particular approach being taken. (Code that is hard to understand is often, but
not always, a sign of poorly-written code, so refactoring should be considered
ahead of merely commenting.) Comments should typically not be used to explain
what the code is doing, but rather why it is doing it. They should be used to
explain the rationale behind a particular approach, or to explain the purpose of
a particular function or module.

##### Examples

This is an example of a whole-line comment:

```rust
//  This is a whole-line comment. It is composed of the comment symbol followed
//  by a tab, followed by the comment text. It spans multiple lines, but each
//  line repeats the same prefix.
```

#### Inline comments

Inline comments should be separated from the code by two or more spaces,
followed by the comment symbol (`//`), followed by two spaces, followed by the
comment text. If there are other inline comments in the same region of code,
they should be vertically aligned. Inline comments are not allowed to span
multiple lines.

##### Examples

This is an example of an inline comment:

```rust
let foo = bar * 10;  //  This is an inline comment.
```


## Code linting

There are two levels of linting that are relevant: the Rust compiler, and
Clippy, which is run via `cargo clippy`. Each has its own set of rules, and
the recommended configuration is described below.

### Complete configuration

The complete linting configuration is provided here, and described in more
detail in the following sections. It is recommended that you copy this code
block into your `main.rs` or `lib.rs` file, and it links back to this
documentation with the intent of decluttering your codebase.

```rust
//		Global configuration

//	For an explanation of the following configuration, see:
//	https://github.com/dotfive/standards-rs#code-linting

#![cfg_attr(feature = "reasons", feature(lint_reasons))]

//		Standard Rust compiler lints											
//	Future compatibility lints
#![deny(future_incompatible)]
//	Deprecated approach lints
#![deny(rust_2018_compatibility)]
#![warn(rust_2018_idioms)]
#![deny(rust_2021_compatibility)]
//	Unused code lints
#![warn(unused)]
//	Cherry-picked lints
#![forbid(
    unsafe_code,
    unsafe_op_in_unsafe_fn,
)]
#![deny(
    deprecated,
    deprecated_where_clause_location,
    incomplete_features,
    internal_features,
    macro_use_extern_crate,
    unknown_lints,
    unnameable_test_items,
    unreachable_pub,
)]
#![warn(
    let_underscore_drop,
    meta_variable_misuse,
    missing_copy_implementations,
    missing_debug_implementations,
    missing_docs,
    single_use_lifetimes,
    trivial_casts,
    trivial_numeric_casts,
    unused_crate_dependencies,
    unused_import_braces,
    unused_lifetimes,
    unused_qualifications,
    unused_results,
    unused_tuple_struct_fields,
    variant_size_differences,
)]
//		Clippy lints															
//	Clippy lint categories
#![warn(
    clippy::cargo,
    clippy::nursery,
    clippy::pedantic,
)]
//	Clippy cargo lints
#![deny(
    clippy::negative_feature_names,
    clippy::wildcard_dependencies,
)]
//	Clippy pedantic lints
#![cfg_attr(    feature = "reasons",  allow(clippy::module_name_repetitions, reason = "This is not required"))]
#![cfg_attr(not(feature = "reasons"), allow(clippy::module_name_repetitions))]
//	Clippy restriction lints
#![forbid(
    clippy::allow_attributes_without_reason,
    clippy::dbg_macro,
    clippy::exit,
    clippy::infinite_loop,
    clippy::missing_assert_message,
    clippy::missing_docs_in_private_items,
    clippy::mod_module_files,
    clippy::multiple_inherent_impl,
    clippy::panic_in_result_fn,
    clippy::str_to_string,
    clippy::string_to_string,
    clippy::tests_outside_test_module,
    clippy::unimplemented,
    clippy::unwrap_in_result,
)]
#![deny(
    clippy::clone_on_ref_ptr,
    clippy::empty_structs_with_brackets,
    clippy::error_impl_error,
    clippy::exhaustive_enums,
    clippy::exhaustive_structs,
    clippy::expect_used,
    clippy::float_cmp_const,
    clippy::fn_to_numeric_cast_any,
    clippy::format_push_string,
    clippy::get_unwrap,
    clippy::impl_trait_in_params,
    clippy::integer_division,
    clippy::lossy_float_literal,
    clippy::mem_forget,
    clippy::panic,
    clippy::print_stderr,
    clippy::print_stdout,
    clippy::rc_mutex,
    clippy::try_err,
    clippy::unwrap_used,
    clippy::wildcard_enum_match_arm,
)]
#![warn(
    clippy::absolute_paths,
    clippy::arithmetic_side_effects,
    clippy::as_underscore,
    clippy::decimal_literal_representation,
    clippy::default_numeric_fallback,
    clippy::deref_by_slicing,
    clippy::empty_drop,
    clippy::filetype_is_file,
    clippy::if_then_some_else_none,
    clippy::indexing_slicing,
    clippy::iter_over_hash_type,
    clippy::let_underscore_must_use,
    clippy::let_underscore_untyped,
    clippy::map_err_ignore,
    clippy::missing_asserts_for_indexing,
    clippy::mixed_read_write_in_expression,
    clippy::mutex_atomic,
    clippy::pattern_type_mismatch,
    clippy::pub_without_shorthand,
    clippy::rc_buffer,
    clippy::redundant_type_annotations,
    clippy::rest_pat_in_fully_bound_structs,
    clippy::same_name_method,
    clippy::semicolon_outside_block,
    clippy::shadow_reuse,
    clippy::shadow_same,
    clippy::shadow_unrelated,
    clippy::std_instead_of_core,
    clippy::string_lit_chars_any,
    clippy::string_slice,
    clippy::suspicious_xor_used_as_pow,
    clippy::todo,
    clippy::unnecessary_safety_comment,
    clippy::unnecessary_safety_doc,
    clippy::unneeded_field_pattern,
    clippy::unreachable,
    clippy::unseparated_literal_suffix,
    clippy::use_debug,
    clippy::verbose_file_reads,
)]
```

### Standard Rust compiler lints

The lints configured and described here are correct for Rust 1.74.0.

For further information on specific lints, see the [Rust compiler lint
documentation](https://doc.rust-lang.org/rustc/lints/index.html).

#### Future compatibility lints

At the time of writing, the only item from the `future_incompatible` category
not enabled by default is the `pointer_structural_match` lint. The lints in the
`future_incompatible` category are variously set to `warn` or `deny`, and as
referencing the category name affects this nuanced distinction, the single
`pointer_structural_match` lint should be set explicitly instead if a mix of
`warn` and `deny` is desired. However, as all code should be following the Rust
2021 approaches and also remain fully future-compatible, this lint category is
set to `deny`.

There are currently 40 lints in the `future_incompatible` category, so they are
not listed here.

```rust
#![deny(future_incompatible)]
```

#### Deprecated approach lints

##### Rust 2018 compatibility

The defaults for the items included in the `rust_2018_compatibility` category
are:

| **Lint**                                  | **Default** | **Applied** |
|-------------------------------------------|-------------|-------------|
| `absolute_paths_not_starting_with_crate`  | `allow`     | -> `deny`   |
| `anonymous_parameters`                    | `warn`      | -> `deny`   |
| `keyword_idents`                          | `allow`     | -> `deny`   |
| `tyvar_behind_raw_pointer`                | `warn`      | -> `deny`   |

Instead, all of these are set to `deny`, as all code should be following the
Rust 2021 approaches.
    
```rust
#![deny(rust_2018_compatibility)]
```

The defaults for the items included in the `rust_2018_idioms` category are:

| **Lint**                                  | **Default** | **Applied**           |
|-------------------------------------------|-------------|-----------------------|
| `bare_trait_objects`                      | `warn`      | -> `warn` (no change) |
| `elided_lifetimes_in_paths`               | `allow`     | -> `warn`             |
| `ellipsis_inclusive_range_patterns`       | `warn`      | -> `warn` (no change) |
| `explicit_outlives_requirements`          | `allow`     | -> `warn`             |
| `unused_extern_crates`                    | `allow`     | -> `warn`             |

Enabling `rust_2018_idioms` sets all of these to `warn`, as all code should be
following the Rust 2021 approaches.

```rust
#![warn(rust_2018_idioms)]
```

##### Rust 2021 compatibility

The defaults for the items included in the `rust_2021_compatibility` category
are:

| **Lint**                                  | **Default** | **Applied** |
|-------------------------------------------|-------------|-------------|
| `array_into_iter`                         | `warn`      | -> `deny`   |
| `bare_trait_objects`                      | `warn`      | -> `deny`   |
| `ellipsis_inclusive_range_patterns`       | `warn`      | -> `deny`   |
| `non_fmt_panics`                          | `warn`      | -> `deny`   |
| `rust_2021_incompatible_closure_captures` | `allow`     | -> `deny`   |
| `rust_2021_incompatible_or_patterns`      | `allow`     | -> `deny`   |
| `rust_2021_prefixes_incompatible_syntax`  | `allow`     | -> `deny`   |
| `rust_2021_prelude_collisions`            | `allow`     | -> `deny`   |

Instead, all of these are set to `deny`, as all code should be following the
Rust 2021 approaches.

```rust
#![deny(rust_2021_compatibility)]
```

#### Unused code lints

The defaults for the items included in the `unused` category are:

| **Lint**                                  | **Default** | **Applied**           |
|-------------------------------------------|-------------|-----------------------|
| `dead_code`                               | `warn`      | -> `warn` (no change) |
| `map_unit_fn`                             | `warn`      | -> `warn` (no change) |
| `path_statements`                         | `warn`      | -> `warn` (no change) |
| `redundant_semicolons`                    | `warn`      | -> `warn` (no change) |
| `unreachable_code`                        | `warn`      | -> `warn` (no change) |
| `unreachable_patterns`                    | `warn`      | -> `warn` (no change) |
| `unused_allocation`                       | `warn`      | -> `warn` (no change) |
| `unused_assignments`                      | `warn`      | -> `warn` (no change) |
| `unused_attributes`                       | `warn`      | -> `warn` (no change) |
| `unused_braces`                           | `warn`      | -> `warn` (no change) |
| `unused_doc_comments`                     | `warn`      | -> `warn` (no change) |
| `unused_extern_crates`                    | `allow`     | -> `warn`             |
| `unused_features`                         | `warn`      | -> `warn` (no change) |
| `unused_imports`                          | `warn`      | -> `warn` (no change) |
| `unused_labels`                           | `warn`      | -> `warn` (no change) |
| `unused_macro_rules`                      | `allow`     | -> `warn`             |
| `unused_macros`                           | `warn`      | -> `warn` (no change) |
| `unused_must_use`                         | `warn`      | -> `warn` (no change) |
| `unused_mut`                              | `warn`      | -> `warn` (no change) |
| `unused_parens`                           | `warn`      | -> `warn` (no change) |
| `unused_unsafe`                           | `warn`      | -> `warn` (no change) |
| `unused_variables`                        | `warn`      | -> `warn` (no change) |

Instead, all of these are set to `warn`.

```rust
#![warn(unused)]
```

#### Cherry-picked lints

##### Forbid

The following lints are set to `allow` by default, and have been changed to
`forbid`:

```rust
#![forbid(
	unsafe_code,
	unsafe_op_in_unsafe_fn,
)]
```

##### Deny

The following lints are set to `allow` by default, and have been changed to
`deny`:

```rust
#![deny(
	macro_use_extern_crate,
	unreachable_pub,
)]
```

**Note that `unreachable_pub` should be removed for binaries**, as it only makes
sense for libraries. The lint behaviour may well change in the future:

  - https://github.com/rust-lang/rust/issues/74970

Additionally, there are some that are currently unstable and should be added
once they become stable: `fuzzy_provenance_casts`, `lossy_provenance_casts`, and
`unnameable_types`.

The following lints are set to `warn` by default, and have been changed to
`deny`:

```rust
#![deny(
	deprecated,
	deprecated_where_clause_location,
	incomplete_features,
	internal_features,
	unknown_lints,
	unnameable_test_items,
)]
```

Additionally, there are some that are currently unstable and should be added
once they become stable: `unknown_or_malformed_diagnostic_attributes`.

##### Warn

The following lints are set to `allow` by default, and have been changed to
`warn`:

```rust
#![warn(
	let_underscore_drop,
	meta_variable_misuse,
	missing_copy_implementations,
	missing_debug_implementations,
	missing_docs,
	single_use_lifetimes,
	trivial_casts,
	trivial_numeric_casts,
	unused_crate_dependencies,
	unused_import_braces,
	unused_lifetimes,
	unused_qualifications,
	unused_results,
	unused_tuple_struct_fields,
	variant_size_differences,
)]
```

Additionally, there are some that are currently unstable and should be added
once they become stable: `must_not_suspend`.

### Clippy lints

The approach to configuring the various Clippy lints is as follows:

  - Lints set to `forbid` mean, "you should never do these things". These lints
    cannot be contextually allowed, and Clippy will never pass a build with the
    associated issues present. The code approaches have to be changed before the
    build will pass.

  - Lints set to `deny` mean, "you need to deal with these things". These lints
    can be contextually allowed, and either the code should be changed or the
    approach marked as allowed in context (with a reason) before the build will
    pass. This is to ensure that the situations have been properly thought about
    and handled.

  - Lints set to `warn` mean, "you need to be careful of these things". They
    will not fail the build, so code can potentially still ship, but they should
    be investigated and either fixed or allowed in context (with a reason).

The configuration is from the perspective of shipping production-ready code. The
warnings will not hold up local development, and are related to areas that will
generally not affect functionality, and so a decision can be taken as to whether
or not they should be fixed before shipping. The higher-level lints that cause
build errors are considered more important, as they have a functional effect, or
involve bad practices, or highlight areas that absolutely need to be carefully
considered before shipping. As the Clippy lints do not affect the standard Rust
compiler, local development can continue with `cargo build` and the running of
`cargo clippy` is taken to imply that there is an intent to prepare the code for
shipping.

For further information on specific lints, see the Clippy documentation:

  - https://doc.rust-lang.org/clippy/
  - https://rust-lang.github.io/rust-clippy/master/

#### Clippy lint categories

By default, the standard Clippy lint categories are applied as follows:

| **Lint**                                  | **Default** | **Applied**           |
|-------------------------------------------|-------------|-----------------------|
| `clippy::cargo`                           | `allow`     | -> `warn`/`deny`      |
| `clippy::complexity`                      | `warn`      | -> `warn` (no change) |
| `clippy::correctness`                     | `deny`      | -> `deny` (no change) |
| `clippy::nursery`                         | `allow`     | -> `warn`             |
| `clippy::pedantic`                        | `allow`     | -> `warn`             |
| `clippy::perf`                            | `warn`      | -> `warn` (no change) |
| `clippy::style`                           | `warn`      | -> `warn` (no change) |
| `clippy::suspicious`                      | `warn`      | -> `warn` (no change) |

Together, the categories that are enabled by default make up `clippy::all`. Due
to the nuanced nature of their application, they should be adjusted on a
per-category basis, or a per-lint basis, rather than adjusting `clippy::all`.

```rust
#![warn(
	clippy::cargo,
	clippy::nursery,
	clippy::pedantic,
)]
```

#### Clippy cargo lints

The following lints are set to `allow` by default, and have been changed to
`deny`:

```rust
#![deny(
	clippy::negative_feature_names,
	clippy::wildcard_dependencies,
)]
```

The remaining lints are set to `allow` by default, and have been changed to
`warn` at a category level.

#### Clippy nursery lints

There are no specific amendments to the `nursery` lints at present, beyond those
applied at a category level.

#### Clippy pedantic lints

The following lints are set to `allow` by default, get set to `warn` at the
`clippy::pedantic` category level, and have been changed back to `allow`:

```rust
#![allow(
    clippy::module_name_repetitions,
)]
```

Note that if using the `reasons` crate feature, as described in the [Giving
reasons](#giving-reasons) section below, then the `module_name_repetitions` lint
will need to be disabled by adding an exception in the following manner:

```rust
#![cfg_attr(    feature = "reasons",  allow(clippy::module_name_repetitions, reason = "This is not required"))]
#![cfg_attr(not(feature = "reasons"), allow(clippy::module_name_repetitions))]
```

This approach is already included in the [complete configuration](#complete-configuration)
provided above, so if you are using that, there is nothing else to do.

#### Clippy restriction lints

Note that the lints configured here assume that unsafe code has been forbidden
at a Rust compiler level. If working on a project with unsafe code, resulting in
that compiler lint being relaxed, then it is recommended that the various Clippy
lints relating to unsafe code be examined, and any relevant ones enabled.
Otherwise, they are not required.

##### Forbid

The following lints are set to `allow` by default, and have been changed to
`forbid`:

```rust
#![forbid(
    clippy::allow_attributes_without_reason,
    clippy::dbg_macro,
    clippy::exit,
    clippy::missing_assert_message,
    clippy::missing_docs_in_private_items,
    clippy::mod_module_files,
    clippy::multiple_inherent_impl,
    clippy::panic_in_result_fn,
    clippy::str_to_string,
    clippy::string_to_string,
    clippy::tests_outside_test_module,
    clippy::unimplemented,
    clippy::unwrap_in_result,
)]
```

##### Deny

The following lints are set to `allow` by default, and have been changed to
`deny`:

```rust
#![deny(
    clippy::clone_on_ref_ptr,
    clippy::empty_structs_with_brackets,
    clippy::error_impl_error,
    clippy::exhaustive_enums,
    clippy::exhaustive_structs,
    clippy::expect_used,
    clippy::float_cmp_const,
    clippy::fn_to_numeric_cast_any,
    clippy::format_push_string,
    clippy::get_unwrap,
    clippy::impl_trait_in_params,
    clippy::integer_division,
    clippy::lossy_float_literal,
    clippy::mem_forget,
    clippy::panic,
    clippy::print_stderr,
    clippy::print_stdout,
    clippy::rc_mutex,
    clippy::try_err,
    clippy::unwrap_used,
    clippy::wildcard_enum_match_arm,
)]
```

##### Warn

The following lints are set to `allow` by default, and have been changed to
`warn`:

```rust
#![warn(
    clippy::absolute_paths,
    clippy::arithmetic_side_effects,
    clippy::as_underscore,
    clippy::decimal_literal_representation,
    clippy::default_numeric_fallback,
    clippy::deref_by_slicing,
    clippy::empty_drop,
    clippy::filetype_is_file,
    clippy::if_then_some_else_none,
    clippy::indexing_slicing,
    clippy::let_underscore_must_use,
    clippy::let_underscore_untyped,
    clippy::map_err_ignore,
    clippy::missing_asserts_for_indexing,
    clippy::mixed_read_write_in_expression,
    clippy::mutex_atomic,
    clippy::pattern_type_mismatch,
    clippy::pub_without_shorthand,
    clippy::rc_buffer,
    clippy::redundant_type_annotations,
    clippy::rest_pat_in_fully_bound_structs,
    clippy::same_name_method,
    clippy::semicolon_outside_block,
    clippy::shadow_reuse,
    clippy::shadow_same,
    clippy::shadow_unrelated,
    clippy::std_instead_of_core,
    clippy::string_lit_chars_any,
    clippy::string_slice,
    clippy::suspicious_xor_used_as_pow,
    clippy::todo,
    clippy::unnecessary_safety_comment,
    clippy::unnecessary_safety_doc,
    clippy::unneeded_field_pattern,
    clippy::unreachable,
    clippy::unseparated_literal_suffix,
    clippy::use_debug,
    clippy::verbose_file_reads,
)]
```

Note that `clippy::missing_trait_methods` can be useful, but is not enabled by
default as it can be very noisy. It is recommended that it be enabled on a
per-trait basis, as required.

Similarly, the `clippy::min_ident_chars` and
`clippy::single_char_lifetime_names` lints can also be useful, but it is very
common in idiomatic Rust to use single-character variable and lifetime names,
and so these are not enabled by default as they cause a lot of noise and would
result in either unnecessary verbosity or a lot of linting exceptions. It may be
that they can be useful on a per-file basis.

Additionally, there are some that are currently unstable and should be added
once they become stable: `clippy::iter_over_hash_type` (coming in Rust 1.75).

#### Clippy configuration

Note that, in addition to the lints themselves, configuration options can be set
in a `clippy.toml` file. For instance, if using `clippy::min_ident_chars`, the
following configuration can be used to set the threshold to two characters, and
to allow the most-commonly-used acceptable short variable names:

```toml
min-ident-chars-threshold = 2
allowed-idents-below-min-chars = ["i", "Ok", "id", "ip", "to"]
```

The `clippy::min_ident_chars` lint is set to trigger at two characters by the
configuration above, i.e. variable names should be three characters or more. In
most cases, this will be suitable when combined with the most-commonly-used
acceptable short variable names listed, namely `i`, `id`, `ip`, and `to` (plus
`Ok` to handle the `Result::Ok` variant). However, there are many additional
possibilities, such as `x`, `y`, `z` for co-ordinates; `dy` `dx` for
differentials; and so on, plus it is idiomatic to use single-character variables
in many places such as closures, formatters, and conversion functions. If
self-explanatory short names such as these are common to a project then the
configuration could be adjusted to list them, or to decrease the threshold to
one character — if uncommon, then the lint could be disabled for a specific
function. The ultimate goal is to achieve a high standard of clarity and
readability, and if one- and two-character variable names are generally not
descriptive enough, this lint may prove useful. Still, due to the commonality
and idiomatic nature of single-character variable names, it is not enabled by
default.

There are currently no Clippy configuration options set by defualt as part of
the recommended coding standards setup.

### Giving reasons

When a lint is disabled, i.e. an exception is added to allow the associated
behaviour, it is important to give a reason for doing so. This can be done by
adding a comment, but it is much better to do so in a manner that is
enforceable. There is a Rust feature called `lint_reasons` that allows this, but
it is currently unstable. Discussions have come on a long way recently (i.e. in
2023), and it looks like it, along with the now-associated `#[expect()]`
attribute, will be stabilised in the near future.

Therefore, to take advantage of this, the following approach is recommended:

 1. Add the following to your `Cargo.toml` file:
    
    ```toml
    [features]
    reasons = []
    ```
    
    This will create a feature on your crate called `reasons`, which will be
    disabled by default.

 2. Add the following to your `main.rs` or `lib.rs` file:
    
    ```rust
    #![cfg_attr(feature = "reasons", feature(lint_reasons))]
    ```
    
    This will enable the `lint_reasons` Rust feature if the `reasons` crate
    feature is enabled. Note, this is included in the [complete configuration](#complete-configuration)
    provided above.

 3. When adding an exception for a lint, i.e. allowing the associated behaviour,
    instead of simply using `#[allow()]`, use the following approach:
    
    ```rust
    #[cfg_attr(    feature = "reasons",  allow(LINTNAME, reason = "EXPLANATION"))]
    #[cfg_attr(not(feature = "reasons"), allow(LINTNAME))]
    ```
    
    For instance, to allow unused code by disabling the `dead_code` lint:
    
    ```rust
    #[cfg_attr(    feature = "reasons",  allow(dead_code, reason = "This is not intended to be used yet"))]
    #[cfg_attr(not(feature = "reasons"), allow(dead_code))]
    ```
    
    Note, the same approach applies for file-level declarations, i.e.
    `#![allow()]`.
    
 4. When building, you can build normally by using `cargo build` and `cargo
    clippy`. Assuming you are on the `stable` Rust toolchain, the lints will be
    disabled, and the reasons will be ignored. Then, when you want to run a
    check that all reasons are present, you can use the following:
    
    ```sh
    cargo +nightly clippy --features reasons
    ```
    
    This will enable the `reasons` crate feature, which will enable the
    `lint_reasons` Rust feature, which will enable the `reason` attribute.
    Assuming you are using the recommended lints, the
    `clippy::allow_attributes_without_reason` lint will be enabled, which will
    cause Clippy to fail if any reasons are missing.

This approach only requires a single additional line of code per lint, and
provides a way to enforce the presence of reasons. In that regard it is no more
verbose than adding a comment, but benefits from linting checks. It is also easy
to amend when the Rust feature is eventually stabilised.

Note that in order for this approach to work, you will need to have the
`nightly` Rust toolchain installed. You can do this by running `rustup install
nightly`. You do not have to switch over to the `nightly` toolchain, as the
commands above will use it only when specifically requested.

It is advisable to develop using the `stable` toolchain, especially when writing
libraries, but keeping an eye on the future of Rust and discretely using certain
features can be very beneficial.


## Filesystem layout

A typical Rust project should have the following structure:

  - `src/`
    - `main.rs` or `lib.rs`
    - *other modules*
    - `tests`
      - *module tests*
  - `tests/`
    - *integration tests*
  - `Cargo.toml`
  - `README.md`
  - `LICENSE`

A multi-crate workspace should place each crate under a `crates/` directory,
with the workspace `Cargo.toml` file at the root of the repository. The crate
should each follow normal structure in their own subdirectories.

If a module grows too large, it should be split into multiple files. These
should be placed in a subdirectory named after the module, and the module should
publish whatever internals are needed. However, avoid deeply-nested submodules,
as they complicate the crate structure. Typically a module should not have more
than two levels of modules (i.e. one nested level of submodules).

We take the approach of placing unit tests in separate files to the code they
test, and making the tests submodules. For more information on this, see the
[Testing](#testing) section.


## Code documentation

Documentation should be written using Rustdoc documentation comments. These
comments are written using the documentation comment symbols (`///` and `//!`),
and should be placed immediately before the item they document (except for
`//!`, which documents the "thing" they are in). They should be written in
British English, in present imperative tense, and in the third person, avoiding
any personal pronouns.

You should assume that the reader of your documentation already knows Rust and
will understand your code, and therefore the focus should be to explain what was
built, how to use it, and anything unusual or not obvious about it.

You should always document all structs, struct fields, enums, enum variants,
traits, functions, methods, and macros. You should typically also document
modules, although this can be skipped if the module provides one primary struct
and the purpose is clear through its documentation. Tests do not require
documentation.

If you link to other items, you should use the `[]` syntax, and leave as much as
possible up to Rustdoc to generate the links. In other words, do not use HTML
links, do not manually specify anchors, and do not use the `[name](link)` syntax
at all unless you need to override the link. This is because Rustdoc will
automatically generate the correct links for you, and do so in a way that is not
otherwise achievable.

Wrap your documentation at the 80th column, regardless of whether it started as
indented or not (you should find that your documentation blocks are never
indented by more than one tab). The exception here is links: the title of the
link is required to be fully constrained inside the 80 columns, but the link URL
can extend past this. If the link URL would go beyond 120 columns, wrap before
the link title and place the entire link on its own line.

### Structs

Structs should be documented comprehensively for the most part, varying
depending on their importance.

Struct fields should have at least a minimal description.

### Enums

Most enums are fairly self-explanatory, and so only tend to require a minimal
description.

Enum variants should have at least a minimal description.

### Traits

Traits should be documented comprehensively, as they are the primary interface
for a type.

You should document trait methods on the trait, and not on the implementing
type. This is because the trait is the primary interface, and so the
documentation should be there.

### Functions and methods

Functions and methods should have a number of key sections that help inform
about their behaviour, which are not applicable to other items. These should be
presented in sections using Markdown headings. Not all of the sections are
always applicable, but think about which may be needed and include them if
they are relevant.

  - Parameters
  - Errors
  - Panics
  - Examples
  - See also

Notably, Markdown lists should follow two individual styles, depending on the
context. For parameters and see-also links, the list should use `*` style
bullets. However, for lists that are part of general documentation prose, they
should use `  - ` style bullets, i.e. two spaces for indentation, followed by a
dash, followed by a space.

The "See Also" section should always be a list of links to other items, and not
contain any prose.

*Note: You should document trait methods on the trait, and not on the
implementing type.*

### Macros

Macros should be documented in the same way as [functions and methods](#functions-and-methods),
as they are basically a special type of function.

### Modules

Modules do not require documentation if they provide one primary struct and the
purpose is clear through its documentation. Otherwise, they should be documented
comprehensively.

### Tests

Tests do not require Rustdoc documentation.


## General documentation

All documentation that is not related specifically to the code, and cannot live
inside the source code files as Rustdoc documentation, should exist in Markdown
files. The primary example of this is the main `README.md` file that all
projects should have in their root. If it is necessary to split the
documentation into multiple files, then these should be linked to from the main
`README.md` file, unless there is very good reason not to. This helps maintain
discoverability.

All Markdown should follow general Markdown formatting rules, in similar fashion
to the [Rustdoc documentation](#code-documentation). Broadly speaking, use
correct headings and other constructs, and align to 80 characters except for
links (link URLs are allowed to extend out further — their titles must stay
within the limit though).


## Testing

There are essentially three kinds of test that we are interested in: unit tests,
integration tests, and compilation tests. Compilation tests are a special case,
as they can only be achieved as a form of integration test, and so are placed
under that area of the filesystem.

### Unit tests

Unit tests should be placed in a `tests/` directory alongside/under the module
they are testing. So for instance, if your module is `src/foo.rs`, then your
unit tests should be in `src/tests/foo.rs`. The unit tests should be set up as a
submodule of the module they are testing, which allows testing of private
aspects of the module. This means that every module has a submodule called
`tests`.

Typically your tests should just sit directly inside the test module, but in
some cases it may be necessary or useful to create submodules in the tests
module.

Each unit test should test one conceptual "thing". That does not mean one unit
test per method, and not does it mean one assertion per method. Rather, it means
that you should consider the outcome of your test, and try to make it atomic. If
you have an assertion checking a valid state and another one checking an invalid
state, it may be best to create two test methods, each with just one assertion.
Conversely, if you have a whole load of assertions checking different valid
variations, or maybe different fields of a struct under one use case, then these
most likely belong together in the same test method.

Name your test methods after the thing they are testing, e.g. if you are testing
a method called `foo()` then call your test `foo()`. Any variations that need to
be tested should then add a suffix of two underscores, followed by a short
description of the concept — for instance, `foo__valid()` and `foo__invalid()`.

Try to avoid code duplication by placing common code, such as that used to set
up and prepare the test environment, into separate (non-test) functions. If your
common methods belong in the `foo::tests` module, then prefix their names with
two underscores, e.g. `__setup()`. However, if they have wider purpose then
place them into a top-level module called `tests`, in which they can exist
without the double-underscore prefix requirement.

### Integration tests

In Rust, integration tests are placed in a `tests` directory as a sibling to the
`src` directory. The tests run from here will not be able to test any of the
internals of the application, and should test it from the outside in.

The approach to integration tests should broadly follow the same rules as for
[unit tests](#unit-tests).

### Compilation tests

In some cases there may be things we need to test that, if they fail, would
result in compilation failures. In context, this is not the same as something
generating a panic at runtime — it is a panic at compile-time. Therefore, such
cases cannot be caught in the usual way by using `#[should_panic]`. In order to
test for expected compilation errors, we use [`trybuild`](https://crates.io/crates/trybuild).

The way that `trybuild` works is to wrap the compilation process and catch the
results. It then compares the results against ones already stored. If they
match, the test is successful. This means that testing for successful or
unsuccessful builds use exactly the same approach. It also means that a build
test is a special kind of integration test, and gets run as an integration test
from the integration tests folder. However, it gets *triggered* from a unit test
(usually — although it could also be triggered by an integration test).

We tend to add a `tests/compile_fail` subfolder, and place the files to run
there (along with the expected output), and then execute them from one function
in the unit test suite. Multiple compile tests can be run at once that way, if
needed. The normal rules should be followed when deciding what should constitute
a separate test and give rise to an individual integration test function.


## Version control

A full description of Git etiquette is currently outside the scope of this
document, but in basic summary you should create meaningful, semantic,
carefully-staged commits, with appropriate commit messages and descriptions,
following the Dotfive Git standards.

Specifically to the context of Rust projects, you should try not to push any
commits that individually fail tests. As far as possible, each commit should be
able to pass testing and linting checks. Certainly by the time your branch is
submitted for PR, all tests should be passing along with all linting checks.


## Development process

The general development process should go something like this:

  1. Write your Rust code, using idiomatic approaches, and obeying the Dotfive
     coding standards.
  2. Write documentation for all new code you produce, in line with the
     requirements in this document.
  3. Ensure that you have complete test coverage for your additions or changes.
     If you are writing new code, then ideally you will use TDD to the greatest
     extent possible. If fixing a bug, consider if you need to add new tests to
     check for the specific thing you have fixed.
  4. Run `cargo build`, `cargo test`, `cargo clippy`, and `cargo doc` to check
     that the application builds correctly, the tests pass, and that there are
     no linting or documentation errors to correct.
  5. Commit and push.

We do not expect there to be zero warnings, although it should be strived for.
When a codebase is in the process of development, there can often be parts that
are as-yet unused, and in that case it is fine to ignore the related warnings
about those areas. The following flag can be useful to disable those warnings in
such cases:

```sh
RUSTFLAGS="-A unused" cargo clippy
```

Similarly, there may be areas of code that generate warnings but for which there
is good reason to not follow the suggested changes. In such cases, if the
approach is considered permanent, add an annotation to disable the associated
warning for that specific instance.

Otherwise, try to clean things up to eliminate or at least minimise warnings.
And, it should go without saying — **your code should always compile**.


## Templates and examples

A number of example files are provided for reference, and to use as templates.
They demonstrate the coding styles and standards described in this document.

### Root directory

  - `Cargo.toml` - This provides a typical Rust project configuration file. The
    appropriate license should be chosen and placed into the repository. For
    open-source licenses, the `license` field can be used; for proprietary
    licenses, the `license-file` field needs to be used, and the required
    license file placed in the root of the repository as `LICENSE`.
  - `LICENSE-MIT` - The MIT license. This is a permissive open-source license.
  - `LICENSE-proprietary` - A proprietary license. This is a placeholder file
    that should be replaced with the actual license file if it differs.
  - `src` - The source code directory. This is where all the Rust code lives.
  - `tests` - The integration tests directory. This is not present in this
    repository, and should be created if needed.

### `src`

  - `main.rs` - A typical Rust application entry point. This is just a simple
    "hello, world" program, but shows a standard layout.
  - `foo.rs` - A typical Rust module file. This demonstrates a number of aspects
    of the coding style, and includes a main struct, a settings struct, an enum,
    an error, and Rustdoc documentation stubs.
  - `tests/foo.rs` - A typical unit test file. This demonstrates a standard
    layout in line with the required coding style, and includes some tests for
    the `Foo::new()` method.
  - `tests.rs` - A test helper module, which contains an example of a common
    test helper function. Note that because of the `#[cfg(test)]` declaration
    in the `main.rs` file, this module is only compiled when running tests, and
    is not otherwise available.
  - `styles.rs` - A list of all the comment header styles that are available,
    for quick reference.


