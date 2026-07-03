//! The `#[iterable(crate = "...")]` escape hatch: points the generated impl
//! at the right crate path when `struct_iterable` is renamed in Cargo.toml
//! or re-exported from another crate (proc macros have no `$crate`).

use struct_iterable as si;

#[derive(si::Iterable)]
#[iterable(crate = "si")]
struct Renamed {
    x: u8,
}

#[test]
fn derive_works_through_a_renamed_crate_path() {
    use si::Iterable;

    let r = Renamed { x: 5 };
    let fields: Vec<(&'static str, &dyn std::any::Any)> = r.iter().collect();
    assert_eq!(fields.len(), 1);
    assert_eq!(fields[0].0, "x");
    assert_eq!(fields[0].1.downcast_ref::<u8>(), Some(&5));
}
