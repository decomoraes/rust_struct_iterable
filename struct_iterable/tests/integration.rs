//! Core integration tests.
//!
//! These lock in the observable behavior of `#[derive(Iterable)]`: field
//! order, name strings, `dyn Any` identity of the yielded values, and the
//! iterator capabilities user code relies on. Any change that makes one of
//! these fail is a breaking change for downstream users.
//!
//! Note (0.2.0): raw identifiers now yield their bare name — `r#type` is
//! `"type"`, not `"r#type"` — matching serde. Recorded in the CHANGELOG
//! migration table.

use std::any::Any;
use struct_iterable::Iterable;

#[derive(Iterable)]
struct Basic {
    id: u32,
    name: String,
    maybe: Option<String>,
    r#type: bool,
}

fn sample() -> Basic {
    Basic {
        id: 7,
        name: "Ada".to_string(),
        maybe: None,
        r#type: true,
    }
}

#[test]
fn yields_fields_in_declaration_order_with_names() {
    let names: Vec<&'static str> = sample().iter().map(|(name, _)| name).collect();
    assert_eq!(names, ["id", "name", "maybe", "type"]);
}

#[test]
fn values_downcast_to_their_concrete_types() {
    let s = sample();
    let fields: Vec<(&'static str, &dyn Any)> = s.iter().collect();
    assert_eq!(fields[0].1.downcast_ref::<u32>(), Some(&7));
    assert_eq!(
        fields[1].1.downcast_ref::<String>().map(String::as_str),
        Some("Ada")
    );
    assert_eq!(fields[2].1.downcast_ref::<Option<String>>(), Some(&None));
    assert_eq!(fields[3].1.downcast_ref::<bool>(), Some(&true));
    assert!(fields[0].1.downcast_ref::<i32>().is_none());
    assert!(fields[1].1.downcast_ref::<&str>().is_none());
}

#[test]
fn iterator_supports_len_rev_and_adapters() {
    let s = sample();
    assert_eq!(s.iter().len(), 4);
    assert_eq!(s.iter().count(), 4);

    let reversed: Vec<&'static str> = s.iter().rev().map(|(name, _)| name).collect();
    assert_eq!(reversed, ["type", "maybe", "name", "id"]);

    let filtered: Vec<&'static str> = s
        .iter()
        .filter(|(name, _)| name.contains('a'))
        .map(|(name, _)| name)
        .collect();
    assert_eq!(filtered, ["name", "maybe"]);
}

#[test]
fn works_through_a_boxed_trait_object() {
    let boxed: Box<dyn Iterable> = Box::new(sample());
    let names: Vec<&'static str> = boxed.iter().map(|(name, _)| name).collect();
    assert_eq!(names, ["id", "name", "maybe", "type"]);

    // `for` over a trait object reference also works (IntoIterator).
    let mut count = 0;
    for (_name, _value) in &*boxed {
        count += 1;
    }
    assert_eq!(count, 4);
}

#[test]
fn by_name_lookup_uses_the_yielded_name_for_raw_identifiers() {
    let s = sample();
    // `r#type` yields "type" (0.2 behavior), and `field` matches that name —
    // not the raw spelling.
    assert!(s.field("type").is_some());
    assert!(s.field("r#type").is_none());
}

#[test]
fn single_field_struct_works() {
    #[derive(Iterable)]
    struct One {
        only: i64,
    }
    let one = One { only: -1 };
    let fields: Vec<(&'static str, &dyn Any)> = one.iter().collect();
    assert_eq!(fields.len(), 1);
    assert_eq!(fields[0].0, "only");
    assert_eq!(fields[0].1.downcast_ref::<i64>(), Some(&-1));
}
