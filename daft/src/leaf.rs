use crate::Diffable;

/// A primitive or atomic change.
///
/// For more information, see the [crate-level documentation](crate).
#[derive(Debug, PartialEq, Eq)]
pub struct Leaf<'daft, T: ?Sized> {
    pub before: &'daft T,
    pub after: &'daft T,
}

impl<'daft, T: Diffable + ?Sized> Leaf<'daft, T> {
    /// Perform a diff on before and after, returning `T::Diff`.
    ///
    /// This is useful when `T::Diff` is not a leaf node.
    #[inline]
    pub fn diff_pair(&self) -> T::Diff<'daft> {
        self.before.diff(self.after)
    }
}

impl<'daft, T: ?Sized> Leaf<'daft, &T> {
    /// Map a `Leaf<&T>` to a `Leaf<T>`.
    ///
    /// # Example
    ///
    /// ```
    /// use daft::{Diffable, Leaf};
    ///
    /// let before = "hello";
    /// let after = "world";
    ///
    /// let leaf: Leaf<'_, &str> = Leaf { before: &before, after: &after };
    /// // unref turns a Leaf<&str> into a Leaf<str>.
    /// let leaf: Leaf<'_, str> = leaf.unref();
    /// assert_eq!(leaf.before, "hello");
    /// assert_eq!(leaf.after, "world");
    /// ```
    #[inline]
    pub fn unref(self) -> Leaf<'daft, T> {
        Leaf { before: *self.before, after: *self.after }
    }
}

// Hand-implement Clone and Copy so that it doesn't require T: Copy.
impl<T: ?Sized> Clone for Leaf<'_, T> {
    #[inline]
    fn clone(&self) -> Self {
        *self
    }
}
impl<T: ?Sized> Copy for Leaf<'_, T> {}
