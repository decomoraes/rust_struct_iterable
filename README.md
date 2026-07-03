# struct_iterable

[![crates.io](https://img.shields.io/crates/v/struct_iterable.svg)](https://crates.io/crates/struct_iterable)
[![docs.rs](https://img.shields.io/docsrs/struct_iterable)](https://docs.rs/struct_iterable)
[![CI](https://github.com/decomoraes/rust_struct_iterable/actions/workflows/ci.yml/badge.svg)](https://github.com/decomoraes/rust_struct_iterable/actions/workflows/ci.yml)
![MSRV](https://img.shields.io/crates/msrv/struct_iterable)

One derive to iterate a struct's fields as `(name, value)` pairs.
No framework to adopt, no new concepts to learn: derive, loop, done —
in `#![no_std]`, with zero allocation, zero feature flags, and
`#![forbid(unsafe_code)]`.

## Quick start

```toml
[dependencies]
struct_iterable = "0.2.0"
```

```rust
use struct_iterable::Iterable;

#[derive(Iterable)]
struct Connection {
    host: String,
    port: u16,
}

let conn = Connection { host: "localhost".into(), port: 5432 };

for (name, value) in conn.iter() {
    if let Some(text) = value.downcast_ref::<String>() {
        println!("{name} = {text:?}");
    } else if let Some(number) = value.downcast_ref::<u16>() {
        println!("{name} = {number}");
    }
}
```

## What you get

- Field names as `&'static str`, yielded in declaration order.
- Field values as `&dyn Any` — `downcast_ref::<T>()` them back to the concrete
  types you know are in there.
- An allocation-free iterator, `FieldIter`: exact-size, double-ended, fused,
  `Clone`, and it works with every iterator adapter.
- By-name and by-index access: `field("name")` and `field_at(index)`.
- Mutation via `IterableMut` — same derive, no extra attribute:
  `field_mut("name")`, `field_at_mut(index)`, `visit_fields_mut(...)`.
- Custom trait objects: `#[iterable(to(dyn Trait))]` yields fields as
  `&dyn Trait` instead of `&dyn Any`, checked at compile time — and it works
  for structs borrowing non-`'static` data.
- `#![no_std]` with zero allocation and zero feature flags, verified on
  `thumbv7em-none-eabihf`. (`core::any::Any` *is* `std::any::Any`, so your
  `downcast_ref` call sites don't change.)
- Named-field structs, tuple structs (fields named `"0"`, `"1"`, …), and unit
  structs — including generic, lifetime-parameterized, and const-generic ones.
- `#[iterable(skip)]` and `#[iterable(rename = "kebab-name")]` field
  attributes.
- An object-safe trait: `Box<dyn Iterable>` supports the full API, including
  `.iter()`.
- Clear, spanned compile errors — not proc-macro panics — when the derive
  can't apply.

## Mutation

`#[derive(Iterable)]` also implements `IterableMut`: mutate fields by name,
by index, or by visiting them all.

```rust
use struct_iterable::{Iterable, IterableMut};

#[derive(Iterable)]
struct Settings {
    retries: u32,
    verbose: bool,
}

let mut settings = Settings { retries: 0, verbose: false };

// By name:
*settings.field_mut("retries").unwrap().downcast_mut::<u32>().unwrap() = 5;

// Or visit every field:
settings.visit_fields_mut(&mut |_name, value| {
    if let Some(flag) = value.downcast_mut::<bool>() {
        *flag = true;
    }
});

assert_eq!(settings.retries, 5);
assert!(settings.verbose);
```

There is deliberately no `Iterator`-shaped `iter_mut`: an iterator handing out
multiple live `&mut dyn Any` cannot be built from a safe, manually
implementable trait without `unsafe` aliasing contracts. The three shapes
above cover mutation with no unsafe code anywhere.

## Custom trait objects

`&dyn Any` gives you downcasting but demands `'static` — sometimes you would
rather iterate as a trait you actually want to call.
`#[iterable(to(dyn Trait))]` yields every field as `&dyn Trait`, with a
compile error (spanned to the offending field) if a field doesn't implement
it. Because `&dyn Trait` has no `'static` requirement, this also works for
structs that borrow non-`'static` data — the scenario the `dyn Any` path can
never support:

```rust
use struct_iterable::{Iterable, IterableAs};

trait Checkable {
    fn ok(&self) -> bool;
}

struct MaxJobs(u32);
struct Caches<'a>(&'a str);

impl Checkable for MaxJobs {
    fn ok(&self) -> bool { self.0 > 0 }
}
impl<'a> Checkable for Caches<'a> {
    fn ok(&self) -> bool { !self.0.is_empty() }
}

// `no_any` suppresses the dyn-Any-based impls, which `Caches<'a>`
// could never satisfy.
#[derive(Iterable)]
#[iterable(to(dyn Checkable), no_any)]
struct NixHealth<'a> {
    max_jobs: MaxJobs,
    caches: Caches<'a>,
}

let cache_url = String::from("https://cache.nixos.org");
let health = NixHealth {
    max_jobs: MaxJobs(4),
    caches: Caches(&cache_url),
};

for (name, check) in health.iter_as() {
    println!("{name}: {}", if check.ok() { "ok" } else { "FAILED" });
}
```

`to(...)` is repeatable — a struct can iterate as several trait objects, with
`IterableAsExt::iter_fields_as::<dyn Trait>()` to disambiguate — and extra
bounds are allowed: `to(dyn Trait + Send)`. Every `Iterable` type is
automatically `IterableAs<dyn Any>`.

## Limitations

Honest and up front:

- **Structs only.** Named-field, tuple, and unit structs all derive; enums and
  unions are rejected with a clear compile error.
- **The `dyn Any` path needs `'static` field types.** `Any` requires
  `'static`, so the derived `Iterable`/`IterableMut` impls apply where every
  non-skipped field type is `'static`: `Config<String>` and
  `Borrowed<'static>` iterate, `Borrowed<'a>` does not. Two escape hatches for
  borrowed data: `#[iterable(skip)]` the borrowed fields, or iterate through
  your own trait with `#[iterable(to(dyn Trait))]` — `&dyn Trait` has no
  `'static` requirement.

## Renamed the crate?

The generated code refers to `::struct_iterable` by name (proc macros have no
`$crate`). If you rename the facade in `Cargo.toml`, or re-export it from
another crate, point the derive at the right path:

```rust
use struct_iterable as si;

#[derive(si::Iterable)]
#[iterable(crate = "si")]
struct Point {
    x: i32,
    y: i32,
}
```

## Comparison with alternatives

| Crate | What it does | Pick it when |
|---|---|---|
| **struct_iterable** | `(name, &dyn Any)` iteration plus mutation, by-name access, and custom trait objects — one derive, no_std, zero allocation | You want the 30-second version: derive, loop, downcast |
| [bevy_reflect](https://crates.io/crates/bevy_reflect) | Full reflection framework: `DynamicStruct`, enums, runtime construction, type registry | You need real reflection (enums, runtime construction, a registry) or you're in Bevy |
| [facet](https://crates.io/crates/facet) | Const shape data per type plus a reflection ecosystem (serialization, diffing, CLI) | You're building serializers or tooling on top of reflection |
| [metastruct](https://crates.io/crates/metastruct) | Per-struct macros over fields, zero dynamic dispatch, mutation support | You want compile-time codegen and no `dyn Any` |
| [field_access](https://crates.io/crates/field_access) | `field("name")` accessors with typed getters, mutation, no_std | You want by-name access with typed getters |
| [valuable](https://crates.io/crates/valuable) | Visitor-based structured values (the tracing ecosystem) | Structured logging |
| [struct-field-names-as-array](https://crates.io/crates/struct-field-names-as-array) | `const` array of field names only | You only need the names |

An honest pointer: if you need **enums**, **runtime construction**, or a
**type registry**, bevy_reflect is a real reflection framework and this crate
is deliberately not one — it stays a single derive you can read in an
afternoon.

## Migrating from 0.1

No 0.1 user is upgraded automatically (cargo's 0.x rule), and the common path
needs no changes: if you write `for (name, value) in x.iter()` and
`downcast_ref`, you change nothing.

| Your 0.1 code | 0.2.0 status |
|---|---|
| `#[derive(Iterable)]` + `for (name, value) in x.iter()` + `downcast_ref::<T>()` | works unchanged |
| `.collect()`, `.len()`, `.rev()`, iterator adapters | works unchanged (`FieldIter` is exact-size, double-ended, fused, `Clone`) |
| `Box<dyn Iterable>` / `&dyn Iterable` | works unchanged |
| Named the return type `std::vec::IntoIter<...>` | change it to `struct_iterable::FieldIter<'_>` |
| Called `.as_slice()` / `.as_mut_slice()` on the iterator | gone (they were `vec::IntoIter` inherent methods) — collect into a `Vec` first |
| Manual `impl Iterable` | implement `field_count`, `field_at`, and `iter` (the `iter` body is always `FieldIter::new(self)`) |
| Raw-identifier field names (`r#type`) | now yield the bare name: `"type"`, was `"r#type"` — matching serde |
| Named `struct_iterable_internal::Iterable` in an import or bound | bump `struct_iterable_internal` to 0.2.0 (deprecated shim) in the same update, or switch the path to `struct_iterable::Iterable` — leaving it at 0.1 gives you two distinct traits and E0277 |
| Another in-scope trait with a method named `field`, `field_count`, or `field_at` | `Iterable` now provides methods with those names; disambiguate with UFCS (`YourTrait::field(&x, ...)`) |
| Renamed the facade in `Cargo.toml`, or depend only on `_derive`/`_internal` | add `#[iterable(crate = "path")]` — generated code now names `::struct_iterable` by default |
| Generic, lifetime-parameterized, tuple, or unit structs that failed to compile | newly compile |

The `Iterable` trait now lives in this crate rather than in
`struct_iterable_internal`; the internal crate is a deprecated re-export shim.

## MSRV

The minimum supported Rust version is **1.71**, verified by a dedicated CI job.
Raising the MSRV is a minor-version bump, done only when a dependency forces it
or a meaningfully useful language feature justifies it.

## Semver policy

Pre-1.0, breaking changes happen only at 0.x version bumps, and each one is
called out in the [CHANGELOG](CHANGELOG.md) with a migration table.

The crates in this repository are versioned and released in lockstep. Depend
only on `struct_iterable` — `struct_iterable_derive` is an implementation
detail, and `struct_iterable_internal` is a deprecated compatibility shim
kept for 0.1-era dependents.

## License

Licensed under either of

- Apache License, Version 2.0 ([LICENSE-APACHE](LICENSE-APACHE) or
  <http://www.apache.org/licenses/LICENSE-2.0>)
- MIT license ([LICENSE-MIT](LICENSE-MIT) or
  <http://opensource.org/licenses/MIT>)

at your option.

### Contribution

Unless you explicitly state otherwise, any contribution intentionally submitted
for inclusion in the work by you, as defined in the Apache-2.0 license, shall be
dual licensed as above, without any additional terms or conditions.
