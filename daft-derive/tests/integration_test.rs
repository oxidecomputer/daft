use daft::{Diffable, Leaf};
use std::{
    collections::{BTreeMap, BTreeSet},
    fmt::Debug,
};
use uuid::Uuid;

#[derive(Debug, Eq, PartialEq, Diffable)]
enum SomeEnum {
    A,
    B,
    C(u32),
}

#[derive(Debug, Eq, PartialEq, Diffable)]
struct SomeStruct {
    a: i32,
}

#[derive(Debug, Eq, PartialEq, Diffable)]
struct Large {
    a: i32,
    b: SomeEnum,
    c: BTreeMap<Uuid, BTreeSet<usize>>,
    d: SomeStruct,
}

#[derive(Debug, Eq, PartialEq, Diffable)]
struct TupleStruct(String);

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
    println!("{diff:#?}");

    assert_eq!(diff.a.before, diff.a.after);
    assert_eq!(diff.b.before, &SomeEnum::C(4));
    assert_eq!(diff.b.after, &SomeEnum::B);
    assert_eq!(diff.c.unchanged().count(), 0);
    assert_eq!(diff.c.added.len(), 1);
    assert_eq!(diff.c.removed.len(), 0);
    assert_eq!(diff.c.modified().count(), 1);

    let set_diff = &diff.c.modified_diff().next().unwrap().1;
    assert_eq!(set_diff.common, [&1, &2, &4, &5].into_iter().collect());
    assert_eq!(set_diff.added, [&6].into_iter().collect());
    assert_eq!(set_diff.removed, [&3].into_iter().collect());

    assert_eq!(diff.d.a.before, &0);
    assert_eq!(diff.d.a.after, &1);

    let a = TupleStruct("oxide".into());
    let b = TupleStruct("computer company".into());
    let diff = a.diff(&b);
    assert_eq!(diff.0.before, &"oxide".to_string());
    assert_eq!(diff.0.after, &"computer company".to_string());
    println!("{diff:#?}");
}

#[test]
fn test_enum_with_generics() {
    #[derive(Debug, Eq, PartialEq, Diffable)]
    enum EnumWithGenerics<'a, T, U> {
        A(T),
        B(&'a U),
    }

    let x = 5usize;
    let y = 5u8;
    let a = EnumWithGenerics::A(x);
    let b = EnumWithGenerics::B(&y);
    let diff = a.diff(&b);
    assert_eq!(Leaf { before: &a, after: &b }, diff);
}

#[test]
fn test_struct_with_generics() {
    #[derive(Debug, Eq, PartialEq, Diffable)]
    struct StructWithGenerics<'d, 'e, T, U>
    where
        T: Diffable + 'd,
        U: Diffable + 'e,
    {
        b: usize,
        c: &'d T,
        d: &'e U,
    }

    let x = StructWithGenerics { b: 6, c: &5, d: &6 };
    let y = StructWithGenerics { b: 7, c: &5, d: &7 };
    let diff = x.diff(&y);

    assert_eq!(diff.b, Leaf { before: &6, after: &7 });
    assert_eq!(diff.c, Leaf { before: &5, after: &5 });
    assert_eq!(diff.d, Leaf { before: &6, after: &7 });
    println!("{diff:?}");

    #[derive(Debug, Eq, PartialEq, Diffable)]
    struct S<'a, T, U>
    where
        T: Diffable + Eq + 'a,
        U: Diffable + 'a,
    {
        a: BTreeMap<usize, T>,
        b: usize,
        c: &'a U,
        d: &'a str,
    }

    let x = S {
        a: [(5, 2usize)].into_iter().collect(),
        b: 5,
        c: &6usize,
        d: "hello",
    };
    let y = S {
        a: [(5, 1usize)].into_iter().collect(),
        b: 5,
        c: &6usize,
        d: "world",
    };
    let diff = x.diff(&y);

    assert_eq!(diff.a.unchanged().count(), 0);
    assert_eq!(diff.a.modified().count(), 1);
    assert_eq!(diff.a.added.len(), 0);
    assert_eq!(diff.a.removed.len(), 0);
    assert_eq!(diff.b.before, diff.b.after);
    assert_eq!(diff.c.before, diff.c.after);
    assert_eq!(diff.d.before, "hello");
    assert_eq!(diff.d.after, "world");

    println!("{diff:#?}");
}

#[test]
fn test_empty_structs() {
    // Cover every shape that yields an empty diff: unit, empty named, empty
    // tuple, and any of the above where every field is `#[daft(ignore)]`.
    #[derive(Debug, Eq, PartialEq, Diffable)]
    struct UnitStruct;

    #[derive(Debug, Eq, PartialEq, Diffable)]
    struct EmptyNamed {}

    #[derive(Debug, Eq, PartialEq, Diffable)]
    struct EmptyTuple();

    #[derive(Debug, Eq, PartialEq, Diffable)]
    struct AllIgnoredNamed {
        #[daft(ignore)]
        a: i32,
        #[daft(ignore)]
        b: String,
    }

    #[derive(Debug, Eq, PartialEq, Diffable)]
    struct AllIgnoredTuple(#[daft(ignore)] i32, #[daft(ignore)] String);

    // Two diffs of any empty type should compare equal -- there is nothing to
    // distinguish them.
    assert_eq!(UnitStruct.diff(&UnitStruct), UnitStruct.diff(&UnitStruct));
    assert_eq!(
        EmptyNamed {}.diff(&EmptyNamed {}),
        EmptyNamed {}.diff(&EmptyNamed {})
    );
    assert_eq!(
        EmptyTuple().diff(&EmptyTuple()),
        EmptyTuple().diff(&EmptyTuple())
    );

    // For all-ignored structs, even values that differ in the ignored fields
    // must produce equal diffs.
    let a = AllIgnoredNamed { a: 1, b: "x".into() };
    let b = AllIgnoredNamed { a: 2, b: "y".into() };
    assert_eq!(a.diff(&b), a.diff(&a));

    let a = AllIgnoredTuple(1, "x".into());
    let b = AllIgnoredTuple(2, "y".into());
    assert_eq!(a.diff(&b), a.diff(&a));

    // Debug output is just the type name with no field listing.
    assert_eq!(format!("{:?}", UnitStruct.diff(&UnitStruct)), "UnitStructDiff");
    assert_eq!(
        format!(
            "{:?}",
            AllIgnoredTuple(0, String::new())
                .diff(&AllIgnoredTuple(0, String::new()))
        ),
        "AllIgnoredTupleDiff",
    );

    // Empty diff structs should be `Send + Sync` regardless of the original
    // type's auto-traits -- there is no data inside to share.
    #[derive(Diffable)]
    struct AllIgnoredNonSync {
        #[daft(ignore)]
        _c: std::cell::Cell<u32>,
    }

    fn assert_send<T: Send>() {}
    fn assert_sync<T: Sync>() {}

    assert_send::<UnitStructDiff<'_>>();
    assert_sync::<UnitStructDiff<'_>>();
    assert_send::<AllIgnoredTupleDiff<'_>>();
    assert_sync::<AllIgnoredTupleDiff<'_>>();
    assert_send::<AllIgnoredNonSyncDiff<'_>>();
    assert_sync::<AllIgnoredNonSyncDiff<'_>>();
}

#[test]
fn diff_pair_lifetimes() {
    // Complex type to ensure lifetimes are correct.
    #[derive(Diffable)]
    struct Inner {
        a: u32,
        b: &'static str,
    }

    #[derive(Diffable)]
    struct Outer {
        #[daft(leaf)]
        inner: Inner,
    }

    let owned: Leaf<String> = {
        let before = Outer { inner: Inner { a: 5, b: "hello" } };
        let after = Outer { inner: Inner { a: 6, b: "world" } };

        let diff = before.diff(&after);
        let inner_diff = {
            let inner: Leaf<&Inner> = diff.inner;
            // Ensure that inner.diff_pair outlives inner.
            inner.diff_pair()
        };

        assert_eq!(*inner_diff.a.before, 5);
        assert_eq!(*inner_diff.a.after, 6);
        assert_eq!(inner_diff.b.before, "hello");
        assert_eq!(inner_diff.b.after, "world");

        // The return value of this will outlive before and after as well.
        inner_diff.b.map(str::to_owned)
    };

    assert_eq!(owned.before, "hello");
    assert_eq!(owned.after, "world");
}

#[cfg(feature = "serde")]
mod changes_serde {
    //! End-to-end coverage of `#[daft(changes)]` plus the `serde` feature:
    //! the projected diff must round-trip through `serde_json` with every
    //! unchanged leaf omitted.

    use daft::{Diffable, IntoChanges};
    use serde_json::json;
    use std::collections::BTreeMap;

    #[test]
    fn struct_with_changes_serializes_only_modified_fields() {
        #[derive(Debug, Eq, PartialEq, Diffable)]
        #[daft(changes)]
        struct Config {
            name: String,
            retries: u32,
        }

        let before = Config { name: "alpha".to_owned(), retries: 3 };
        let after = Config { name: "alpha".to_owned(), retries: 5 };

        let changes =
            before.diff(&after).into_changes().expect("retries changed");
        let json = serde_json::to_value(&changes).unwrap();
        assert_eq!(json, json!({ "retries": { "before": 3, "after": 5 } }));

        // Two equal values produce no changes at all.
        assert!(before.diff(&before).into_changes().is_none());
    }

    #[test]
    fn nested_struct_changes_serialize_recursively() {
        #[derive(Debug, Eq, PartialEq, Diffable)]
        #[daft(changes)]
        struct Inner {
            value: u32,
            note: String,
        }

        #[derive(Debug, Eq, PartialEq, Diffable)]
        #[daft(changes)]
        struct Outer {
            inner: Inner,
            tag: u32,
        }

        let before =
            Outer { inner: Inner { value: 1, note: "a".to_owned() }, tag: 7 };
        let after =
            Outer { inner: Inner { value: 2, note: "a".to_owned() }, tag: 7 };

        let changes =
            before.diff(&after).into_changes().expect("inner.value changed");
        let json = serde_json::to_value(&changes).unwrap();

        // `note` is unchanged so it's dropped; `tag` is unchanged so it's
        // dropped; only `inner.value` survives.
        assert_eq!(
            json,
            json!({ "inner": { "value": { "before": 1, "after": 2 } } }),
        );
    }

    #[test]
    fn map_changes_filter_unchanged_entries() {
        #[derive(Debug, Eq, PartialEq, Diffable)]
        #[daft(changes)]
        struct Cache {
            entries: BTreeMap<u32, &'static str>,
        }

        let before = Cache {
            entries: [(1, "alpha"), (2, "beta"), (3, "gamma")]
                .into_iter()
                .collect(),
        };
        let after = Cache {
            entries: [(1, "alpha"), (2, "BETA"), (4, "delta")]
                .into_iter()
                .collect(),
        };

        let changes = before.diff(&after).into_changes().expect("map changed");
        let json = serde_json::to_value(&changes).unwrap();

        assert_eq!(
            json,
            json!({
                "entries": {
                    "common": {
                        "2": { "before": "beta", "after": "BETA" },
                    },
                    "added": { "4": "delta" },
                    "removed": { "3": "gamma" },
                },
            }),
        );
    }

    #[test]
    fn tuple_struct_changes_serialize_positionally() {
        #[derive(Debug, Eq, PartialEq, Diffable)]
        #[daft(changes)]
        struct Pair(u32, &'static str);

        let before = Pair(1, "same");
        let after = Pair(2, "same");

        let changes =
            before.diff(&after).into_changes().expect("first changed");
        // Tuple structs serialize as a sequence with `null` for skipped
        // positions in formats that preserve nulls; we rely on
        // `serialize_tuple_struct` to elide positions entirely.
        let json = serde_json::to_value(&changes).unwrap();
        assert_eq!(json, json!([{ "before": 1, "after": 2 }]));
    }
}
