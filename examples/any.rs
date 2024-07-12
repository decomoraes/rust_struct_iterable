use struct_iterable::Iterable;

// Use iterable attribut without custom item
// defaults to std::any::Any
#[derive(Iterable)]
struct MyStruct {
    print_me: Option<String>,
    do_not_print_me: Vec<u8>,
}

fn main() {
    let my_struct = MyStruct {
        print_me: Some("the test works".to_string()),
        do_not_print_me: vec![4,2],
    };

    for (key, value) in my_struct.iter() {
        if let Some(string_opt) = value.downcast_ref::<Option<String>>() {
            if let Some(string) = string_opt.as_deref() {
                println!("{key}=\"{string}\"");
            }
        }
    }
}
