#[derive(Debug, PartialEq, Eq)]
struct SDiff<'a, 'daft, T, U>
where
    for<'x> T: Diffable<'x> + Debug + Eq + 'x,
    U: Diffable<'a> + Debug + Eq,
{
    a: <BTreeMap<usize, T> as daft::Diffable<'daft>>::Diff,
    b: <usize as daft::Diffable<'daft>>::Diff,
    c: <U as daft::Diffable<'a>>::Diff,
}
impl<'a: 'daft, 'daft, T: 'daft, U: 'daft> daft::Diffable<'daft> for S<'a, T, U>
where
    for<'x> T: Diffable<'x> + Debug + Eq + 'x,
    U: Diffable<'a> + Debug + Eq,
{
    type Diff = SDiff<'a, 'daft, T, U>;
    fn diff(&'daft self, other: &'daft Self) -> Self::Diff {
        Self::Diff {
            a: daft::Diffable::diff(&self.a, &other.a),
            b: daft::Diffable::diff(&self.b, &other.b),
            c: daft::Diffable::diff(&*self.c, &*other.c),
        }
    }
}
