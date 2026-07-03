//! Uses the derive and trait crates directly, without the facade, via the
//! `#[iterable(crate = "...")]` container attribute.

use struct_iterable_internal::Iterable;

#[derive(struct_iterable_derive::Iterable)]
#[iterable(crate = "struct_iterable_internal")]
struct Point {
    x: i32,
    y: i32,
}

fn main() {
    let point = Point { x: 1, y: 2 };

    for (field_name, field_value) in point.iter() {
        println!("{field_name} = {:?}", field_value.downcast_ref::<i32>());
    }
}
