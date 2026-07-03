//! IterableAs (issue #5): iterate fields as `&dyn UserTrait` instead of
//! `&dyn Any`, including for structs that borrow non-'static data — the
//! scenario the `dyn Any` path can never support.

use std::any::Any;
use std::fmt::Display;
use struct_iterable::{Iterable, IterableAs, IterableAsExt};

trait Checkable {
    fn ok(&self) -> bool;
}

#[derive(PartialEq, Debug)]
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

// The NixHealth shape from issue #5: a lifetime-parameterized struct whose
// fields all implement one trait. `no_any` because `Caches<'a>` can never be
// `dyn Any`.
#[derive(Iterable)]
#[iterable(to(dyn Checkable), no_any)]
struct NixHealth<'a> {
    max_jobs: MaxJobs,
    caches: Caches<'a>,
}

#[test]
fn borrowed_struct_iterates_as_custom_trait() {
    let owned = String::from("https://cache.nixos.org");
    let health = NixHealth {
        max_jobs: MaxJobs(4),
        caches: Caches(&owned),
    };

    let mut names = Vec::new();
    for (name, check) in health.iter_as() {
        names.push(name);
        assert!(check.ok());
    }
    assert_eq!(names, ["max_jobs", "caches"]);
    assert_eq!(health.field_count_as(), 2);
    assert!(health.field_at_as(2).is_none());
}

// A 'static struct can expose several trait objects at once, on top of the
// usual dyn Any path.
#[derive(Iterable)]
#[iterable(to(dyn Display))]
#[iterable(to(dyn Checkable))]
struct Stats {
    jobs: MaxJobs2,
    load: f64,
}

#[derive(PartialEq, Debug)]
struct MaxJobs2(u32);

impl Checkable for MaxJobs2 {
    fn ok(&self) -> bool {
        self.0 > 0
    }
}
impl Checkable for f64 {
    fn ok(&self) -> bool {
        !self.is_nan()
    }
}
impl Display for MaxJobs2 {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{} jobs", self.0)
    }
}

#[test]
fn multiple_trait_objects_disambiguated_with_turbofish() {
    let stats = Stats {
        jobs: MaxJobs2(2),
        load: 0.5,
    };

    let rendered: Vec<String> = stats
        .iter_fields_as::<dyn Display>()
        .map(|(name, value)| format!("{name}={value}"))
        .collect();
    assert_eq!(rendered, ["jobs=2 jobs", "load=0.5"]);

    let all_ok = stats
        .iter_fields_as::<dyn Checkable>()
        .all(|(_, check)| check.ok());
    assert!(all_ok);

    // The plain dyn Any path still works alongside the custom traits.
    let first = stats.field_at(0).unwrap();
    assert_eq!(first.1.downcast_ref::<MaxJobs2>(), Some(&MaxJobs2(2)));
}

#[test]
fn every_iterable_is_iterable_as_dyn_any() {
    #[derive(Iterable)]
    struct Plain {
        x: u32,
    }

    let p = Plain { x: 1 };
    // Through the blanket bridge, without any to(...) attribute:
    let fields: Vec<(&'static str, &dyn Any)> = p.iter_fields_as::<dyn Any>().collect();
    assert_eq!(fields.len(), 1);
    assert_eq!(fields[0].1.downcast_ref::<u32>(), Some(&1));
}

#[test]
fn iter_as_iterator_capabilities() {
    let owned = String::from("cache");
    let health = NixHealth {
        max_jobs: MaxJobs(1),
        caches: Caches(&owned),
    };

    let it = health.iter_as();
    assert_eq!(it.len(), 2);
    let reversed: Vec<&'static str> = health.iter_as().rev().map(|(name, _)| name).collect();
    assert_eq!(reversed, ["caches", "max_jobs"]);
    let cloned = health.iter_as().clone().count();
    assert_eq!(cloned, 2);
}

#[test]
fn works_through_a_trait_object_of_iterable_as() {
    let owned = String::from("cache");
    let health = NixHealth {
        max_jobs: MaxJobs(1),
        caches: Caches(&owned),
    };
    let dyn_health: &dyn IterableAs<dyn Checkable> = &health;
    assert_eq!(dyn_health.field_count_as(), 2);
    assert!(dyn_health.iter_as().all(|(_, check)| check.ok()));
}
