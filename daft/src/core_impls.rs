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
        = Leaf<'daft, Option<T>>
    where
        T: 'daft;

    fn diff<'daft>(&'daft self, other: &'daft Self) -> Self::Diff<'daft> {
        Leaf { before: self, after: other }
    }
}

impl<T, U> Diffable for Result<T, U> {
    type Diff<'daft>
        = Leaf<'daft, Result<T, U>>
    where
        T: 'daft,
        U: 'daft;
    fn diff<'daft>(&'daft self, other: &'daft Self) -> Self::Diff<'daft> {
        Leaf { before: self, after: other }
    }
}

impl<'a, T: Diffable + ?Sized> Diffable for &'a T {
    type Diff<'daft>
        = <T as Diffable>::Diff<'daft>
    where
        &'a T: 'daft;

    fn diff<'daft>(&'daft self, other: &'daft Self) -> Self::Diff<'daft> {
        (**self).diff(other)
    }
}

// Can't express lifetimes due to `RefCell`'s limited borrows, so we must return
// a leaf node that can be recursively diffed.
impl<T: ?Sized> Diffable for RefCell<T> {
    type Diff<'daft>
        = Leaf<'daft, Self>
    where
        T: 'daft;

    fn diff<'daft>(&'daft self, other: &'daft Self) -> Self::Diff<'daft> {
        Leaf { before: self, after: other }
    }
}

impl<T: ?Sized> Diffable for PhantomData<T> {
    type Diff<'daft>
        = Leaf<'daft, PhantomData<T>>
    where
        Self: 'daft;

    fn diff<'daft>(&'daft self, other: &'daft Self) -> Self::Diff<'daft> {
        Leaf { before: self, after: other }
    }
}

/// Treat slices as leaf nodes.
impl<T: Diffable> Diffable for [T] {
    type Diff<'daft>
        = Leaf<'daft, [T]>
    where
        T: 'daft;

    fn diff<'daft>(&'daft self, other: &'daft Self) -> Self::Diff<'daft> {
        Leaf { before: self, after: other }
    }
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
        let diff: Leaf<'_, str> = before.diff(&after);
        assert_eq!(diff.before, ******before);
        assert_eq!(diff.after, ******after);
    }
}
