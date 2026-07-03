use struct_iterable::Iterable;

#[derive(Iterable)]
union Raw {
    int: u32,
    float: f32,
}

fn main() {}
