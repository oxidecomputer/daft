use crate::{Diffable, Leaf};
use newtype_uuid::{TypedUuid, TypedUuidKind};

impl<T> Diffable for TypedUuid<T>
where
    T: TypedUuidKind + Diffable,
{
    type Diff<'daft> = Leaf<&'daft TypedUuid<T>>;

    fn diff<'daft>(&'daft self, other: &'daft Self) -> Self::Diff<'daft> {
        Leaf { before: self, after: other }
    }
}
