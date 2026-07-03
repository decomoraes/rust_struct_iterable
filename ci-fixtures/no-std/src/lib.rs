//! Compile-only proof that `#[derive(Iterable)]` works in a `#![no_std]`
//! crate on a target with no std available at all (issue #7's scenario:
//! thumbv7em-none-eabihf).

#![no_std]

use struct_iterable::{Iterable, IterableMut};

#[derive(Iterable)]
pub struct Telemetry {
    pub temperature: i16,
    pub voltage: u32,
}

/// Iterate (allocation-free) and count fields via the trait.
pub fn field_names(t: &Telemetry) -> usize {
    let mut n = 0;
    for (_name, _value) in t.iter() {
        n += 1;
    }
    n + t.field_count()
}

/// Mutate through the visitor, no alloc anywhere.
pub fn bump_voltage(t: &mut Telemetry) {
    t.visit_fields_mut(&mut |name, value| {
        if name == "voltage" {
            if let Some(v) = value.downcast_mut::<u32>() {
                *v += 1;
            }
        }
    });
}
