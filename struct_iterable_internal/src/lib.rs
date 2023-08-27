/// The `Iterable` trait.
///
/// This trait is implemented for structs that derive the `Iterable` proc macro.
/// It provides the `iter` and `iter_mut` methods which return iterators over the struct's 
/// fields as tuples, containing the field name as a static string and a reference to the field's 
/// value as `dyn Any`.
///
/// You usually don't need to implement this trait manually, as it is automatically derived when 
/// using the `#[derive(Iterable)]` proc macro.
///
/// # Example
///
/// ```
/// use struct_iterable::Iterable;
///
/// #[derive(Iterable)]
/// struct MyStruct {
///     field1: i32,
///     field2: String,
/// }
///
/// let my_instance = MyStruct {
///     field1: 42,
///     field2: "Hello, world!".to_string(),
/// };
///
/// // Iterate over the fields of `my_instance`:
/// for (field_name, field_value) in my_instance.iter() {
///     println!("{}: {:?}", field_name, field_value);
/// }
/// ```
pub trait Iterable {
    /// Returns an iterator over the struct's fields as tuples.
    ///
    /// Each tuple contains a field's name as a static string and a reference to the field's 
    /// value as `dyn Any`.
    ///
    /// # Example
    ///
    /// ```
    /// use struct_iterable::Iterable;
    ///
    /// #[derive(Iterable)]
    /// struct MyStruct {
    ///     field1: i32,
    ///     field2: String,
    /// }
    ///
    /// let my_instance = MyStruct {
    ///     field1: 42,
    ///     field2: "Hello, world!".to_string(),
    /// };
    ///
    /// // Iterate over the fields of `my_instance`:
    /// for (field_name, field_value) in my_instance.iter() {
    ///     println!("{}: {:?}", field_name, field_value);
    /// }
    /// ```
    fn iter(&self) -> std::vec::IntoIter<(&'static str, &'_ dyn std::any::Any)>;

    /// Returns a mutable iterator over the struct's fields as tuples.
    ///
    /// Each tuple contains a field's name as a static string and a mutable reference to the field's 
    /// value as `dyn Any`.
    ///
    /// # Example
    ///
    /// ```
    /// use struct_iterable::Iterable;
    ///
    /// #[derive(Iterable)]
    /// struct MyStruct {
    ///     field1: i32,
    ///     field2: String,
    /// }
    ///
    /// let mut my_instance = MyStruct {
    ///     field1: 42,
    ///     field2: "Hello, world!".to_string(),
    /// };
    ///
    /// // Iterate over the fields of `my_instance`:
    /// for (field_name, field_value) in my_instance.iter_mut() {
    ///     if let Some(num) = field_value.downcast_mut::<i32>() {
    ///         *num += 1;
    ///     }
    /// }
    /// ```
    fn iter_mut(&mut self) -> std::vec::IntoIter<(&'static str, &'_ mut dyn std::any::Any)>;
}