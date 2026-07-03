//! Iterator-contract details of FieldIter/FieldIterAs that adapters rely on:
//! meeting-in-the-middle for double-ended iteration, len/size_hint during
//! partial consumption, and clone independence.

use struct_iterable::Iterable;

#[derive(Iterable)]
struct Four {
    a: u8,
    b: u8,
    c: u8,
    d: u8,
}

fn sample() -> Four {
    Four {
        a: 1,
        b: 2,
        c: 3,
        d: 4,
    }
}

#[test]
fn next_and_next_back_meet_in_the_middle() {
    let s = sample();
    let mut it = s.iter();
    assert_eq!(it.next().unwrap().0, "a");
    assert_eq!(it.next_back().unwrap().0, "d");
    assert_eq!(it.next().unwrap().0, "b");
    assert_eq!(it.next_back().unwrap().0, "c");
    assert!(it.next().is_none());
    assert!(it.next_back().is_none());
    assert_eq!(it.size_hint(), (0, Some(0)));
    // Fused: stays None after exhaustion.
    assert!(it.next().is_none());
}

#[test]
fn len_and_size_hint_track_partial_consumption() {
    let s = sample();
    let mut it = s.iter();
    assert_eq!(it.len(), 4);
    it.next();
    assert_eq!(it.len(), 3);
    assert_eq!(it.size_hint(), (3, Some(3)));
    it.next_back();
    assert_eq!(it.len(), 2);
}

#[test]
fn clones_iterate_independently() {
    let s = sample();
    let mut first = s.iter();
    first.next();
    let mut second = first.clone();
    first.next();

    // `second` still sees "b" even though `first` moved past it.
    assert_eq!(second.next().unwrap().0, "b");
    assert_eq!(first.next().unwrap().0, "c");
    assert_eq!(second.len(), 2);
    assert_eq!(first.len(), 1);
}
