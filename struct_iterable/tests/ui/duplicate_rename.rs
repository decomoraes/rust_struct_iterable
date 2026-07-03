use struct_iterable::Iterable;

#[derive(Iterable)]
struct Config {
    #[iterable(rename = "x")]
    a: u32,
    #[iterable(rename = "x")]
    b: u32,
}

fn main() {}
