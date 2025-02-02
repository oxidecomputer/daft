#[derive(Debug, PartialEq, Eq)]
struct WithAttrsDiff<'daft> {
    a: <i32 as daft::Diffable<'daft>>::Diff,
    b: <BTreeMap<Uuid, BTreeSet<usize>> as daft::Diffable<'daft>>::Diff,
    d: daft::Leaf<'daft, Lazy>,
    e: daft::Leaf<'daft, usize>,
    f: <usize as daft::Diffable<'daft>>::Diff,
}
impl<'daft> daft::Diffable<'daft> for WithAttrs {
    type Diff = WithAttrsDiff<'daft>;
    fn diff(&'daft self, other: &'daft Self) -> Self::Diff {
        Self::Diff {
            a: daft::Diffable::diff(&self.a, &other.a),
            b: daft::Diffable::diff(&self.b, &other.b),
            d: daft::Leaf {
                before: &self.d,
                after: &other.d,
            },
            e: daft::Leaf {
                before: &self.e,
                after: &other.e,
            },
            f: daft::Diffable::diff(&self.f, &other.f),
        }
    }
}
