use struct_iterable::Iterable;

#[derive(Iterable)]
#[iterable(no_any)]
struct Config {
    retries: u32,
}

fn main() {}
