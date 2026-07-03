#![doc = include_str!("../README.md")]

/// Derive macro that implements the [`Iterable`](trait@Iterable) trait for a
/// struct with named fields.
///
/// The generated `iter` method yields the fields in declaration order as
/// `(&'static str, &dyn Any)` pairs.
///
/// # What can be derived
///
/// Any struct with named fields, including generic, lifetime-parameterized,
/// and const-generic ones. Because values are erased to `dyn Any` (which
/// requires `'static`), the generated impl carries an `Any` bound for each
/// field type: the impl applies whenever every field type is `'static`.
///
/// # Compile errors
///
/// The derive rejects enums, unions, tuple structs, and unit structs, as well
/// as field-level `#[iterable(...)]` attributes (not supported yet) — each
/// with an error spanned to the offending item.
///
/// # Container attribute
///
/// `#[iterable(crate = "path")]` sets the path the generated code uses to
/// reach the `Iterable` trait (default: `::struct_iterable`). Use it when
/// this crate is renamed in `Cargo.toml` or re-exported from another crate.
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

/// Trait implemented by [`#[derive(Iterable)]`](derive@Iterable): iterate a
/// struct's fields as `(name, value)` pairs.
///
/// The contract of `iter`:
///
/// - **Item type:** `(&'static str, &dyn Any)` — the field's name and a
///   type-erased reference to its value; call `downcast_ref::<T>()` on the
///   value to get the concrete type back.
/// - **Order:** fields are yielded in declaration order.
/// - **Iterator type:** a plain [`std::vec::IntoIter`], so it is exact-size,
///   double-ended, and works with every iterator adapter.
/// - The trait must be in scope to call `.iter()`:
///   `use struct_iterable::Iterable;`.
///
/// You don't implement this trait by hand — derive it.
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
/// ```
pub use struct_iterable_internal::Iterable;
