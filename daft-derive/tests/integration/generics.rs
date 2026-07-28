use daft::{Diffable, Leaf};
use std::collections::BTreeMap;

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

        // The return value of this will outlive before and after as well.
        inner_diff.b.map(str::to_owned)
    };

    assert_eq!(owned.before, "hello");
    assert_eq!(owned.after, "world");
}
