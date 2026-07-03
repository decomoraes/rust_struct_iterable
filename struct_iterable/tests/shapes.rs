//! Struct shapes and field attributes new in 0.2.0: tuple structs, unit
//! structs, empty structs, `#[iterable(skip)]`, `#[iterable(rename)]`.

use std::any::Any;
use struct_iterable::{Iterable, IterableMut};

#[derive(Iterable)]
struct Wrapper(u32, String);

#[derive(Iterable)]
struct Nothing;

#[derive(Iterable)]
struct Empty {}

#[derive(Iterable)]
struct WithSkip {
    kept: u32,
    #[iterable(skip)]
    hidden: String,
    also_kept: bool,
}

#[derive(Iterable)]
struct Renamed {
    #[iterable(rename = "flake-enabled")]
    flake_enabled: bool,
    plain: u8,
}

#[derive(Iterable)]
struct RenamedTuple(#[iterable(rename = "inner")] u32, u8);

// The skip attribute is the documented escape hatch for borrowed fields on
// the dyn Any path: with the borrow skipped, no `&'a str: Any` bound is
// emitted and the struct is Iterable for EVERY lifetime.
#[derive(Iterable)]
struct PartlyBorrowed<'a> {
    #[iterable(skip)]
    name: &'a str,
    retries: u32,
}

#[test]
fn tuple_struct_fields_are_named_by_position() {
    let w = Wrapper(9, "nine".to_string());
    let fields: Vec<(&'static str, &dyn Any)> = w.iter().collect();
    assert_eq!(fields.len(), 2);
    assert_eq!(fields[0].0, "0");
    assert_eq!(fields[0].1.downcast_ref::<u32>(), Some(&9));
    assert_eq!(fields[1].0, "1");
    assert_eq!(
        fields[1].1.downcast_ref::<String>().map(String::as_str),
        Some("nine")
    );
}

#[test]
fn unit_and_empty_structs_iterate_empty() {
    assert_eq!(Nothing.field_count(), 0);
    assert_eq!(Nothing.iter().count(), 0);
    assert!(Nothing.field_at(0).is_none());

    let e = Empty {};
    assert_eq!(e.field_count(), 0);
    assert_eq!(e.iter().count(), 0);
}

#[test]
fn skipped_fields_are_invisible() {
    let s = WithSkip {
        kept: 1,
        hidden: "secret".to_string(),
        also_kept: true,
    };
    let names: Vec<&'static str> = s.iter().map(|(name, _)| name).collect();
    assert_eq!(names, ["kept", "also_kept"]);
    assert_eq!(s.field_count(), 2);
    assert!(s.field("hidden").is_none());
    // Skipping only affects iteration; direct access is untouched.
    assert_eq!(s.hidden, "secret");
}

#[test]
fn renamed_fields_yield_the_new_name() {
    let r = Renamed {
        flake_enabled: true,
        plain: 3,
    };
    let names: Vec<&'static str> = r.iter().map(|(name, _)| name).collect();
    assert_eq!(names, ["flake-enabled", "plain"]);
    assert!(r.field("flake-enabled").is_some());
    assert!(r.field("flake_enabled").is_none());

    let t = RenamedTuple(5, 6);
    let names: Vec<&'static str> = t.iter().map(|(name, _)| name).collect();
    assert_eq!(names, ["inner", "1"]);
}

#[test]
fn mutable_by_name_lookup_uses_the_renamed_name() {
    let mut r = Renamed {
        flake_enabled: false,
        plain: 0,
    };
    *r.field_mut("flake-enabled")
        .unwrap()
        .downcast_mut::<bool>()
        .unwrap() = true;
    assert!(r.flake_enabled);
    assert!(r.field_mut("flake_enabled").is_none());
}

#[test]
fn skip_unlocks_borrowed_structs_on_the_any_path() {
    let owned = String::from("borrowed");
    let p = PartlyBorrowed {
        name: &owned,
        retries: 2,
    };
    let fields: Vec<(&'static str, &dyn Any)> = p.iter().collect();
    assert_eq!(fields.len(), 1);
    assert_eq!(fields[0].0, "retries");
    // Skipping only affects iteration; direct access is untouched.
    assert_eq!(p.name, "borrowed");
}
