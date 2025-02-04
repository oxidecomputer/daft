struct SDiff<'daft, 'a: 'daft, 'b: 'daft, 'c: 'a + 'daft, 'inv: 'daft, 'contra: 'daft> {
    multi_ref: <&'a &'b Vec<u8> as daft::Diffable>::Diff<'daft>,
    bound_ref: <&'c Vec<u8> as daft::Diffable>::Diff<'daft>,
    inv_ref: <PhantomData<Cell<&'inv ()>> as daft::Diffable>::Diff<'daft>,
    contra_ref: <PhantomData<fn(&'contra ())> as daft::Diffable>::Diff<'daft>,
}
impl<
    'daft,
    'a: 'daft,
    'b: 'daft,
    'c: 'a + 'daft,
    'inv: 'daft,
    'contra: 'daft,
> ::std::fmt::Debug for SDiff<'daft, 'a, 'b, 'c, 'inv, 'contra>
where
    <&'a &'b Vec<u8> as daft::Diffable>::Diff<'daft>: ::std::fmt::Debug,
    <&'c Vec<u8> as daft::Diffable>::Diff<'daft>: ::std::fmt::Debug,
    <PhantomData<Cell<&'inv ()>> as daft::Diffable>::Diff<'daft>: ::std::fmt::Debug,
    <PhantomData<fn(&'contra ())> as daft::Diffable>::Diff<'daft>: ::std::fmt::Debug,
{
    fn fmt(&self, f: &mut ::std::fmt::Formatter<'_>) -> ::std::fmt::Result {
        f.debug_struct(stringify!(SDiff))
            .field(stringify!(multi_ref), &self.multi_ref)
            .field(stringify!(bound_ref), &self.bound_ref)
            .field(stringify!(inv_ref), &self.inv_ref)
            .field(stringify!(contra_ref), &self.contra_ref)
            .finish()
    }
}
impl<
    'daft,
    'a: 'daft,
    'b: 'daft,
    'c: 'a + 'daft,
    'inv: 'daft,
    'contra: 'daft,
> ::std::cmp::PartialEq for SDiff<'daft, 'a, 'b, 'c, 'inv, 'contra>
where
    <&'a &'b Vec<u8> as daft::Diffable>::Diff<'daft>: ::std::cmp::PartialEq,
    <&'c Vec<u8> as daft::Diffable>::Diff<'daft>: ::std::cmp::PartialEq,
    <PhantomData<Cell<&'inv ()>> as daft::Diffable>::Diff<'daft>: ::std::cmp::PartialEq,
    <PhantomData<fn(&'contra ())> as daft::Diffable>::Diff<'daft>: ::std::cmp::PartialEq,
{
    fn eq(&self, other: &Self) -> bool {
        self.multi_ref == other.multi_ref && self.bound_ref == other.bound_ref
            && self.inv_ref == other.inv_ref && self.contra_ref == other.contra_ref
    }
}
impl<
    'daft,
    'a: 'daft,
    'b: 'daft,
    'c: 'a + 'daft,
    'inv: 'daft,
    'contra: 'daft,
> ::std::cmp::Eq for SDiff<'daft, 'a, 'b, 'c, 'inv, 'contra>
where
    <&'a &'b Vec<u8> as daft::Diffable>::Diff<'daft>: ::std::cmp::Eq,
    <&'c Vec<u8> as daft::Diffable>::Diff<'daft>: ::std::cmp::Eq,
    <PhantomData<Cell<&'inv ()>> as daft::Diffable>::Diff<'daft>: ::std::cmp::Eq,
    <PhantomData<fn(&'contra ())> as daft::Diffable>::Diff<'daft>: ::std::cmp::Eq,
{}
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
