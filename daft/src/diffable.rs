/// Represents a type which can be diffed.
///
/// For more information, see the [crate-level documentation](crate).
pub trait Diffable {
    /// The type of the diff.
    ///
    /// This is a [generic associated type][GAT], also known as a GAT. The
    /// `'daft` lifetime is used in the `diff` method to ensure that the
    /// returned diff is valid for the lifetime of the input values.
    ///
    /// [GAT]: https://blog.rust-lang.org/2021/08/03/GATs-stabilization-push.html
    type Diff<'daft>
    where
        Self: 'daft;

    /// Compute the diff between two values.
    fn diff<'daft>(&'daft self, other: &'daft Self) -> Self::Diff<'daft>;
}
