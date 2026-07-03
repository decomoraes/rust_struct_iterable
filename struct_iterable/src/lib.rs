#![no_std]
#![forbid(unsafe_code)]
#![doc = include_str!("../README.md")]

use core::any::Any;
use core::fmt;
use core::iter::FusedIterator;

/// Derive macro that implements [`Iterable`](trait@Iterable) and
/// [`IterableMut`] for a struct, plus [`IterableAs`] for every trait named in
/// a `#[iterable(to(dyn Trait))]` attribute.
///
/// # What can be derived
///
/// Structs with named fields, tuple structs (fields named `"0"`, `"1"`, …),
/// and unit structs (an empty iterator) — including generic,
/// lifetime-parameterized, and const-generic ones. Enums and unions are
/// rejected with a spanned compile error.
///
/// Because values are erased to `dyn Any` (which requires `'static`), the
/// generated `Iterable`/`IterableMut` impls carry an `Any` bound for each
/// field type: the impls apply whenever every non-skipped field type is
/// `'static`. Structs borrowing non-`'static` data can still iterate through
/// [`IterableAs`] with a custom trait, or `#[iterable(skip)]` the borrowed
/// fields.
///
/// # Container attributes
///
/// - `#[iterable(to(dyn Trait))]` — additionally implement
///   [`IterableAs<dyn Trait>`](IterableAs), yielding fields as `&dyn Trait`.
///   Repeatable; every non-skipped field must implement the trait (compile
///   error otherwise, spanned to the offending field). Extra bounds are
///   allowed: `to(dyn Trait + Send)`.
/// - `#[iterable(no_any)]` — suppress the `dyn Any`-based impls (`Iterable`
///   and `IterableMut`), keeping only the `to(...)` impls.
/// - `#[iterable(crate = "path")]` — path the generated code uses to reach
///   this crate (default: `::struct_iterable`), for renamed or re-exported
///   facades.
///
/// # Field attributes
///
/// - `#[iterable(skip)]` — leave the field out of iteration entirely (also
///   the escape hatch for non-`'static` fields on the `dyn Any` path).
/// - `#[iterable(rename = "other-name")]` — yield a different name string
///   (e.g. kebab-case keys for query strings).
///
/// # Example
///
/// ```
/// use struct_iterable::Iterable;
///
/// #[derive(Iterable)]
/// struct Pair<T> {
///     left: T,
///     right: T,
/// }
///
/// let pair = Pair { left: 1_u32, right: 2_u32 };
/// let names: Vec<&'static str> = pair.iter().map(|(name, _)| name).collect();
/// assert_eq!(names, ["left", "right"]);
/// ```
pub use struct_iterable_derive::Iterable;

/// A struct whose fields can be enumerated as `(name, &dyn Any)` pairs.
///
/// Implemented via [`#[derive(Iterable)]`](derive@Iterable).
///
/// The contract of the iteration:
///
/// - **Item type:** `(&'static str, &dyn Any)` — the field's name and a
///   type-erased reference to its value; call `downcast_ref::<T>()` on the
///   value to get the concrete type back.
/// - **Order:** fields are yielded in declaration order, skipped fields
///   excluded.
/// - **Iterator type:** [`FieldIter`] — allocation-free, exact-size,
///   double-ended, fused, works with every iterator adapter.
/// - The trait must be in scope to call `.iter()`:
///   `use struct_iterable::Iterable;`.
/// - The trait is object-safe: `Box<dyn Iterable>` supports every method on
///   this trait, including `.iter()`. (Trait objects don't get the blanket
///   [`IterableAs<dyn Any>`](IterableAs) bridge, which needs a sized type —
///   call `.iter()` on them instead.)
///
/// # Manual implementations
///
/// Derive when you can. When you can't, implement `field_count`, `field_at`,
/// and `iter` (whose body is always `FieldIter::new(self)`). `field_at(i)`
/// must return `Some` for every `i < field_count()` and `None` otherwise,
/// with a stable order. Violating this makes the iterators misbehave —
/// items may be skipped or yielded after a `None` (even through `.fuse()`,
/// since [`FieldIter`] advertises [`FusedIterator`]), and a lying
/// `field_count` makes `size_hint` lie, which can make `collect()` panic —
/// but it can never cause memory unsafety (this crate has no unsafe code and
/// nothing relies on the contract for safety).
///
/// # Example
///
/// ```
/// use struct_iterable::Iterable;
///
/// #[derive(Iterable)]
/// struct MyStruct {
///     id: u32,
///     name: String,
/// }
///
/// let my_instance = MyStruct { id: 7, name: "Ada".to_string() };
///
/// for (field_name, field_value) in my_instance.iter() {
///     if let Some(id) = field_value.downcast_ref::<u32>() {
///         println!("{field_name}: {id}");
///     } else if let Some(name) = field_value.downcast_ref::<String>() {
///         println!("{field_name}: {name}");
///     }
/// }
///
/// assert_eq!(my_instance.field_count(), 2);
/// assert_eq!(
///     my_instance.field("id").and_then(|v| v.downcast_ref::<u32>()),
///     Some(&7)
/// );
/// ```
pub trait Iterable {
    /// Number of iterable (non-skipped) fields.
    fn field_count(&self) -> usize;

    /// Name and value of the field at `index`, or `None` past the end.
    fn field_at(&self, index: usize) -> Option<(&'static str, &dyn Any)>;

    /// Iterate over `(field_name, &dyn Any)` pairs. Allocation-free.
    ///
    /// Implemented by the derive; a manual implementation is always
    /// `FieldIter::new(self)`. (It is a required method rather than a
    /// provided one so that it stays callable on `dyn Iterable`.)
    fn iter(&self) -> FieldIter<'_>;

    /// Look up a field's value by name. O(n) in the number of fields.
    fn field(&self, name: &str) -> Option<&dyn Any> {
        (0..self.field_count()).find_map(|i| {
            let (field_name, value) = self.field_at(i)?;
            (field_name == name).then_some(value)
        })
    }
}

/// Mutable access to the fields of an [`Iterable`] struct.
///
/// Emitted by the same [`#[derive(Iterable)]`](derive@Iterable) — no second
/// derive needed. There is deliberately no `Iterator`-shaped `iter_mut`: an
/// iterator handing out multiple live `&mut dyn Any` cannot be built from a
/// safe, manually-implementable trait without `unsafe` aliasing contracts.
/// The three shapes below cover mutation without any unsafe code.
///
/// # Example
///
/// ```
/// use struct_iterable::{Iterable, IterableMut};
///
/// #[derive(Iterable)]
/// struct Config {
///     retries: u32,
///     verbose: bool,
/// }
///
/// let mut config = Config { retries: 0, verbose: false };
///
/// // 1. By name:
/// *config.field_mut("retries").unwrap().downcast_mut::<u32>().unwrap() = 5;
///
/// // 2. Visitor over every field:
/// config.visit_fields_mut(&mut |_name, value| {
///     if let Some(flag) = value.downcast_mut::<bool>() {
///         *flag = true;
///     }
/// });
///
/// // 3. Indexed loop (each borrow ends before the next begins):
/// for i in 0..config.field_count() {
///     let (_name, _value) = config.field_at_mut(i).unwrap();
/// }
///
/// assert_eq!(config.retries, 5);
/// assert!(config.verbose);
/// ```
pub trait IterableMut: Iterable {
    /// Name and mutable value of the field at `index`, or `None` past the end.
    fn field_at_mut(&mut self, index: usize) -> Option<(&'static str, &mut dyn Any)>;

    /// Look up a field's value by name, mutably. O(n) in the number of fields.
    fn field_mut(&mut self, name: &str) -> Option<&mut dyn Any> {
        let index = (0..self.field_count())
            .find(|&i| matches!(self.field_at(i), Some((n, _)) if n == name))?;
        self.field_at_mut(index).map(|(_, value)| value)
    }

    /// Visit every field mutably, in declaration order.
    fn visit_fields_mut(&mut self, f: &mut dyn FnMut(&'static str, &mut dyn Any)) {
        for i in 0..self.field_count() {
            if let Some((name, value)) = self.field_at_mut(i) {
                f(name, value);
            }
        }
    }
}

/// Iterate a struct's fields as `&Dyn` for a caller-chosen trait object type,
/// instead of `&dyn Any`.
///
/// Request impls with [`#[iterable(to(dyn Trait))]`](derive@Iterable); every
/// non-skipped field must implement the trait, enforced at compile time. A
/// struct may implement `IterableAs` for several trait objects at once —
/// disambiguate with [`IterableAsExt::iter_fields_as`].
///
/// Unlike the `dyn Any` path, this works for structs borrowing
/// non-`'static` data: `&dyn Trait` does not require `'static`.
///
/// Every sized [`Iterable`] type is automatically `IterableAs<dyn Any>`
/// (trait objects like `dyn Iterable` are excluded — the bridge needs the
/// unsize coercion, so call `.iter()` on them instead).
///
/// # Example
///
/// ```
/// use struct_iterable::Iterable;
///
/// #[derive(Iterable)]
/// #[iterable(to(dyn std::fmt::Display), no_any)]
/// struct Report<'a> {
///     title: &'a str,
///     count: usize,
/// }
///
/// let owned = String::from("daily");
/// let report = Report { title: &owned, count: 3 };
///
/// // `Report<'a>` borrows non-'static data, yet iterates as &dyn Display:
/// use struct_iterable::IterableAs;
/// for (name, value) in report.iter_as() {
///     println!("{name} = {value}");
/// }
/// ```
pub trait IterableAs<Dyn: ?Sized> {
    /// Number of iterable (non-skipped) fields.
    fn field_count_as(&self) -> usize;

    /// Name and value of the field at `index` as `&Dyn`, or `None` past the
    /// end.
    fn field_at_as(&self, index: usize) -> Option<(&'static str, &Dyn)>;

    /// Iterate over `(field_name, &Dyn)` pairs. Allocation-free.
    ///
    /// Implemented by the derive; a manual implementation is always
    /// `FieldIterAs::new(self)`.
    fn iter_as(&self) -> FieldIterAs<'_, Dyn>;
}

/// Every sized [`Iterable`] type iterates as `dyn Any` through the general
/// mechanism too. (`dyn Iterable` itself can't take part: the impl body
/// needs the `&T -> &dyn IterableAs<dyn Any>` unsize coercion, which only
/// exists for sized types.)
impl<T: Iterable> IterableAs<dyn Any> for T {
    #[inline]
    fn field_count_as(&self) -> usize {
        self.field_count()
    }

    #[inline]
    fn field_at_as(&self, index: usize) -> Option<(&'static str, &dyn Any)> {
        self.field_at(index)
    }

    #[inline]
    fn iter_as(&self) -> FieldIterAs<'_, dyn Any> {
        FieldIterAs::new(self)
    }
}

/// Turbofish ergonomics when a struct implements [`IterableAs`] for several
/// trait objects: `value.iter_fields_as::<dyn Trait>()`.
///
/// # Example
///
/// ```
/// use struct_iterable::{Iterable, IterableAsExt};
///
/// #[derive(Iterable)]
/// #[iterable(to(dyn std::fmt::Display))]
/// struct Stats {
///     jobs: u32,
///     load: f64,
/// }
///
/// let stats = Stats { jobs: 2, load: 0.5 };
///
/// // `Stats` iterates both as `dyn Display` (requested) and as `dyn Any`
/// // (automatic); the turbofish picks which:
/// let rendered: Vec<String> = stats
///     .iter_fields_as::<dyn std::fmt::Display>()
///     .map(|(name, value)| format!("{name}={value}"))
///     .collect();
/// assert_eq!(rendered, ["jobs=2", "load=0.5"]);
/// ```
pub trait IterableAsExt {
    /// Like [`IterableAs::iter_as`], with the trait object chosen at the call
    /// site.
    #[inline]
    fn iter_fields_as<Dyn: ?Sized>(&self) -> FieldIterAs<'_, Dyn>
    where
        Self: IterableAs<Dyn>,
    {
        IterableAs::<Dyn>::iter_as(self)
    }
}

impl<T: ?Sized> IterableAsExt for T {}

/// Allocation-free iterator over an [`Iterable`]'s fields.
///
/// Yields `(&'static str, &dyn Any)` in declaration order. Implements
/// [`ExactSizeIterator`], [`DoubleEndedIterator`], [`FusedIterator`], and
/// [`Clone`] — the trait surface `std::vec::IntoIter` offered, so adapter
/// chains, `len()`, and `rev()` keep working. (`vec::IntoIter`'s inherent
/// `as_slice`/`as_mut_slice` have no equivalent here; collect first if you
/// need a slice.)
pub struct FieldIter<'a> {
    obj: &'a (dyn Iterable + 'a),
    front: usize,
    back: usize,
}

impl<'a> FieldIter<'a> {
    #[inline]
    pub fn new(obj: &'a (dyn Iterable + 'a)) -> Self {
        FieldIter {
            obj,
            front: 0,
            back: obj.field_count(),
        }
    }
}

impl<'a> Iterator for FieldIter<'a> {
    type Item = (&'static str, &'a dyn Any);

    #[inline]
    fn next(&mut self) -> Option<Self::Item> {
        if self.front < self.back {
            let i = self.front;
            self.front += 1;
            self.obj.field_at(i)
        } else {
            None
        }
    }

    #[inline]
    fn size_hint(&self) -> (usize, Option<usize>) {
        let n = self.back - self.front;
        (n, Some(n))
    }
}

impl<'a> DoubleEndedIterator for FieldIter<'a> {
    #[inline]
    fn next_back(&mut self) -> Option<Self::Item> {
        if self.front < self.back {
            self.back -= 1;
            self.obj.field_at(self.back)
        } else {
            None
        }
    }
}

impl<'a> ExactSizeIterator for FieldIter<'a> {}
impl<'a> FusedIterator for FieldIter<'a> {}

impl<'a> Clone for FieldIter<'a> {
    fn clone(&self) -> Self {
        FieldIter {
            obj: self.obj,
            front: self.front,
            back: self.back,
        }
    }
}

impl<'a> fmt::Debug for FieldIter<'a> {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        f.debug_struct("FieldIter")
            .field("front", &self.front)
            .field("back", &self.back)
            .finish_non_exhaustive()
    }
}

/// Allocation-free iterator over an [`IterableAs`]'s fields, yielding
/// `(&'static str, &Dyn)`.
pub struct FieldIterAs<'a, Dyn: ?Sized> {
    obj: &'a (dyn IterableAs<Dyn> + 'a),
    front: usize,
    back: usize,
}

impl<'a, Dyn: ?Sized> FieldIterAs<'a, Dyn> {
    #[inline]
    pub fn new(obj: &'a (dyn IterableAs<Dyn> + 'a)) -> Self {
        FieldIterAs {
            obj,
            front: 0,
            back: obj.field_count_as(),
        }
    }
}

impl<'a, Dyn: ?Sized> Iterator for FieldIterAs<'a, Dyn> {
    type Item = (&'static str, &'a Dyn);

    #[inline]
    fn next(&mut self) -> Option<Self::Item> {
        if self.front < self.back {
            let i = self.front;
            self.front += 1;
            self.obj.field_at_as(i)
        } else {
            None
        }
    }

    #[inline]
    fn size_hint(&self) -> (usize, Option<usize>) {
        let n = self.back - self.front;
        (n, Some(n))
    }
}

impl<'a, Dyn: ?Sized> DoubleEndedIterator for FieldIterAs<'a, Dyn> {
    #[inline]
    fn next_back(&mut self) -> Option<Self::Item> {
        if self.front < self.back {
            self.back -= 1;
            self.obj.field_at_as(self.back)
        } else {
            None
        }
    }
}

impl<'a, Dyn: ?Sized> ExactSizeIterator for FieldIterAs<'a, Dyn> {}
impl<'a, Dyn: ?Sized> FusedIterator for FieldIterAs<'a, Dyn> {}

impl<'a, Dyn: ?Sized> Clone for FieldIterAs<'a, Dyn> {
    fn clone(&self) -> Self {
        FieldIterAs {
            obj: self.obj,
            front: self.front,
            back: self.back,
        }
    }
}

impl<'a, Dyn: ?Sized> fmt::Debug for FieldIterAs<'a, Dyn> {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        f.debug_struct("FieldIterAs")
            .field("front", &self.front)
            .field("back", &self.back)
            .finish_non_exhaustive()
    }
}

/// `for (name, value) in &*boxed` works on trait objects too.
impl<'a, 't> IntoIterator for &'a (dyn Iterable + 't) {
    type Item = (&'static str, &'a dyn Any);
    type IntoIter = FieldIter<'a>;

    #[inline]
    fn into_iter(self) -> Self::IntoIter {
        FieldIter::new(self)
    }
}
