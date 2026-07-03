use struct_iterable::Iterable;

#[derive(Iterable)]
struct Borrowed<'a> {
    s: &'a str,
}

fn main() {
    let s = "hello";
    let b = Borrowed { s };
    for (name, _) in b.iter() {
        println!("{name}");
    }
}
