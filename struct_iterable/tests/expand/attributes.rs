use struct_iterable::Iterable;

trait Checkable {}

#[derive(Iterable)]
#[iterable(to(dyn Checkable))]
struct Kitchen<'a, T> {
    plain: T,
    #[iterable(rename = "kebab-name")]
    renamed: u32,
    #[iterable(skip)]
    skipped: &'a str,
}

fn main() {}
