struct BasicDiff<'daft> {
    a: <i32 as daft::Diffable>::Diff<'daft>,
    b: <BTreeMap<Uuid, BTreeSet<usize>> as daft::Diffable>::Diff<'daft>,
}
impl<'daft> ::std::fmt::Debug for BasicDiff<'daft>
where
    <i32 as daft::Diffable>::Diff<'daft>: ::std::fmt::Debug,
    <BTreeMap<Uuid, BTreeSet<usize>> as daft::Diffable>::Diff<'daft>: ::std::fmt::Debug,
{
    fn fmt(&self, f: &mut ::std::fmt::Formatter<'_>) -> ::std::fmt::Result {
        f.debug_struct(stringify!(BasicDiff))
            .field(stringify!(a), &self.a)
            .field(stringify!(b), &self.b)
            .finish()
    }
}
impl<'daft> ::std::cmp::PartialEq for BasicDiff<'daft>
where
    <i32 as daft::Diffable>::Diff<'daft>: ::std::cmp::PartialEq,
    <BTreeMap<
        Uuid,
        BTreeSet<usize>,
    > as daft::Diffable>::Diff<'daft>: ::std::cmp::PartialEq,
{
    fn eq(&self, other: &Self) -> bool {
        self.a == other.a && self.b == other.b
    }
}
impl<'daft> ::std::cmp::Eq for BasicDiff<'daft>
where
    <i32 as daft::Diffable>::Diff<'daft>: ::std::cmp::Eq,
    <BTreeMap<Uuid, BTreeSet<usize>> as daft::Diffable>::Diff<'daft>: ::std::cmp::Eq,
{}
impl daft::Diffable for Basic {
    type Diff<'daft> = BasicDiff<'daft> where Self: 'daft;
    fn diff<'daft>(&'daft self, other: &'daft Self) -> BasicDiff<'daft> {
        Self::Diff {
            a: daft::Diffable::diff(&self.a, &other.a),
            b: daft::Diffable::diff(&self.b, &other.b),
        }
    }
}
