use crate::Diffable;
use core::ops::{Deref, DerefMut};

/// A primitive or atomic change.
///
/// `T` is normally a reference of some kind, but it can be any type.
///
/// For more information, see the [crate-level documentation](crate).
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub struct Leaf<T> {
    pub before: T,
    pub after: T,
}

impl<T> Leaf<T> {
    /// Convert from `&Leaf<T>` to `Leaf<&T>`.
    #[inline]
    pub fn as_ref(&self) -> Leaf<&T> {
        Leaf { before: &self.before, after: &self.after }
    }

    /// Convert from `&mut Leaf<T>` to `Leaf<&mut T>`.
    #[inline]
    pub fn as_mut(&mut self) -> Leaf<&mut T> {
        Leaf { before: &mut self.before, after: &mut self.after }
    }

    /// Convert from `Leaf<T>` or `&Leaf<T>` to `Leaf<&T::Target>`.
    #[inline]
    pub fn as_deref(&self) -> Leaf<&T::Target>
    where
        T: Deref,
    {
        Leaf { before: &*self.before, after: &*self.after }
    }

    /// Convert from `Leaf<T>` or `&mut Leaf<T>` to `Leaf<&mut T::Target>`.
    #[inline]
    pub fn as_deref_mut(&mut self) -> Leaf<&mut T::Target>
    where
        T: DerefMut,
    {
        Leaf { before: &mut *self.before, after: &mut *self.after }
    }
}

impl<'daft, T: ?Sized + Diffable> Leaf<&'daft T> {
    /// Perform a diff on [`before`][Self::before] and [`after`][Self::after],
    /// returning `T::Diff`.
    ///
    /// This is useful when `T::Diff` is not a leaf node.
    #[inline]
    pub fn diff_pair(self) -> T::Diff<'daft> {
        self.before.diff(&self.after)
    }
}

impl<T> Leaf<&T> {
    /// Create a clone of the leaf with owned values.
    ///
    /// # Example
    ///
    /// ```
    /// # #[cfg(feature = "std")] {
    /// use daft::{Diffable, Leaf};
    ///
    /// let before = String::from("hello");
    /// let after = String::from("world");
    ///
    /// let leaf: Leaf<&String> = Leaf { before: &before, after: &after };
    /// // cloned turns a Leaf<&String> into a Leaf<String>.
    /// let leaf: Leaf<String> = leaf.cloned();
    /// assert_eq!(leaf.before, "hello");
    /// assert_eq!(leaf.after, "world");
    /// # }
    /// ```
    #[inline]
    pub fn cloned(self) -> Leaf<T>
    where
        T: Clone,
    {
        Leaf { before: self.before.clone(), after: self.after.clone() }
    }

    /// Create a copy of the leaf with owned values.
    ///
    /// # Example
    ///
    /// ```
    /// use daft::{Diffable, Leaf};
    ///
    /// let before = "hello";
    /// let after = "world";
    ///
    /// let leaf: Leaf<&&str> = Leaf { before: &before, after: &after };
    /// // copied turns a Leaf<&&str> into a Leaf<&str>.
    /// let leaf: Leaf<&str> = leaf.copied();
    /// assert_eq!(leaf.before, "hello");
    /// assert_eq!(leaf.after, "world");
    /// ```
    #[inline]
    pub fn copied(self) -> Leaf<T>
    where
        T: Copy,
    {
        Leaf { before: *self.before, after: *self.after }
    }
}
