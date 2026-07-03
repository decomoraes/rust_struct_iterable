use struct_iterable::Iterable;

trait Marker {}
impl Marker for u32 {}

// no_any suppresses the Iterable/IterableMut impls entirely: calling iter()
// must not compile.
#[derive(Iterable)]
#[iterable(no_any, to(dyn Marker))]
struct NoAny {
    x: u32,
}

fn main() {
    let n = NoAny { x: 1 };
    for _ in n.iter() {}
}
