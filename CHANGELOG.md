# Changelog

All notable changes to this project will be documented in this file.

The format is based on [Keep a Changelog](https://keepachangelog.com/en/1.1.0/),
and this project adheres to [Semantic Versioning](https://semver.org/spec/v2.0.0.html).

The three crates in this repository (`struct_iterable`,
`struct_iterable_derive`, `struct_iterable_internal`) are versioned and
released in lockstep; this changelog covers all of them.

## [0.1.2] - Unreleased

### Added

- `#[iterable(crate = "path")]` container attribute: points the generated code
  at the crate that exports the `Iterable` trait, for facades renamed in
  `Cargo.toml` or re-exported from another crate.

### Fixed

- `#[derive(Iterable)]` now works on generic, lifetime-parameterized, and
  const-generic structs (the compile failures from issue #1). The generated
  impl carries an `Any` bound per field type, so it applies whenever every
  field type is `'static` (e.g. `Wrapper<String>`, `Borrowed<'static>`).
- The derive reports invalid input (enums, unions, tuple structs, unit
  structs, unsupported attributes) as spanned `syn::Error` compile errors
  pointing at the offending item, instead of panicking with
  "proc macro panicked".
- The generated code uses fully qualified paths throughout (hygiene): the
  derive no longer breaks when `Iterable`, `vec!`, or `Any` are shadowed at
  the call site.

### Changed

- Behavioral edge of the hygiene fix: generated code now names
  `::struct_iterable` explicitly — if you depend only on the `_derive` or
  `_internal` crates, or rename the facade, add `#[iterable(crate = "path")]`.
- Removed the unused `erased-serde` dependency from the derive crate
  (smaller dependency tree).
- Relicensed from MIT to MIT OR Apache-2.0.
- Repository overhauled: Cargo workspace with path dependencies, full test
  suite (integration, trybuild UI, macrotest expansion snapshots), CI,
  runnable examples, and rewritten documentation.

## [0.1.1] - 2023-05-10

### Fixed

- Documentation and metadata updates.

## [0.1.0] - 2023-05-10 [YANKED]

### Added

- `#[derive(Iterable)]` for structs with named fields.
- `Iterable` trait: `iter()` over `(&'static str, &dyn Any)`.

[0.1.2]: https://crates.io/crates/struct_iterable
[0.1.1]: https://crates.io/crates/struct_iterable/0.1.1
[0.1.0]: https://crates.io/crates/struct_iterable/0.1.0
