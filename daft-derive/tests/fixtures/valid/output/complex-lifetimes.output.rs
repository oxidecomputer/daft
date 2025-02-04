#[derive(Debug, PartialEq, Eq)]
struct SDiff<'daft, 'a: 'daft, 'b: 'daft, 'c: 'a + 'daft, 'inv: 'daft, 'contra: 'daft> {
    multi_ref: <&'a &'b Vec<u8> as daft::Diffable>::Diff<'daft>,
    bound_ref: <&'c Vec<u8> as daft::Diffable>::Diff<'daft>,
    inv_ref: <PhantomData<Cell<&'inv ()>> as daft::Diffable>::Diff<'daft>,
    contra_ref: <PhantomData<fn(&'contra ())> as daft::Diffable>::Diff<'daft>,
}
impl<'a, 'b, 'c: 'a, 'inv, 'contra> daft::Diffable for S<'a, 'b, 'c, 'inv, 'contra> {
    type Diff<'daft> = SDiff<'daft, 'a, 'b, 'c, 'inv, 'contra> where Self: 'daft;
    fn diff<'daft>(
        &'daft self,
        other: &'daft Self,
    ) -> SDiff<'daft, 'a, 'b, 'c, 'inv, 'contra> {
        Self::Diff {
            multi_ref: daft::Diffable::diff(&self.multi_ref, &other.multi_ref),
            bound_ref: daft::Diffable::diff(&self.bound_ref, &other.bound_ref),
            inv_ref: daft::Diffable::diff(&self.inv_ref, &other.inv_ref),
            contra_ref: daft::Diffable::diff(&self.contra_ref, &other.contra_ref),
        }
    }
}
