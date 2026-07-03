# Contributing

Thanks for helping out. This crate is deliberately small and stable: bug
fixes, docs, and tests are always welcome; for new features, please open an
issue to discuss the design first.

This project is maintained by one person as a side project. Issues and PRs
are triaged in batches, typically within a few weeks — slow responses are not
abandonment. PRs that include tests get reviewed first.

## Workspace map

This repository is a Cargo workspace with three crates, versioned and
released in lockstep:

| Crate | Role |
|---|---|
| `struct_iterable/` | The facade — the only crate users depend on. Re-exports the derive macro and the trait. **All tests and examples live here** (`struct_iterable/tests/`, `struct_iterable/examples/`). |
| `struct_iterable_derive/` | The `#[derive(Iterable)]` proc macro. Implementation detail. |
| `struct_iterable_internal/` | The `Iterable` trait definition. Implementation detail. |

Tests live in the facade because a proc-macro crate cannot invoke its own
macro in unit tests, and because fixtures should exercise exactly what a user
writes (`use struct_iterable::Iterable;`) — including re-export hygiene.

## Commands

```sh
# Everything: unit, integration, UI, expansion, and doctests
# (the README's code blocks are doctests too, via include_str!):
cargo test --workspace

# Refresh trybuild stderr snapshots after changing a diagnostic:
TRYBUILD=overwrite cargo test -p struct_iterable --test ui

# Refresh macrotest expansion snapshots after changing codegen
# (requires: cargo install cargo-expand):
MACROTEST=overwrite cargo test -p struct_iterable --test expand

# Run the examples:
cargo run -p struct_iterable --example basic
cargo run -p struct_iterable --example config_validation
```

## The one hard rule

**Every change to the derive crate lands with a UI test or an expansion
snapshot.** If you change a diagnostic, the refreshed `.stderr` file in
`struct_iterable/tests/ui/` is the review artifact. If you change the
generated code, the refreshed `.expanded.rs` file in
`struct_iterable/tests/expand/` is. A derive change without either is not
reviewable.

## Commit messages

Use [Conventional Commits](https://www.conventionalcommits.org/) — `feat:`,
`fix:`, `docs:`, `test:`, and `feat!:`/`fix!:` for breaking changes. Release
automation infers version bumps and changelog entries from them.

## MSRV

The minimum supported Rust version is **1.71**, enforced by a dedicated CI
job that runs `cargo check --workspace --locked` on a pinned 1.71 toolchain.
Raising the MSRV is a minor-version change, done only when a dependency
forces it or a meaningfully useful language feature justifies it.
Dev-dependencies do not need to honor the MSRV — only the published crates do.

## Releases

Releases are currently cut manually by the maintainer, publishing the three
crates in dependency order (internal → derive → facade). Automated releases
via release-plz arrive with 0.2. Until then, don't touch version numbers or
the CHANGELOG's version headings in PRs — the maintainer handles those at
release time.
