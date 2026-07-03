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
| `struct_iterable/` | The facade — the only crate users depend on. Defines the `Iterable`/`IterableMut`/`IterableAs` traits and the `FieldIter`/`FieldIterAs` iterators, and re-exports the derive macro. **All tests and examples live here** (`struct_iterable/tests/`, `struct_iterable/examples/`). |
| `struct_iterable_derive/` | The `#[derive(Iterable)]` proc macro. Implementation detail. |
| `struct_iterable_internal/` | Deprecated compatibility shim re-exporting the facade, kept for 0.1-era dependents. No new API ever lands here. |

Tests live in the facade because a proc-macro crate cannot invoke its own
macro in unit tests, and because fixtures should exercise exactly what a user
writes (`use struct_iterable::Iterable;`) — including re-export hygiene.

## Commands

```sh
# Unit, integration, and doctests (the README's code blocks are doctests
# too, via include_str!):
cargo test --workspace

# Snapshot tests (trybuild UI + macrotest expansion) are gated behind
# SNAPSHOT_TESTS because their output drifts across toolchains — CI runs
# them on a pinned toolchain. Requires: cargo install cargo-expand
SNAPSHOT_TESTS=1 cargo test -p struct_iterable --test ui --test expand

# Refresh trybuild stderr snapshots after changing a diagnostic:
SNAPSHOT_TESTS=1 TRYBUILD=overwrite cargo test -p struct_iterable --test ui

# Refresh macrotest expansion snapshots after changing codegen:
SNAPSHOT_TESTS=1 MACROTEST=overwrite cargo test -p struct_iterable --test expand

# Run the examples:
cargo run -p struct_iterable --example basic
cargo run -p struct_iterable --example config_validation
cargo run -p struct_iterable --example custom_trait
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

Releases are automated with [release-plz](https://release-plz.dev)
(`release-plz.toml` + `.github/workflows/release-plz.yml`), with the
maintainer's merge as the only trigger — nothing is ever published without
that explicit approval:

1. Every push to `main` opens or updates a **release PR** containing the
   computed version bumps (inferred from Conventional Commits), updated
   `CHANGELOG.md` entries, and a `cargo-semver-checks` report. The three
   crates share one `version_group`, so they always bump together; and
   because they reference each other by `version + path`, a change in the
   derive crate re-releases the facade and the internal shim too — which is
   the desired lockstep behavior.
2. Merging the release PR runs the release job: it publishes the crates to
   crates.io in dependency order, pushes a single lockstep tag `v{version}`
   (one tag per release, never per-crate tags), and creates the GitHub
   release.

The release job is double-gated: it only runs when the repository variable
`RELEASE_PLZ_ENABLED` is `"true"`, and it needs the `CARGO_REGISTRY_TOKEN`
repository secret (a crates.io token scoped to publish-update). Until both
are configured, pushes to `main` only open/update the release PR and
releases use the manual process below.

### Manual fallback

Publish in dependency order. The facade depends on the derive crate, and the
`struct_iterable_internal` shim depends on the facade — so the shim goes
**last**:

```sh
cargo publish -p struct_iterable_derive --locked
cargo publish -p struct_iterable --locked
cargo publish -p struct_iterable_internal --locked
git tag v0.2.0 && git push origin v0.2.0   # single lockstep tag: v{version}
```

Then stamp the release date in `CHANGELOG.md` and create the GitHub release
from the tag.

In PRs, don't touch version numbers or the CHANGELOG's version headings —
release automation (or the maintainer, in the fallback) handles those at
release time.
