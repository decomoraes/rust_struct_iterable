# struct_iterable

[![crates.io](https://img.shields.io/crates/v/struct_iterable.svg)](https://crates.io/crates/struct_iterable)
[![docs.rs](https://img.shields.io/docsrs/struct_iterable)](https://docs.rs/struct_iterable)
[![CI](https://github.com/decomoraes/rust_struct_iterable/actions/workflows/ci.yml/badge.svg)](https://github.com/decomoraes/rust_struct_iterable/actions/workflows/ci.yml)
![MSRV](https://img.shields.io/crates/msrv/struct_iterable)

One derive to iterate a struct's fields as `(name, value)` pairs.
No framework to adopt, no new concepts to learn: derive, loop, done.

## Quick start

```toml
[dependencies]
struct_iterable = "0.1.2"
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
- A plain `std::vec::IntoIter` iterator: exact-size, double-ended, and it works
  with every iterator adapter.
- Support for generic, lifetime-parameterized, and const-generic structs (with
  the `'static` caveat below).
- An object-safe trait: `Box<dyn Iterable>` works.
- Clear, spanned compile errors — not proc-macro panics — when the derive
  can't apply.

## Limitations

Honest and up front:

- **Named-field structs only.** Tuple structs, unit structs, enums, and unions
  are rejected with a clear compile error.
- **Field types must be `'static`.** Values are erased to `dyn Any`, and `Any`
  requires `'static`. Generic and lifetime-parameterized structs compile, but
  the `Iterable` impl only applies where every field type is `'static`:
  `Config<String>` and `Borrowed<'static>` iterate, `Borrowed<'a>` does not.
- **Read-only access.** There is no `iter_mut` yet; it is on the roadmap.
- Each `iter()` call allocates one `Vec` behind the scenes.

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
| **struct_iterable** | `(name, &dyn Any)` iteration, one derive | You want the 30-second version: derive, loop, downcast |
| [bevy_reflect](https://crates.io/crates/bevy_reflect) | Full reflection framework: `field_mut`, `DynamicStruct`, enums, tuple structs, type registry | You need real reflection (mutation, by-name access, runtime construction) or you're in Bevy |
| [facet](https://crates.io/crates/facet) | Const shape data per type plus a reflection ecosystem (serialization, diffing, CLI) | You're building serializers or tooling on top of reflection |
| [metastruct](https://crates.io/crates/metastruct) | Per-struct macros over fields, zero dynamic dispatch, mutation support | You want compile-time codegen and no `dyn Any` |
| [field_access](https://crates.io/crates/field_access) | `field("name")` accessors with typed getters, mutation, no_std | You want by-name access to individual fields |
| [valuable](https://crates.io/crates/valuable) | Visitor-based structured values (the tracing ecosystem) | Structured logging |
| [struct-field-names-as-array](https://crates.io/crates/struct-field-names-as-array) | `const` array of field names only | You only need the names |

An honest pointer: if you need **mutation** or **non-`'static` fields** today,
bevy_reflect or metastruct will serve you better until our roadmap lands.

## MSRV

The minimum supported Rust version is **1.71**, verified by a dedicated CI job.
Raising the MSRV is a minor-version bump, done only when a dependency forces it
or a meaningfully useful language feature justifies it.

## Semver policy

Pre-1.0, breaking changes happen only at 0.x version bumps, and each one is
called out in the [CHANGELOG](CHANGELOG.md) with a migration table.

The three crates in this repository (`struct_iterable`,
`struct_iterable_derive`, `struct_iterable_internal`) are versioned and
released in lockstep. Depend only on `struct_iterable` — the `_derive` and
`_internal` crates are implementation details.

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
