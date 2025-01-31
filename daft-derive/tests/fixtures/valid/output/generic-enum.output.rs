impl<
    'a: 'daft,
    'daft,
    T: Eq + Debug + 'daft,
    U: Eq + Debug + 'daft,
> daft::Diffable<'daft> for EnumWithGenerics<'a, T, U> {
    type Diff = daft::Leaf<'daft, Self>;
    fn diff(&'daft self, other: &'daft Self) -> Self::Diff {
        daft::Leaf {
            before: self,
            after: other,
        }
    }
}
