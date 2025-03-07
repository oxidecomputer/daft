struct BasicDiff<'__daft> {
    a: <i32 as ::daft::Diffable>::Diff<'__daft>,
    b: <BTreeMap<Uuid, BTreeSet<usize>> as ::daft::Diffable>::Diff<'__daft>,
}
impl<'__daft> ::core::fmt::Debug for BasicDiff<'__daft>
where
    <i32 as ::daft::Diffable>::Diff<'__daft>: ::core::fmt::Debug,
    <BTreeMap<
        Uuid,
        BTreeSet<usize>,
    > as ::daft::Diffable>::Diff<'__daft>: ::core::fmt::Debug,
{
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_struct(stringify!(BasicDiff))
            .field(stringify!(a), &self.a)
            .field(stringify!(b), &self.b)
            .finish()
    }
}
impl<'__daft> ::core::cmp::PartialEq for BasicDiff<'__daft>
where
    <i32 as ::daft::Diffable>::Diff<'__daft>: ::core::cmp::PartialEq,
    <BTreeMap<
        Uuid,
        BTreeSet<usize>,
    > as ::daft::Diffable>::Diff<'__daft>: ::core::cmp::PartialEq,
{
    fn eq(&self, other: &Self) -> bool {
        self.a == other.a && self.b == other.b
    }
}
impl<'__daft> ::core::cmp::Eq for BasicDiff<'__daft>
where
    <i32 as ::daft::Diffable>::Diff<'__daft>: ::core::cmp::Eq,
    <BTreeMap<
        Uuid,
        BTreeSet<usize>,
    > as ::daft::Diffable>::Diff<'__daft>: ::core::cmp::Eq,
{}
impl ::daft::Diffable for Basic {
    type Diff<'__daft> = BasicDiff<'__daft> where Self: '__daft;
    fn diff<'__daft>(&'__daft self, other: &'__daft Self) -> BasicDiff<'__daft> {
        Self::Diff {
            a: ::daft::Diffable::diff(&self.a, &other.a),
            b: ::daft::Diffable::diff(&self.b, &other.b),
        }
    }
}
