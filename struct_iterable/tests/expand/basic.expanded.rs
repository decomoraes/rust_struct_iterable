use struct_iterable::Iterable;
struct Point {
    x: f32,
    y: f32,
}
#[automatically_derived]
impl Iterable for Point
where
    f32: ::std::any::Any,
{
    fn iter(&self) -> ::std::vec::IntoIter<(&'static str, &'_ dyn ::std::any::Any)> {
        ::alloc::boxed::box_assume_init_into_vec_unsafe(
                ::alloc::intrinsics::write_box_via_move(
                    ::alloc::boxed::Box::new_uninit(),
                    [
                        ("x", &(self.x) as &dyn ::std::any::Any),
                        ("y", &(self.y) as &dyn ::std::any::Any),
                    ],
                ),
            )
            .into_iter()
    }
}
fn main() {}
