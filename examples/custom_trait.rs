use struct_iterable::Iterable;

// Use the iterable attribute to derive a custom trait instead
#[derive(Iterable)]
#[iterable(ToString)]
struct MyStruct {
    a: u32,
    b: String,
}

fn main() {
    let my_struct = MyStruct {
        a: 42,
        b: String::from("foobar"),
    };

    for (key, value) in my_struct.iter() {
        println!("{key}={}", value.to_string());
    }
}
