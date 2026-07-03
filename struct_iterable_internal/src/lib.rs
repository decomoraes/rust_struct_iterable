//! **Deprecated** implementation detail of
//! [`struct_iterable`](https://docs.rs/struct_iterable).
//!
//! Since 0.2.0 the `Iterable` trait lives in the `struct_iterable` facade
//! itself; this crate is a compatibility shim that re-exports everything from
//! there. Depend on `struct_iterable` directly — this crate will receive no
//! further releases.

#![no_std]

pub use struct_iterable::*;
