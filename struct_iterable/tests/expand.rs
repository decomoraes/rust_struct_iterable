//! macrotest expansion snapshots: any change to the code the derive emits
//! shows up as a reviewable diff in tests/expand/*.expanded.rs.
//!
//! Gated behind SNAPSHOT_TESTS=1 because it shells out to cargo-expand
//! (`cargo install cargo-expand`) and its output drifts across toolchains:
//! CI runs this only in the pinned-toolchain `ui` job. Refresh with
//! `SNAPSHOT_TESTS=1 MACROTEST=overwrite cargo test -p struct_iterable --test expand`.

#[test]
fn expand() {
    if std::env::var_os("SNAPSHOT_TESTS").is_none() {
        eprintln!("skipping macrotest snapshots; set SNAPSHOT_TESTS=1 to run them");
        return;
    }
    macrotest::expand("tests/expand/*.rs");
}
