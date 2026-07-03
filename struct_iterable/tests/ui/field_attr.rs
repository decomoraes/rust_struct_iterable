use struct_iterable::Iterable;

#[derive(Iterable)]
struct Config {
    #[iterable(skip)]
    secret: String,
    retries: u32,
}

fn main() {}
