//! Derives `Iterable` exactly as a user of the published facade 0.1.1 would,
//! but with the local (unreleased) derive doing the code generation.

use struct_iterable::Iterable;

#[derive(Iterable)]
struct Config {
    name: String,
    port: u16,
    enabled: bool,
}

fn main() {
    let config = Config {
        name: "demo".to_string(),
        port: 8080,
        enabled: true,
    };

    for (field_name, field_value) in config.iter() {
        if let Some(port) = field_value.downcast_ref::<u16>() {
            println!("{field_name}: {port}");
        } else {
            println!("{field_name}");
        }
    }
}
