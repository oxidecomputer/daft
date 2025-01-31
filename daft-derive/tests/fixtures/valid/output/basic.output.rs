#[derive(Debug, PartialEq, Eq)]
struct BasicDiff<'daft> {
    a: <i32 as daft::Diffable<'daft>>::Diff,
    b: <BTreeMap<Uuid, BTreeSet<usize>> as daft::Diffable<'daft>>::Diff,
}
impl<'daft> daft::Diffable<'daft> for Basic {
    type Diff = BasicDiff<'daft>;
    fn diff(&'daft self, other: &'daft Self) -> Self::Diff {
        Self::Diff {
            a: daft::Diffable::diff(&self.a, &other.a),
            b: daft::Diffable::diff(&self.b, &other.b),
        }
    }
}
