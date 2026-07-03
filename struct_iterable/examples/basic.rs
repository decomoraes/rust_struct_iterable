//! Derive `Iterable`, loop over the fields, downcast the values.
//!
//! Run with: `cargo run -p struct_iterable --example basic`

use struct_iterable::Iterable;

#[derive(Iterable)]
struct Connection {
    host: String,
    port: u16,
    keepalive: bool,
}

fn main() {
    let conn = Connection {
        host: "localhost".into(),
        port: 5432,
        keepalive: true,
    };

    // Fields come out in declaration order as (&'static str, &dyn Any).
    for (name, value) in conn.iter() {
        if let Some(text) = value.downcast_ref::<String>() {
            println!("{name} = {text:?}");
        } else if let Some(number) = value.downcast_ref::<u16>() {
            println!("{name} = {number}");
        } else if let Some(flag) = value.downcast_ref::<bool>() {
            println!("{name} = {flag}");
        }
    }
}
