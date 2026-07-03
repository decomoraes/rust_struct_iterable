use struct_iterable::Iterable;

#[derive(Iterable)]
struct Wrapper<T> {
    inner: T,
}

fn main() {
    let w = Wrapper { inner: 42u32 };
    for (name, _) in w.iter() {
        println!("{name}");
    }
}
