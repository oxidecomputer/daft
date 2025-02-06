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
    println!("{:#?}", diff);

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
    println!("{:#?}", diff);
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

        inner_diff.b.map(str::to_owned)
    };

    assert_eq!(owned.before, "hello");
    assert_eq!(owned.after, "world");
}
