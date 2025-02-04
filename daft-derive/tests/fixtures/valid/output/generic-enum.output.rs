impl<'a, T: Eq + Debug, U: Eq + Debug> daft::Diffable for EnumWithGenerics<'a, T, U> {
    type Diff<'daft> = daft::Leaf<'daft, Self> where Self: 'daft;
    fn diff<'daft>(&'daft self, other: &'daft Self) -> Self::Diff<'daft> {
        daft::Leaf {
            before: self,
            after: other,
        }
    }
}
