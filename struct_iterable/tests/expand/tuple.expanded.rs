use struct_iterable::Iterable;
struct Wrapper(u32, String);
#[automatically_derived]
impl ::struct_iterable::Iterable for Wrapper
where
    u32: ::core::any::Any,
    String: ::core::any::Any,
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
                ::core::option::Option::Some(("0", &self.0 as &dyn ::core::any::Any))
            }
            1usize => {
                ::core::option::Option::Some(("1", &self.1 as &dyn ::core::any::Any))
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
impl ::struct_iterable::IterableMut for Wrapper
where
    u32: ::core::any::Any,
    String: ::core::any::Any,
{
    fn field_at_mut(
        &mut self,
        index: usize,
    ) -> ::core::option::Option<(&'static str, &mut dyn ::core::any::Any)> {
        match index {
            0usize => {
                ::core::option::Option::Some((
                    "0",
                    &mut self.0 as &mut dyn ::core::any::Any,
                ))
            }
            1usize => {
                ::core::option::Option::Some((
                    "1",
                    &mut self.1 as &mut dyn ::core::any::Any,
                ))
            }
            _ => ::core::option::Option::None,
        }
    }
}
fn main() {}
