//! trybuild UI tests: lock in the exact compile errors users see when the
//! derive is applied to something unsupported.
//!
//! Gated behind SNAPSHOT_TESTS=1 because rustc's stderr output drifts across
//! versions: CI runs this only in the pinned-toolchain `ui` job, not in the
//! stable/beta matrix. Refresh snapshots with
//! `SNAPSHOT_TESTS=1 TRYBUILD=overwrite cargo test -p struct_iterable --test ui`
//! and review the .stderr diff like any other code change.

#[test]
fn ui() {
    if std::env::var_os("SNAPSHOT_TESTS").is_none() {
        eprintln!("skipping trybuild snapshots; set SNAPSHOT_TESTS=1 to run them");
        return;
    }
    let t = trybuild::TestCases::new();
    t.compile_fail("tests/ui/*.rs");
}
