# Changelog

All notable changes to this project will be documented in this file.

The format is based on [Keep a Changelog](https://keepachangelog.com/en/1.1.0/),
and this project adheres to [Semantic Versioning](https://semver.org/spec/v2.0.0.html).

The three crates in this repository (`struct_iterable`,
`struct_iterable_derive`, `struct_iterable_internal`) are versioned and
released in lockstep; this changelog covers all of them.

## [0.2.0] - Unreleased

The headline: `#![no_std]`, zero allocation, mutable access, and iteration as
custom trait objects. The common path needs no changes — `#[derive(Iterable)]`
plus `for (name, value) in x.iter()` plus `downcast_ref` compiles as-is — and
no 0.1 user is upgraded automatically (cargo's 0.x rule). See the migration
table below.

### Added

- `#![no_std]`: the whole crate (and its generated code) uses only `core`,
  with zero feature flags, zero allocation, and `#![forbid(unsafe_code)]`.
  `core::any::Any` *is* `std::any::Any` — same trait, same `TypeId`s — so
  `downcast_ref` call sites are untouched. Verified on
  `thumbv7em-none-eabihf`.
- `FieldIter<'_>`: the allocation-free iterator now returned by `iter()`.
  Exact-size, double-ended, fused, and `Clone` — a superset of what
  `std::vec::IntoIter` offered. Plus `IntoIterator for &dyn Iterable`.
- By-index and by-name field access on `Iterable`: `field_count()`,
  `field_at(index)`, and the provided `field(name)`.
- `IterableMut`, emitted by the same `#[derive(Iterable)]`: `field_at_mut(index)`,
  `field_mut(name)`, and `visit_fields_mut(closure)`. There is deliberately no
  `Iterator`-shaped `iter_mut` — it cannot be built from a safe, manually
  implementable trait without `unsafe`; the reasoning is documented on the
  trait.
- `IterableAs<Dyn>`: iterate fields as a caller-chosen trait object.
  `#[iterable(to(dyn Trait))]` (repeatable; extra bounds like
  `to(dyn Trait + Send)` allowed) yields fields as `&dyn Trait`, with a
  compile error spanned to any field that doesn't implement the trait. Works
  for structs borrowing non-`'static` data — which the `dyn Any` path never
  can. `#[iterable(no_any)]` suppresses the `dyn Any`-based impls. Every
  `Iterable` type is automatically `IterableAs<dyn Any>`, and
  `IterableAsExt::iter_fields_as::<dyn Trait>()` disambiguates structs that
  iterate as several trait objects.
- Tuple structs (fields named `"0"`, `"1"`, …) and unit structs (an empty
  iterator) can now be derived.
- `#[iterable(skip)]` field attribute: leaves a field out of iteration — also
  the escape hatch for borrowed fields on the `dyn Any` path.
- `#[iterable(rename = "kebab-name")]` field attribute: yield a different
  name string.

### Changed

- **Breaking:** `iter()` returns `struct_iterable::FieldIter<'_>` instead of
  `std::vec::IntoIter<(&'static str, &dyn Any)>`. This matters only if you
  wrote the return type out; every capability of the old iterator is
  preserved.
- **Breaking:** manual `impl Iterable` blocks must now provide `field_count`,
  `field_at`, and `iter` (whose body is always `FieldIter::new(self)`).
- **Breaking:** raw identifiers now yield their bare name: `r#type` yields
  `"type"` (was `"r#type"`), matching serde.
- The `Iterable` trait moved from `struct_iterable_internal` into the
  `struct_iterable` facade. `struct_iterable_internal` is now a deprecated
  compatibility shim (`pub use struct_iterable::*`) that receives this one
  final 0.2.0 release and no further ones.
- Mutable access builds on work by [@mjhouse](https://github.com/mjhouse) in
  [#3](https://github.com/decomoraes/rust_struct_iterable/pull/3) and
  [#4](https://github.com/decomoraes/rust_struct_iterable/pull/4).
- Custom trait iteration was prompted by
  [@kaimast](https://github.com/kaimast)'s
  [#6](https://github.com/decomoraes/rust_struct_iterable/pull/6) and
  [@srid](https://github.com/srid)'s
  [#5](https://github.com/decomoraes/rust_struct_iterable/issues/5).

### Fixed

- [#1](https://github.com/decomoraes/rust_struct_iterable/issues/1) —
  the remaining gap after 0.1.2's `'static` fixes: structs borrowing
  non-`'static` data can now iterate, via `#[iterable(to(dyn Trait))]` or by
  `#[iterable(skip)]`-ing the borrowed fields.
- [#2](https://github.com/decomoraes/rust_struct_iterable/issues/2) —
  mutable access to fields, via `IterableMut`.
- [#5](https://github.com/decomoraes/rust_struct_iterable/issues/5) —
  iterating fields as a user-chosen trait object, via `IterableAs` and
  `#[iterable(to(dyn Trait))]`.
- [#7](https://github.com/decomoraes/rust_struct_iterable/issues/7) —
  `no_std` support, with zero allocation.

### Migrating from 0.1

| Your 0.1 code | 0.2.0 status |
|---|---|
| `#[derive(Iterable)]` + `for (name, value) in x.iter()` + `downcast_ref::<T>()` | works unchanged |
| `.collect()`, `.len()`, `.rev()`, iterator adapters | works unchanged (`FieldIter` is exact-size, double-ended, fused, `Clone`) |
| `Box<dyn Iterable>` / `&dyn Iterable` | works unchanged |
| Named the return type `std::vec::IntoIter<...>` | change it to `struct_iterable::FieldIter<'_>` |
| Manual `impl Iterable` | implement `field_count`, `field_at`, and `iter` (the `iter` body is always `FieldIter::new(self)`) |
| Raw-identifier field names (`r#type`) | now yield the bare name: `"type"`, was `"r#type"` — matching serde |
| Generic, lifetime-parameterized, tuple, or unit structs that failed to compile | newly compile |

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

[0.2.0]: https://crates.io/crates/struct_iterable
[0.1.2]: https://crates.io/crates/struct_iterable/0.1.2
[0.1.1]: https://crates.io/crates/struct_iterable/0.1.1
[0.1.0]: https://crates.io/crates/struct_iterable/0.1.0
