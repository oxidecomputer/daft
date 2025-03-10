impl ::daft::Diffable for Inner {
    type Diff<'__daft> = ::daft::Leaf<&'__daft Self> where Self: '__daft;
    fn diff<'__daft>(&'__daft self, other: &'__daft Self) -> Self::Diff<'__daft> {
        ::daft::Leaf {
            before: self,
            after: other,
        }
    }
}
