//! trybuild UI tests: lock in the exact compile errors users see when the
//! derive is applied to something unsupported. Refresh snapshots with
//! `TRYBUILD=overwrite cargo test -p struct_iterable --test ui` and review
//! the .stderr diff like any other code change.

#[test]
fn ui() {
    let t = trybuild::TestCases::new();
    t.compile_fail("tests/ui/*.rs");
}
