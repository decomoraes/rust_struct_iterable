//! The dominant real-world use case: validate a config struct by iterating
//! its fields instead of writing one `if` per field.
//!
//! Run with: `cargo run -p struct_iterable --example config_validation`

use struct_iterable::Iterable;

#[derive(Iterable)]
struct ServerConfig {
    host: String,
    port: u16,
    api_key: Option<String>,
    tls_cert: Option<String>,
    log_level: String,
}

/// Returns the names of fields that are empty strings or `None`.
///
/// Taking `&impl Iterable` means this works for any derived config struct,
/// not just `ServerConfig`.
fn unset_fields(config: &impl Iterable) -> Vec<&'static str> {
    config
        .iter()
        .filter(|(_, value)| {
            if let Some(s) = value.downcast_ref::<String>() {
                s.is_empty()
            } else if let Some(opt) = value.downcast_ref::<Option<String>>() {
                opt.is_none()
            } else {
                false
            }
        })
        .map(|(name, _)| name)
        .collect()
}

fn main() {
    let config = ServerConfig {
        host: "localhost".to_string(),
        port: 8080,
        api_key: None,
        tls_cert: None,
        log_level: String::new(),
    };

    let unset = unset_fields(&config);
    if unset.is_empty() {
        println!("config OK");
    } else {
        println!("config has {} unset field(s):", unset.len());
        for name in &unset {
            println!("  - {name}");
        }
    }
}
