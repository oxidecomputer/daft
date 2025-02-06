use crate::{Diffable, Leaf};

impl<T> Diffable for newtype_uuid::TypedUuid<T>
where
    T: newtype_uuid::TypedUuidKind + Diffable,
{
    type Diff<'daft> = Leaf<'daft, newtype_uuid::TypedUuid<T>>;

    fn diff<'daft>(&'daft self, other: &'daft Self) -> Self::Diff<'daft> {
        Leaf { before: self, after: other }
    }
}
