#[derive(Debug, PartialEq, Eq)]
struct SDiff<'daft, 'a: 'daft, T: 'daft, U: 'daft>
where
    T: Diffable + Debug + Eq + 'a,
    U: Diffable + Debug + Eq + 'a,
{
    a: <BTreeMap<usize, T> as daft::Diffable>::Diff<'daft>,
    b: <usize as daft::Diffable>::Diff<'daft>,
    c: <&'a U as daft::Diffable>::Diff<'daft>,
    d: <&'a str as daft::Diffable>::Diff<'daft>,
}
impl<'a, T, U> daft::Diffable for S<'a, T, U>
where
    T: Diffable + Debug + Eq + 'a,
    U: Diffable + Debug + Eq + 'a,
{
    type Diff<'daft> = SDiff<'daft, 'a, T, U> where Self: 'daft;
    fn diff<'daft>(&'daft self, other: &'daft Self) -> SDiff<'daft, 'a, T, U> {
        Self::Diff {
            a: daft::Diffable::diff(&self.a, &other.a),
            b: daft::Diffable::diff(&self.b, &other.b),
            c: daft::Diffable::diff(&self.c, &other.c),
            d: daft::Diffable::diff(&self.d, &other.d),
        }
    }
}
