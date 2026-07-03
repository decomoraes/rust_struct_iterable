//! Uses the derive and trait crates directly, without the facade, via the
//! `#[iterable(crate = "...")]` container attribute. Because the facade is
//! absent here, a derive that hardcoded `::struct_iterable` anywhere in its
//! output — including the `to(...)` path — would fail this build.

use struct_iterable_internal::{Iterable, IterableAsExt};

#[derive(struct_iterable_derive::Iterable)]
#[iterable(crate = "struct_iterable_internal")]
#[iterable(to(dyn std::fmt::Debug))]
struct Point {
    x: i32,
    y: i32,
}

fn main() {
    let point = Point { x: 1, y: 2 };

    for (field_name, field_value) in point.iter() {
        println!("{field_name} = {:?}", field_value.downcast_ref::<i32>());
    }

    for (field_name, field_value) in point.iter_fields_as::<dyn std::fmt::Debug>() {
        println!("{field_name} = {field_value:?}");
    }
}
