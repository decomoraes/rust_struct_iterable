use struct_iterable::Iterable;

#[derive(Iterable)]
#[iterable(to(dyn std::fmt::Debug + 'static))]
struct Config {
    retries: u32,
}

fn main() {}
