struct SDiff<
    '__daft,
    'a: '__daft,
    'b: '__daft,
    'daft: 'a + '__daft,
    'inv: '__daft,
    'contra: '__daft,
> {
    multi_ref: <&'a &'b Vec<u8> as ::daft::Diffable>::Diff<'__daft>,
    bound_ref: <&'daft Vec<u8> as ::daft::Diffable>::Diff<'__daft>,
    inv_ref: <PhantomData<Cell<&'inv ()>> as ::daft::Diffable>::Diff<'__daft>,
    contra_ref: <PhantomData<fn(&'contra ())> as ::daft::Diffable>::Diff<'__daft>,
}
impl<
    '__daft,
    'a: '__daft,
    'b: '__daft,
    'daft: 'a + '__daft,
    'inv: '__daft,
    'contra: '__daft,
> ::std::fmt::Debug for SDiff<'__daft, 'a, 'b, 'daft, 'inv, 'contra>
where
    <&'a &'b Vec<u8> as ::daft::Diffable>::Diff<'__daft>: ::std::fmt::Debug,
    <&'daft Vec<u8> as ::daft::Diffable>::Diff<'__daft>: ::std::fmt::Debug,
    <PhantomData<Cell<&'inv ()>> as ::daft::Diffable>::Diff<'__daft>: ::std::fmt::Debug,
    <PhantomData<fn(&'contra ())> as ::daft::Diffable>::Diff<'__daft>: ::std::fmt::Debug,
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
    '__daft,
    'a: '__daft,
    'b: '__daft,
    'daft: 'a + '__daft,
    'inv: '__daft,
    'contra: '__daft,
> ::std::cmp::PartialEq for SDiff<'__daft, 'a, 'b, 'daft, 'inv, 'contra>
where
    <&'a &'b Vec<u8> as ::daft::Diffable>::Diff<'__daft>: ::std::cmp::PartialEq,
    <&'daft Vec<u8> as ::daft::Diffable>::Diff<'__daft>: ::std::cmp::PartialEq,
    <PhantomData<
        Cell<&'inv ()>,
    > as ::daft::Diffable>::Diff<'__daft>: ::std::cmp::PartialEq,
    <PhantomData<
        fn(&'contra ()),
    > as ::daft::Diffable>::Diff<'__daft>: ::std::cmp::PartialEq,
{
    fn eq(&self, other: &Self) -> bool {
        self.multi_ref == other.multi_ref && self.bound_ref == other.bound_ref
            && self.inv_ref == other.inv_ref && self.contra_ref == other.contra_ref
    }
}
impl<
    '__daft,
    'a: '__daft,
    'b: '__daft,
    'daft: 'a + '__daft,
    'inv: '__daft,
    'contra: '__daft,
> ::std::cmp::Eq for SDiff<'__daft, 'a, 'b, 'daft, 'inv, 'contra>
where
    <&'a &'b Vec<u8> as ::daft::Diffable>::Diff<'__daft>: ::std::cmp::Eq,
    <&'daft Vec<u8> as ::daft::Diffable>::Diff<'__daft>: ::std::cmp::Eq,
    <PhantomData<Cell<&'inv ()>> as ::daft::Diffable>::Diff<'__daft>: ::std::cmp::Eq,
    <PhantomData<fn(&'contra ())> as ::daft::Diffable>::Diff<'__daft>: ::std::cmp::Eq,
{}
impl<'a, 'b, 'daft: 'a, 'inv, 'contra> ::daft::Diffable
for S<'a, 'b, 'daft, 'inv, 'contra> {
    type Diff<'__daft> = SDiff<'__daft, 'a, 'b, 'daft, 'inv, 'contra>
    where
        Self: '__daft;
    fn diff<'__daft>(
        &'__daft self,
        other: &'__daft Self,
    ) -> SDiff<'__daft, 'a, 'b, 'daft, 'inv, 'contra> {
        Self::Diff {
            multi_ref: ::daft::Diffable::diff(&self.multi_ref, &other.multi_ref),
            bound_ref: ::daft::Diffable::diff(&self.bound_ref, &other.bound_ref),
            inv_ref: ::daft::Diffable::diff(&self.inv_ref, &other.inv_ref),
            contra_ref: ::daft::Diffable::diff(&self.contra_ref, &other.contra_ref),
        }
    }
}
