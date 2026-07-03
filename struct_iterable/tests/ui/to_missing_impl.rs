use struct_iterable::Iterable;

// `count` does not implement Display, so the to(dyn Display) request must
// fail with the bound error underlining the offending field's type.
#[derive(Iterable)]
#[iterable(to(dyn std::fmt::Display))]
struct Report {
    title: String,
    count: Vec<u8>,
}

fn main() {
    let r = Report {
        title: "t".into(),
        count: vec![1],
    };
    use struct_iterable::IterableAs;
    for (_name, _value) in r.iter_as() {}
}
