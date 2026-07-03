use struct_iterable::Iterable;

#[derive(Iterable)]
#[iterable(unknown_option)]
struct Config {
    retries: u32,
}

fn main() {}
