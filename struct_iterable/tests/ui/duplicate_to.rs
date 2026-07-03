use struct_iterable::Iterable;

#[derive(Iterable)]
#[iterable(to(dyn std::fmt::Debug))]
#[iterable(to(dyn std::fmt::Debug))]
struct Dup {
    x: u32,
}

fn main() {}
