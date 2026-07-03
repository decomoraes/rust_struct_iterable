# struct_iterable_derive

Implementation detail of
[`struct_iterable`](https://crates.io/crates/struct_iterable): the
`#[derive(Iterable)]` proc macro. The traits and iterators it implements
(`Iterable`, `IterableMut`, `IterableAs`, `FieldIter`) live in the
`struct_iterable` facade.

**Do not depend on this crate directly** — its API is only guaranteed through
the facade. See [`struct_iterable` on
crates.io](https://crates.io/crates/struct_iterable) for documentation and
examples.
