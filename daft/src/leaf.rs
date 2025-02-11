use crate::Diffable;
use core::ops::{Deref, DerefMut};

/// A primitive or atomic change.
///
/// `T` is normally a reference of some kind, but it can be any type.
///
/// For more information, see the [crate-level documentation](crate).
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub struct Leaf<T> {
    /// The value on the before side.
    pub before: T,

    /// The value on the after side.
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
    ///
    /// # Example
    ///
    /// ```
    /// # #[cfg(feature = "std")] {
    /// use daft::Leaf;
    ///
    /// let x: Leaf<String> = Leaf { before: "hello".to_owned(), after: "world".to_owned() };
    /// assert_eq!(x.as_deref(), Leaf { before: "hello", after: "world" });
    /// # }
    /// ```
    #[inline]
    pub fn as_deref(&self) -> Leaf<&T::Target>
    where
        T: Deref,
    {
        Leaf { before: &*self.before, after: &*self.after }
    }

    /// Convert from `Leaf<T>` or `&mut Leaf<T>` to `Leaf<&mut T::Target>`.
    ///
    /// # Example
    ///
    /// ```
    /// # #[cfg(feature = "std")] {
    /// use daft::Leaf;
    ///
    /// let mut x: Leaf<String> = Leaf { before: "hello".to_owned(), after: "world".to_owned() };
    /// assert_eq!(
    ///     x.as_deref_mut().map(|x| {
    ///         x.make_ascii_uppercase();
    ///         x
    ///     }),
    ///     Leaf { before: "HELLO".to_owned().as_mut_str(), after: "WORLD".to_owned().as_mut_str() },
    /// );
    /// # }
    /// ```
    #[inline]
    pub fn as_deref_mut(&mut self) -> Leaf<&mut T::Target>
    where
        T: DerefMut,
    {
        Leaf { before: &mut *self.before, after: &mut *self.after }
    }

    /// Map a `Leaf<T>` to a `Leaf<U>` by applying a function to the `before`
    /// and `after` values.
    ///
    /// `f` will be called twice: first with `before`, then with `after`.
    ///
    /// # Example
    ///
    /// A `Leaf<&str>` can be converted to a `Leaf<String>` by calling `map`
    /// with `String::from`:
    ///
    /// ```
    /// # #[cfg(feature = "std")] {
    /// use daft::{Diffable, Leaf};
    ///
    /// let before = "hello";
    /// let after = "world";
    ///
    /// let leaf: Leaf<&str> = Leaf { before, after };
    /// let leaf: Leaf<String> = leaf.map(String::from);
    /// # }
    /// ```
    #[inline]
    pub fn map<U, F>(self, mut f: F) -> Leaf<U>
    where
        F: FnMut(T) -> U,
    {
        Leaf { before: f(self.before), after: f(self.after) }
    }

    /// Return true if before is the same as after.
    ///
    /// This is the same as `self.before == self.after`, but is easier to use in
    /// a chained series of method calls.
    ///
    /// # Example
    ///
    /// ```
    /// use daft::{Diffable, Leaf};
    ///
    /// let before = "hello";
    /// let after = "hello";
    ///
    /// let leaf: Leaf<&str> = Leaf { before, after };
    /// assert!(leaf.is_unchanged());
    ///
    /// let before = "hello";
    /// let after = "world";
    ///
    /// let leaf: Leaf<&str> = Leaf { before, after };
    /// assert!(!leaf.is_unchanged());
    /// ```
    #[inline]
    pub fn is_unchanged(&self) -> bool
    where
        T: Eq,
    {
        self.before == self.after
    }

    /// Return true if before is different from after.
    ///
    /// This is the same as `self.before != self.after`, but is easier to use in
    /// a chained series of method calls.
    ///
    /// # Example
    ///
    /// ```
    /// use daft::{Diffable, Leaf};
    ///
    /// let before = "hello";
    /// let after = "hello";
    ///
    /// let leaf: Leaf<&str> = Leaf { before, after };
    /// assert!(!leaf.is_modified());
    ///
    /// let before = "hello";
    /// let after = "world";
    ///
    /// let leaf: Leaf<&str> = Leaf { before, after };
    /// assert!(leaf.is_modified());
    /// ```
    #[inline]
    pub fn is_modified(&self) -> bool
    where
        T: Eq,
    {
        self.before != self.after
    }
}

impl<'daft, T: ?Sized + Diffable> Leaf<&'daft T> {
    /// Perform a diff on [`before`][Self::before] and [`after`][Self::after],
    /// returning `T::Diff`.
    ///
    /// This is useful when `T::Diff` is not a leaf node.
    #[inline]
    pub fn diff_pair(self) -> T::Diff<'daft> {
        self.before.diff(self.after)
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
