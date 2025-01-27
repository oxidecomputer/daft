use diffwalk::{Diffable, Leaf};
use diffwalk_derive::Diff;
use std::collections::{BTreeMap, BTreeSet};
use uuid::Uuid;

#[derive(Debug, Eq, PartialEq, Diff)]
enum SomeEnum {
    A,
    B,
    C(u32),
}

#[derive(Debug, Eq, PartialEq, Diff)]
struct SomeStruct {
    a: i32,
}

#[derive(Debug, Eq, PartialEq, Diff)]
struct Large {
    a: i32,
    b: SomeEnum,
    c: BTreeMap<Uuid, BTreeSet<usize>>,
    d: SomeStruct,
}

#[test]
fn test_basic() {
    let a = SomeEnum::A;
    let b = SomeEnum::B;

    // Enums are just `Leaf`s. We don't try to walk them. User code can do that
    // as necessary.
    let diff = a.diff(&b);
    let expected = Leaf { before: &SomeEnum::A, after: &SomeEnum::B };
    assert_eq!(diff, expected);

    let a = SomeStruct { a: 0 };
    let b = SomeStruct { a: 1 };
    let diff = a.diff(&b);

    // This is our generated type for a diff
    let expected = SomeStructDiff { a: Leaf { before: &0, after: &1 } };
    assert_eq!(diff, expected);

    let shared_id = Uuid::new_v4();
    let c1: BTreeMap<Uuid, BTreeSet<usize>> =
        [(shared_id, [1, 2, 3, 4, 5].into_iter().collect())]
            .into_iter()
            .collect();
    let mut c2 = c1.clone();
    c2.get_mut(&shared_id).unwrap().remove(&3);
    c2.get_mut(&shared_id).unwrap().insert(6);
    c2.insert(Uuid::new_v4(), [9].into_iter().collect());
    let a = Large { a: 0, b: SomeEnum::C(4), c: c1, d: SomeStruct { a: 0 } };
    let b = Large { a: 0, b: SomeEnum::B, c: c2, d: SomeStruct { a: 1 } };
    let diff = a.diff(&b);
    println!("{:#?}", diff);
}
