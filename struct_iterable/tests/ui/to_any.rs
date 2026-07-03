use struct_iterable::Iterable;

#[derive(Iterable)]
#[iterable(to(dyn std::any::Any))]
struct Config {
    retries: u32,
}

fn main() {}
