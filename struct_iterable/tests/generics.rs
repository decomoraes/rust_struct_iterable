//! Generic and lifetime-parameterized structs (issue #1): the derive now
//! emits `impl<...> Iterable for Struct<...>` with one `FieldTy: Any`
//! where-predicate per field, so these compile whenever the concrete field
//! types are `'static`.

use std::any::Any;
use struct_iterable::Iterable;

#[derive(Iterable)]
struct Wrapper<T> {
    inner: T,
    label: String,
}

#[derive(Iterable)]
struct Pair<T, U: Clone>
where
    U: Default,
{
    left: T,
    right: U,
}

#[derive(Iterable)]
struct Borrowed<'a> {
    s: &'a str,
}

#[derive(Iterable)]
struct WithConst<const N: usize> {
    data: [u8; N],
}

#[test]
fn generic_struct_with_static_type_arguments() {
    let w = Wrapper {
        inner: 42u32,
        label: "answer".to_string(),
    };
    let fields: Vec<(&'static str, &dyn Any)> = w.iter().collect();
    assert_eq!(fields[0].0, "inner");
    assert_eq!(fields[0].1.downcast_ref::<u32>(), Some(&42));
    assert_eq!(fields[1].0, "label");
}

#[test]
fn multiple_type_parameters_with_existing_bounds_and_where_clause() {
    let p = Pair {
        left: -1i64,
        right: 8u8,
    };
    let names: Vec<&'static str> = p.iter().map(|(name, _)| name).collect();
    assert_eq!(names, ["left", "right"]);
}

#[test]
fn lifetime_parameterized_struct_at_static() {
    // This is the exact reproduction from issue #1: a string literal is
    // `&'static str`, so `Borrowed<'static>` is `Iterable`.
    let s = "hello";
    let b = Borrowed { s };
    let fields: Vec<(&'static str, &dyn Any)> = b.iter().collect();
    assert_eq!(fields[0].0, "s");
    assert_eq!(fields[0].1.downcast_ref::<&str>(), Some(&"hello"));
}

#[test]
fn const_generic_struct() {
    let c = WithConst { data: [1, 2, 3] };
    let fields: Vec<(&'static str, &dyn Any)> = c.iter().collect();
    assert_eq!(fields[0].1.downcast_ref::<[u8; 3]>(), Some(&[1, 2, 3]));
}
