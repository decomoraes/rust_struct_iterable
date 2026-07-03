use struct_iterable::Iterable;
struct Point {
    x: f32,
    y: f32,
}
#[automatically_derived]
impl ::struct_iterable::Iterable for Point
where
    f32: ::core::any::Any,
{
    #[inline]
    fn field_count(&self) -> usize {
        2usize
    }
    fn field_at(
        &self,
        index: usize,
    ) -> ::core::option::Option<(&'static str, &dyn ::core::any::Any)> {
        match index {
            0usize => {
                ::core::option::Option::Some(("x", &self.x as &dyn ::core::any::Any))
            }
            1usize => {
                ::core::option::Option::Some(("y", &self.y as &dyn ::core::any::Any))
            }
            _ => ::core::option::Option::None,
        }
    }
    #[inline]
    fn iter(&self) -> ::struct_iterable::FieldIter<'_> {
        ::struct_iterable::FieldIter::new(self)
    }
}
#[automatically_derived]
impl ::struct_iterable::IterableMut for Point
where
    f32: ::core::any::Any,
{
    fn field_at_mut(
        &mut self,
        index: usize,
    ) -> ::core::option::Option<(&'static str, &mut dyn ::core::any::Any)> {
        match index {
            0usize => {
                ::core::option::Option::Some((
                    "x",
                    &mut self.x as &mut dyn ::core::any::Any,
                ))
            }
            1usize => {
                ::core::option::Option::Some((
                    "y",
                    &mut self.y as &mut dyn ::core::any::Any,
                ))
            }
            _ => ::core::option::Option::None,
        }
    }
}
fn main() {}
