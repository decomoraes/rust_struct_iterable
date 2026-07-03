use struct_iterable::Iterable;

#[derive(Iterable)]
struct Borrowed<'a> {
    s: &'a str,
}

fn main() {
    // A field borrowing non-'static data can never be erased to `dyn Any`
    // (`Any` requires `'static`), so calling `iter()` here must not compile.
    let owned = String::from("hello");
    let b = Borrowed { s: &owned };
    for (name, _) in b.iter() {
        println!("{name}");
    }
}
