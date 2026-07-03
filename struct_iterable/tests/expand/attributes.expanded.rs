use struct_iterable::Iterable;
trait Checkable {}
#[iterable(to(dyn Checkable))]
struct Kitchen<'a, T> {
    plain: T,
    #[iterable(rename = "kebab-name")]
    renamed: u32,
    #[iterable(skip)]
    skipped: &'a str,
}
#[automatically_derived]
impl<'a, T> ::struct_iterable::Iterable for Kitchen<'a, T>
where
    T: ::core::any::Any,
    u32: ::core::any::Any,
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
                ::core::option::Option::Some((
                    "plain",
                    &self.plain as &dyn ::core::any::Any,
                ))
            }
            1usize => {
                ::core::option::Option::Some((
                    "kebab-name",
                    &self.renamed as &dyn ::core::any::Any,
                ))
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
impl<'a, T> ::struct_iterable::IterableMut for Kitchen<'a, T>
where
    T: ::core::any::Any,
    u32: ::core::any::Any,
{
    fn field_at_mut(
        &mut self,
        index: usize,
    ) -> ::core::option::Option<(&'static str, &mut dyn ::core::any::Any)> {
        match index {
            0usize => {
                ::core::option::Option::Some((
                    "plain",
                    &mut self.plain as &mut dyn ::core::any::Any,
                ))
            }
            1usize => {
                ::core::option::Option::Some((
                    "kebab-name",
                    &mut self.renamed as &mut dyn ::core::any::Any,
                ))
            }
            _ => ::core::option::Option::None,
        }
    }
}
#[automatically_derived]
impl<'__i, 'a, T> ::struct_iterable::IterableAs<dyn Checkable + '__i> for Kitchen<'a, T>
where
    T: Checkable + '__i,
    u32: Checkable + '__i,
{
    #[inline]
    fn field_count_as(&self) -> usize {
        2usize
    }
    fn field_at_as(
        &self,
        index: usize,
    ) -> ::core::option::Option<(&'static str, &(dyn Checkable + '__i))> {
        match index {
            0usize => {
                ::core::option::Option::Some((
                    "plain",
                    &self.plain as &(dyn Checkable + '__i),
                ))
            }
            1usize => {
                ::core::option::Option::Some((
                    "kebab-name",
                    &self.renamed as &(dyn Checkable + '__i),
                ))
            }
            _ => ::core::option::Option::None,
        }
    }
    #[inline]
    fn iter_as(&self) -> ::struct_iterable::FieldIterAs<'_, dyn Checkable + '__i> {
        ::struct_iterable::FieldIterAs::new(self)
    }
}
fn main() {}
