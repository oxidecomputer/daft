//! Implementations for core types.

use crate::{leaf, Diffable, Leaf};
use core::{
    cell::RefCell,
    marker::PhantomData,
    net::{IpAddr, Ipv4Addr, Ipv6Addr, SocketAddr, SocketAddrV4, SocketAddrV6},
};

leaf! { i64, i32, i16, i8, u64, u32, u16, u8, char, bool, isize, usize, () }
leaf! { IpAddr, Ipv4Addr, Ipv6Addr, SocketAddr, SocketAddrV4, SocketAddrV6 }
leaf! { str }

impl<T> Diffable for Option<T> {
    type Diff<'daft>
        = Leaf<Option<&'daft T>>
    where
        T: 'daft;

    fn diff<'daft>(&'daft self, other: &'daft Self) -> Self::Diff<'daft> {
        Leaf { before: self.as_ref(), after: other.as_ref() }
    }
}

impl<T, U> Diffable for Result<T, U> {
    type Diff<'daft>
        = Leaf<Result<&'daft T, &'daft U>>
    where
        T: 'daft,
        U: 'daft;
    fn diff<'daft>(&'daft self, other: &'daft Self) -> Self::Diff<'daft> {
        Leaf { before: self.as_ref(), after: other.as_ref() }
    }
}

impl<'a, T: Diffable + ?Sized> Diffable for &'a T {
    // It would be nice to extend the lifetime of the diff to `'a` (e.g. diffing
    // &'static str`s results in a Leaf<&'static str>), and it does actually
    // work for that simpler case, but rustc (1.84.1) complains about cases with
    // multiple references like &'a &'b T (see complex-lifetimes.rs for an
    // example). This is probably a bug in rustc: see
    // https://gist.github.com/sunshowers/25fcc2f590f1c6b19daa99e11e927dc7 for
    // the error with Rust 1.84.1.
    type Diff<'daft>
        = <T as Diffable>::Diff<'daft>
    where
        'a: 'daft;

    fn diff<'daft>(&'daft self, other: &'daft Self) -> Self::Diff<'daft> {
        (**self).diff(other)
    }
}

// Can't express lifetimes due to `RefCell`'s limited borrows, so we must return
// a leaf node that can be recursively diffed.
impl<T: ?Sized> Diffable for RefCell<T> {
    type Diff<'daft>
        = Leaf<&'daft Self>
    where
        T: 'daft;

    fn diff<'daft>(&'daft self, other: &'daft Self) -> Self::Diff<'daft> {
        Leaf { before: self, after: other }
    }
}

impl<T: ?Sized> Diffable for PhantomData<T> {
    type Diff<'daft>
        = Leaf<&'daft PhantomData<T>>
    where
        Self: 'daft;

    fn diff<'daft>(&'daft self, other: &'daft Self) -> Self::Diff<'daft> {
        Leaf { before: self, after: other }
    }
}

/// Treat slices as leaf nodes.
impl<T: Diffable> Diffable for [T] {
    type Diff<'daft>
        = Leaf<&'daft [T]>
    where
        T: 'daft;

    fn diff<'daft>(&'daft self, other: &'daft Self) -> Self::Diff<'daft> {
        Leaf { before: self, after: other }
    }
}

macro_rules! tuple_diffable {
    ($(($($name:ident $ix:tt),+)),+) => {
        $(
            impl<$($name: Diffable),+> Diffable for ($($name,)+) {
                type Diff<'daft> = ($($name::Diff<'daft>,)+)
                where
                    $($name: 'daft,)+;

                fn diff<'daft>(&'daft self, other: &'daft Self) -> Self::Diff<'daft> {
                    ($(self.$ix.diff(&other.$ix),)+)
                }
            }
        )+
    }
}

tuple_diffable! {
    (A 0),
    (A 0, B 1),
    (A 0, B 1, C 2),
    (A 0, B 1, C 2, D 3),
    (A 0, B 1, C 2, D 3, E 4),
    (A 0, B 1, C 2, D 3, E 4, F 5),
    (A 0, B 1, C 2, D 3, E 4, F 5, G 6),
    (A 0, B 1, C 2, D 3, E 4, F 5, G 6, H 7),
    (A 0, B 1, C 2, D 3, E 4, F 5, G 6, H 7, I 8),
    (A 0, B 1, C 2, D 3, E 4, F 5, G 6, H 7, I 8, J 9)
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn reference_diffable() {
        // Test that a reference to a diffable type can be diffed.
        let before = &&&&&&"hello";
        let after = &&&&&&"world";

        // This should automatically dereference the references.
        let diff: Leaf<&str> = before.diff(after);
        assert_eq!(diff.before, ******before);
        assert_eq!(diff.after, ******after);
    }

    #[test]
    fn tuple_diffable() {
        let before = (1usize, 2usize, 3usize);
        let after = (4, 5, 6);

        let diff = before.diff(&after);
        assert_eq!(
            diff,
            (
                Leaf { before: &1, after: &4 },
                Leaf { before: &2, after: &5 },
                Leaf { before: &3, after: &6 },
            )
        );
    }
}
