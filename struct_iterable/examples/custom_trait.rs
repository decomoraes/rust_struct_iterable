//! Iterate a struct's fields as `&dyn Trait` of your choosing instead of
//! `&dyn Any`, via `#[iterable(to(dyn Trait))]` — including for structs that
//! borrow non-'static data, which the `dyn Any` path can never support.
//!
//! Run with: `cargo run -p struct_iterable --example custom_trait`

use struct_iterable::{Iterable, IterableAs};

trait Checkable {
    fn ok(&self) -> bool;
}

struct MaxJobs(u32);
struct Caches<'a>(&'a str);

impl Checkable for MaxJobs {
    fn ok(&self) -> bool {
        self.0 > 0
    }
}

impl<'a> Checkable for Caches<'a> {
    fn ok(&self) -> bool {
        !self.0.is_empty()
    }
}

// Every non-skipped field must implement `Checkable` — enforced at compile
// time, with the error spanned to the offending field.
//
// `no_any` suppresses the dyn-Any-based `Iterable`/`IterableMut` impls,
// which `Caches<'a>` could never satisfy (`Any` requires 'static).
#[derive(Iterable)]
#[iterable(to(dyn Checkable), no_any)]
struct NixHealth<'a> {
    max_jobs: MaxJobs,
    caches: Caches<'a>,
}

fn main() {
    // Non-'static data, borrowed by the struct:
    let cache_url = String::from("https://cache.nixos.org");
    let health = NixHealth {
        max_jobs: MaxJobs(4),
        caches: Caches(&cache_url),
    };

    // Fields come out in declaration order as (&'static str, &dyn Checkable),
    // so we can call the trait directly — no downcasting.
    for (name, check) in health.iter_as() {
        println!("{name}: {}", if check.ok() { "ok" } else { "FAILED" });
    }
}
