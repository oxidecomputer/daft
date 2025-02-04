#[derive(Debug, PartialEq, Eq)]
struct BasicDiff<'daft> {
    a: <i32 as daft::Diffable>::Diff<'daft>,
    b: <BTreeMap<Uuid, BTreeSet<usize>> as daft::Diffable>::Diff<'daft>,
}
impl daft::Diffable for Basic {
    type Diff<'daft> = BasicDiff<'daft> where Self: 'daft;
    fn diff<'daft>(&'daft self, other: &'daft Self) -> BasicDiff<'daft> {
        Self::Diff {
            a: daft::Diffable::diff(&self.a, &other.a),
            b: daft::Diffable::diff(&self.b, &other.b),
        }
    }
}
