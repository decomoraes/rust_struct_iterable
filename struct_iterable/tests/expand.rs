//! macrotest expansion snapshots: any change to the code the derive emits
//! shows up as a reviewable diff in tests/expand/*.expanded.rs. Refresh with
//! `MACROTEST=overwrite cargo test -p struct_iterable --test expand`
//! (requires `cargo install cargo-expand`).

#[test]
fn expand() {
    macrotest::expand("tests/expand/*.rs");
}
