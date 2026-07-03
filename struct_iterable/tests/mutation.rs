//! IterableMut (issue #2): mutable field access by name, by index, and via
//! the visitor — all safe code, no `Iterator`-shaped `iter_mut`.

use std::any::Any;
use struct_iterable::{Iterable, IterableMut};

#[derive(Iterable)]
struct Settings {
    retries: u32,
    label: String,
    enabled: bool,
}

fn sample() -> Settings {
    Settings {
        retries: 0,
        label: "old".to_string(),
        enabled: false,
    }
}

#[test]
fn field_mut_by_name_roundtrip() {
    let mut s = sample();
    *s.field_mut("retries")
        .unwrap()
        .downcast_mut::<u32>()
        .unwrap() = 7;
    s.field_mut("label")
        .unwrap()
        .downcast_mut::<String>()
        .unwrap()
        .push_str("-new");

    assert_eq!(s.retries, 7);
    assert_eq!(s.label, "old-new");
    assert!(s.field_mut("missing").is_none());
}

#[test]
fn field_at_mut_indexed_loop() {
    let mut s = sample();
    for i in 0..s.field_count() {
        let (name, value) = s.field_at_mut(i).unwrap();
        if let Some(flag) = value.downcast_mut::<bool>() {
            assert_eq!(name, "enabled");
            *flag = true;
        }
    }
    assert!(s.enabled);
    assert!(s.field_at_mut(3).is_none());
}

#[test]
fn visit_fields_mut_sees_every_field_in_order() {
    let mut s = sample();
    let mut seen = Vec::new();
    s.visit_fields_mut(&mut |name, value| {
        seen.push(name);
        if let Some(n) = value.downcast_mut::<u32>() {
            *n += 1;
        }
    });
    assert_eq!(seen, ["retries", "label", "enabled"]);
    assert_eq!(s.retries, 1);
}

#[test]
fn works_through_a_mutable_trait_object() {
    let mut boxed: Box<dyn IterableMut> = Box::new(sample());
    boxed.visit_fields_mut(&mut |_name, value| {
        if let Some(n) = value.downcast_mut::<u32>() {
            *n = 42;
        }
    });
    let retries: &dyn Any = boxed.field("retries").unwrap();
    assert_eq!(retries.downcast_ref::<u32>(), Some(&42));
}
