struct ConfigDiff<'__daft> {
    name: <String as ::daft::Diffable>::Diff<'__daft>,
    retries: <u32 as ::daft::Diffable>::Diff<'__daft>,
    tags: <BTreeMap<u32, String> as ::daft::Diffable>::Diff<'__daft>,
}
impl<'__daft> ::core::fmt::Debug for ConfigDiff<'__daft>
where
    <String as ::daft::Diffable>::Diff<'__daft>: ::core::fmt::Debug,
    <u32 as ::daft::Diffable>::Diff<'__daft>: ::core::fmt::Debug,
    <BTreeMap<u32, String> as ::daft::Diffable>::Diff<'__daft>: ::core::fmt::Debug,
{
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_struct(stringify!(ConfigDiff))
            .field(stringify!(name), &self.name)
            .field(stringify!(retries), &self.retries)
            .field(stringify!(tags), &self.tags)
            .finish()
    }
}
impl<'__daft> ::core::cmp::PartialEq for ConfigDiff<'__daft>
where
    <String as ::daft::Diffable>::Diff<'__daft>: ::core::cmp::PartialEq,
    <u32 as ::daft::Diffable>::Diff<'__daft>: ::core::cmp::PartialEq,
    <BTreeMap<u32, String> as ::daft::Diffable>::Diff<'__daft>: ::core::cmp::PartialEq,
{
    fn eq(&self, other: &Self) -> bool {
        self.name == other.name && self.retries == other.retries
            && self.tags == other.tags
    }
}
impl<'__daft> ::core::cmp::Eq for ConfigDiff<'__daft>
where
    <String as ::daft::Diffable>::Diff<'__daft>: ::core::cmp::Eq,
    <u32 as ::daft::Diffable>::Diff<'__daft>: ::core::cmp::Eq,
    <BTreeMap<u32, String> as ::daft::Diffable>::Diff<'__daft>: ::core::cmp::Eq,
{}
impl ::daft::Diffable for Config {
    type Diff<'__daft> = ConfigDiff<'__daft> where Self: '__daft;
    fn diff<'__daft>(&'__daft self, other: &'__daft Self) -> ConfigDiff<'__daft> {
        Self::Diff {
            name: ::daft::Diffable::diff(&self.name, &other.name),
            retries: ::daft::Diffable::diff(&self.retries, &other.retries),
            tags: ::daft::Diffable::diff(&self.tags, &other.tags),
        }
    }
}
struct ConfigChanges<'__daft>
where
    <String as ::daft::Diffable>::Diff<'__daft>: ::daft::IntoChanges,
    <u32 as ::daft::Diffable>::Diff<'__daft>: ::daft::IntoChanges,
    <BTreeMap<u32, String> as ::daft::Diffable>::Diff<'__daft>: ::daft::IntoChanges,
{
    name: ::core::option::Option<
        <<String as ::daft::Diffable>::Diff<'__daft> as ::daft::IntoChanges>::Changes,
    >,
    retries: ::core::option::Option<
        <<u32 as ::daft::Diffable>::Diff<'__daft> as ::daft::IntoChanges>::Changes,
    >,
    tags: ::core::option::Option<
        <<BTreeMap<
            u32,
            String,
        > as ::daft::Diffable>::Diff<'__daft> as ::daft::IntoChanges>::Changes,
    >,
}
impl<'__daft> ::core::fmt::Debug for ConfigChanges<'__daft>
where
    <<String as ::daft::Diffable>::Diff<
        '__daft,
    > as ::daft::IntoChanges>::Changes: ::core::fmt::Debug,
    <<u32 as ::daft::Diffable>::Diff<
        '__daft,
    > as ::daft::IntoChanges>::Changes: ::core::fmt::Debug,
    <<BTreeMap<
        u32,
        String,
    > as ::daft::Diffable>::Diff<
        '__daft,
    > as ::daft::IntoChanges>::Changes: ::core::fmt::Debug,
{
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_struct(stringify!(ConfigChanges))
            .field(stringify!(name), &self.name)
            .field(stringify!(retries), &self.retries)
            .field(stringify!(tags), &self.tags)
            .finish()
    }
}
impl<'__daft> ::core::cmp::PartialEq for ConfigChanges<'__daft>
where
    <<String as ::daft::Diffable>::Diff<
        '__daft,
    > as ::daft::IntoChanges>::Changes: ::core::cmp::PartialEq,
    <<u32 as ::daft::Diffable>::Diff<
        '__daft,
    > as ::daft::IntoChanges>::Changes: ::core::cmp::PartialEq,
    <<BTreeMap<
        u32,
        String,
    > as ::daft::Diffable>::Diff<
        '__daft,
    > as ::daft::IntoChanges>::Changes: ::core::cmp::PartialEq,
{
    fn eq(&self, other: &Self) -> bool {
        self.name == other.name && self.retries == other.retries
            && self.tags == other.tags
    }
}
impl<'__daft> ::core::cmp::Eq for ConfigChanges<'__daft>
where
    <<String as ::daft::Diffable>::Diff<
        '__daft,
    > as ::daft::IntoChanges>::Changes: ::core::cmp::Eq,
    <<u32 as ::daft::Diffable>::Diff<
        '__daft,
    > as ::daft::IntoChanges>::Changes: ::core::cmp::Eq,
    <<BTreeMap<
        u32,
        String,
    > as ::daft::Diffable>::Diff<
        '__daft,
    > as ::daft::IntoChanges>::Changes: ::core::cmp::Eq,
{}
impl<'__daft> ::daft::IntoChanges for ConfigDiff<'__daft>
where
    <String as ::daft::Diffable>::Diff<'__daft>: ::daft::IntoChanges,
    <u32 as ::daft::Diffable>::Diff<'__daft>: ::daft::IntoChanges,
    <BTreeMap<u32, String> as ::daft::Diffable>::Diff<'__daft>: ::daft::IntoChanges,
{
    type Changes = ConfigChanges<'__daft>;
    fn into_changes(self) -> ::core::option::Option<Self::Changes> {
        let __daft_name = ::daft::IntoChanges::into_changes(self.name);
        let __daft_retries = ::daft::IntoChanges::into_changes(self.retries);
        let __daft_tags = ::daft::IntoChanges::into_changes(self.tags);
        if __daft_name.is_some() || __daft_retries.is_some() || __daft_tags.is_some() {
            ::core::option::Option::Some(ConfigChanges {
                name: __daft_name,
                retries: __daft_retries,
                tags: __daft_tags,
            })
        } else {
            ::core::option::Option::None
        }
    }
}
#[automatically_derived]
impl<'__daft> ::daft::__private_serde::Serialize for ConfigChanges<'__daft>
where
    <<String as ::daft::Diffable>::Diff<
        '__daft,
    > as ::daft::IntoChanges>::Changes: ::daft::__private_serde::Serialize,
    <<u32 as ::daft::Diffable>::Diff<
        '__daft,
    > as ::daft::IntoChanges>::Changes: ::daft::__private_serde::Serialize,
    <<BTreeMap<
        u32,
        String,
    > as ::daft::Diffable>::Diff<
        '__daft,
    > as ::daft::IntoChanges>::Changes: ::daft::__private_serde::Serialize,
{
    fn serialize<__DaftS>(
        &self,
        serializer: __DaftS,
    ) -> ::core::result::Result<__DaftS::Ok, __DaftS::Error>
    where
        __DaftS: ::daft::__private_serde::Serializer,
    {
        let mut __state = ::daft::__private_serde::Serializer::serialize_struct(
            serializer,
            stringify!(ConfigChanges),
            {
                let mut __count = 0usize;
                if self.name.is_some() {
                    __count += 1;
                }
                if self.retries.is_some() {
                    __count += 1;
                }
                if self.tags.is_some() {
                    __count += 1;
                }
                __count
            },
        )?;
        if let ::core::option::Option::Some(__value) = &self.name {
            ::daft::__private_serde::ser::SerializeStruct::serialize_field(
                &mut __state,
                stringify!(name),
                __value,
            )?;
        }
        if let ::core::option::Option::Some(__value) = &self.retries {
            ::daft::__private_serde::ser::SerializeStruct::serialize_field(
                &mut __state,
                stringify!(retries),
                __value,
            )?;
        }
        if let ::core::option::Option::Some(__value) = &self.tags {
            ::daft::__private_serde::ser::SerializeStruct::serialize_field(
                &mut __state,
                stringify!(tags),
                __value,
            )?;
        }
        ::daft::__private_serde::ser::SerializeStruct::end(__state)
    }
}
struct PairDiff<'__daft>(
    <u32 as ::daft::Diffable>::Diff<'__daft>,
    <String as ::daft::Diffable>::Diff<'__daft>,
);
impl<'__daft> ::core::fmt::Debug for PairDiff<'__daft>
where
    <u32 as ::daft::Diffable>::Diff<'__daft>: ::core::fmt::Debug,
    <String as ::daft::Diffable>::Diff<'__daft>: ::core::fmt::Debug,
{
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple(stringify!(PairDiff)).field(&self.0).field(&self.1).finish()
    }
}
impl<'__daft> ::core::cmp::PartialEq for PairDiff<'__daft>
where
    <u32 as ::daft::Diffable>::Diff<'__daft>: ::core::cmp::PartialEq,
    <String as ::daft::Diffable>::Diff<'__daft>: ::core::cmp::PartialEq,
{
    fn eq(&self, other: &Self) -> bool {
        self.0 == other.0 && self.1 == other.1
    }
}
impl<'__daft> ::core::cmp::Eq for PairDiff<'__daft>
where
    <u32 as ::daft::Diffable>::Diff<'__daft>: ::core::cmp::Eq,
    <String as ::daft::Diffable>::Diff<'__daft>: ::core::cmp::Eq,
{}
impl ::daft::Diffable for Pair {
    type Diff<'__daft> = PairDiff<'__daft> where Self: '__daft;
    fn diff<'__daft>(&'__daft self, other: &'__daft Self) -> PairDiff<'__daft> {
        Self::Diff {
            0: ::daft::Diffable::diff(&self.0, &other.0),
            1: ::daft::Diffable::diff(&self.1, &other.1),
        }
    }
}
struct PairChanges<'__daft>(
    ::core::option::Option<
        <<u32 as ::daft::Diffable>::Diff<'__daft> as ::daft::IntoChanges>::Changes,
    >,
    ::core::option::Option<
        <<String as ::daft::Diffable>::Diff<'__daft> as ::daft::IntoChanges>::Changes,
    >,
)
where
    <u32 as ::daft::Diffable>::Diff<'__daft>: ::daft::IntoChanges,
    <String as ::daft::Diffable>::Diff<'__daft>: ::daft::IntoChanges;
impl<'__daft> ::core::fmt::Debug for PairChanges<'__daft>
where
    <<u32 as ::daft::Diffable>::Diff<
        '__daft,
    > as ::daft::IntoChanges>::Changes: ::core::fmt::Debug,
    <<String as ::daft::Diffable>::Diff<
        '__daft,
    > as ::daft::IntoChanges>::Changes: ::core::fmt::Debug,
{
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple(stringify!(PairChanges)).field(&self.0).field(&self.1).finish()
    }
}
impl<'__daft> ::core::cmp::PartialEq for PairChanges<'__daft>
where
    <<u32 as ::daft::Diffable>::Diff<
        '__daft,
    > as ::daft::IntoChanges>::Changes: ::core::cmp::PartialEq,
    <<String as ::daft::Diffable>::Diff<
        '__daft,
    > as ::daft::IntoChanges>::Changes: ::core::cmp::PartialEq,
{
    fn eq(&self, other: &Self) -> bool {
        self.0 == other.0 && self.1 == other.1
    }
}
impl<'__daft> ::core::cmp::Eq for PairChanges<'__daft>
where
    <<u32 as ::daft::Diffable>::Diff<
        '__daft,
    > as ::daft::IntoChanges>::Changes: ::core::cmp::Eq,
    <<String as ::daft::Diffable>::Diff<
        '__daft,
    > as ::daft::IntoChanges>::Changes: ::core::cmp::Eq,
{}
impl<'__daft> ::daft::IntoChanges for PairDiff<'__daft>
where
    <u32 as ::daft::Diffable>::Diff<'__daft>: ::daft::IntoChanges,
    <String as ::daft::Diffable>::Diff<'__daft>: ::daft::IntoChanges,
{
    type Changes = PairChanges<'__daft>;
    fn into_changes(self) -> ::core::option::Option<Self::Changes> {
        let __daft_0 = ::daft::IntoChanges::into_changes(self.0);
        let __daft_1 = ::daft::IntoChanges::into_changes(self.1);
        if __daft_0.is_some() || __daft_1.is_some() {
            ::core::option::Option::Some(PairChanges(__daft_0, __daft_1))
        } else {
            ::core::option::Option::None
        }
    }
}
#[automatically_derived]
impl<'__daft> ::daft::__private_serde::Serialize for PairChanges<'__daft>
where
    <<u32 as ::daft::Diffable>::Diff<
        '__daft,
    > as ::daft::IntoChanges>::Changes: ::daft::__private_serde::Serialize,
    <<String as ::daft::Diffable>::Diff<
        '__daft,
    > as ::daft::IntoChanges>::Changes: ::daft::__private_serde::Serialize,
{
    fn serialize<__DaftS>(
        &self,
        serializer: __DaftS,
    ) -> ::core::result::Result<__DaftS::Ok, __DaftS::Error>
    where
        __DaftS: ::daft::__private_serde::Serializer,
    {
        let mut __state = ::daft::__private_serde::Serializer::serialize_tuple_struct(
            serializer,
            stringify!(PairChanges),
            {
                let mut __count = 0usize;
                if self.0.is_some() {
                    __count += 1;
                }
                if self.1.is_some() {
                    __count += 1;
                }
                __count
            },
        )?;
        if let ::core::option::Option::Some(__value) = &self.0 {
            ::daft::__private_serde::ser::SerializeTupleStruct::serialize_field(
                &mut __state,
                __value,
            )?;
        }
        if let ::core::option::Option::Some(__value) = &self.1 {
            ::daft::__private_serde::ser::SerializeTupleStruct::serialize_field(
                &mut __state,
                __value,
            )?;
        }
        ::daft::__private_serde::ser::SerializeTupleStruct::end(__state)
    }
}
struct OnlyIgnoredDiff<'__daft> {
    _phantom: ::core::marker::PhantomData<fn() -> &'__daft OnlyIgnored>,
}
impl<'__daft> ::core::fmt::Debug for OnlyIgnoredDiff<'__daft> {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_struct(stringify!(OnlyIgnoredDiff)).finish()
    }
}
impl<'__daft> ::core::cmp::PartialEq for OnlyIgnoredDiff<'__daft> {
    fn eq(&self, other: &Self) -> bool {
        true
    }
}
impl<'__daft> ::core::cmp::Eq for OnlyIgnoredDiff<'__daft> {}
impl ::daft::Diffable for OnlyIgnored {
    type Diff<'__daft> = OnlyIgnoredDiff<'__daft> where Self: '__daft;
    fn diff<'__daft>(&'__daft self, other: &'__daft Self) -> OnlyIgnoredDiff<'__daft> {
        Self::Diff {
            _phantom: ::core::marker::PhantomData,
        }
    }
}
struct OnlyIgnoredChanges<'__daft> {
    _phantom: ::core::marker::PhantomData<fn() -> &'__daft OnlyIgnored>,
}
impl<'__daft> ::core::fmt::Debug for OnlyIgnoredChanges<'__daft> {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_struct(stringify!(OnlyIgnoredChanges)).finish()
    }
}
impl<'__daft> ::core::cmp::PartialEq for OnlyIgnoredChanges<'__daft> {
    fn eq(&self, other: &Self) -> bool {
        true
    }
}
impl<'__daft> ::core::cmp::Eq for OnlyIgnoredChanges<'__daft> {}
impl<'__daft> ::daft::IntoChanges for OnlyIgnoredDiff<'__daft> {
    type Changes = OnlyIgnoredChanges<'__daft>;
    fn into_changes(self) -> ::core::option::Option<Self::Changes> {
        ::core::option::Option::None
    }
}
struct InnerDiff<'__daft> {
    value: <u32 as ::daft::Diffable>::Diff<'__daft>,
}
impl<'__daft> ::core::fmt::Debug for InnerDiff<'__daft>
where
    <u32 as ::daft::Diffable>::Diff<'__daft>: ::core::fmt::Debug,
{
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_struct(stringify!(InnerDiff))
            .field(stringify!(value), &self.value)
            .finish()
    }
}
impl<'__daft> ::core::cmp::PartialEq for InnerDiff<'__daft>
where
    <u32 as ::daft::Diffable>::Diff<'__daft>: ::core::cmp::PartialEq,
{
    fn eq(&self, other: &Self) -> bool {
        self.value == other.value
    }
}
impl<'__daft> ::core::cmp::Eq for InnerDiff<'__daft>
where
    <u32 as ::daft::Diffable>::Diff<'__daft>: ::core::cmp::Eq,
{}
impl ::daft::Diffable for Inner {
    type Diff<'__daft> = InnerDiff<'__daft> where Self: '__daft;
    fn diff<'__daft>(&'__daft self, other: &'__daft Self) -> InnerDiff<'__daft> {
        Self::Diff {
            value: ::daft::Diffable::diff(&self.value, &other.value),
        }
    }
}
struct WrapperDiff<'__daft> {
    inner: ::daft::Leaf<&'__daft Inner>,
    label: <String as ::daft::Diffable>::Diff<'__daft>,
}
impl<'__daft> ::core::fmt::Debug for WrapperDiff<'__daft>
where
    ::daft::Leaf<&'__daft Inner>: ::core::fmt::Debug,
    <String as ::daft::Diffable>::Diff<'__daft>: ::core::fmt::Debug,
{
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_struct(stringify!(WrapperDiff))
            .field(stringify!(inner), &self.inner)
            .field(stringify!(label), &self.label)
            .finish()
    }
}
impl<'__daft> ::core::cmp::PartialEq for WrapperDiff<'__daft>
where
    ::daft::Leaf<&'__daft Inner>: ::core::cmp::PartialEq,
    <String as ::daft::Diffable>::Diff<'__daft>: ::core::cmp::PartialEq,
{
    fn eq(&self, other: &Self) -> bool {
        self.inner == other.inner && self.label == other.label
    }
}
impl<'__daft> ::core::cmp::Eq for WrapperDiff<'__daft>
where
    ::daft::Leaf<&'__daft Inner>: ::core::cmp::Eq,
    <String as ::daft::Diffable>::Diff<'__daft>: ::core::cmp::Eq,
{}
impl ::daft::Diffable for Wrapper {
    type Diff<'__daft> = WrapperDiff<'__daft> where Self: '__daft;
    fn diff<'__daft>(&'__daft self, other: &'__daft Self) -> WrapperDiff<'__daft> {
        Self::Diff {
            inner: ::daft::Leaf {
                before: &self.inner,
                after: &other.inner,
            },
            label: ::daft::Diffable::diff(&self.label, &other.label),
        }
    }
}
struct WrapperChanges<'__daft>
where
    ::daft::Leaf<&'__daft Inner>: ::daft::IntoChanges,
    <String as ::daft::Diffable>::Diff<'__daft>: ::daft::IntoChanges,
{
    inner: ::core::option::Option<
        <::daft::Leaf<&'__daft Inner> as ::daft::IntoChanges>::Changes,
    >,
    label: ::core::option::Option<
        <<String as ::daft::Diffable>::Diff<'__daft> as ::daft::IntoChanges>::Changes,
    >,
}
impl<'__daft> ::core::fmt::Debug for WrapperChanges<'__daft>
where
    <::daft::Leaf<&'__daft Inner> as ::daft::IntoChanges>::Changes: ::core::fmt::Debug,
    <<String as ::daft::Diffable>::Diff<
        '__daft,
    > as ::daft::IntoChanges>::Changes: ::core::fmt::Debug,
{
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_struct(stringify!(WrapperChanges))
            .field(stringify!(inner), &self.inner)
            .field(stringify!(label), &self.label)
            .finish()
    }
}
impl<'__daft> ::core::cmp::PartialEq for WrapperChanges<'__daft>
where
    <::daft::Leaf<
        &'__daft Inner,
    > as ::daft::IntoChanges>::Changes: ::core::cmp::PartialEq,
    <<String as ::daft::Diffable>::Diff<
        '__daft,
    > as ::daft::IntoChanges>::Changes: ::core::cmp::PartialEq,
{
    fn eq(&self, other: &Self) -> bool {
        self.inner == other.inner && self.label == other.label
    }
}
impl<'__daft> ::core::cmp::Eq for WrapperChanges<'__daft>
where
    <::daft::Leaf<&'__daft Inner> as ::daft::IntoChanges>::Changes: ::core::cmp::Eq,
    <<String as ::daft::Diffable>::Diff<
        '__daft,
    > as ::daft::IntoChanges>::Changes: ::core::cmp::Eq,
{}
impl<'__daft> ::daft::IntoChanges for WrapperDiff<'__daft>
where
    ::daft::Leaf<&'__daft Inner>: ::daft::IntoChanges,
    <String as ::daft::Diffable>::Diff<'__daft>: ::daft::IntoChanges,
{
    type Changes = WrapperChanges<'__daft>;
    fn into_changes(self) -> ::core::option::Option<Self::Changes> {
        let __daft_inner = ::daft::IntoChanges::into_changes(self.inner);
        let __daft_label = ::daft::IntoChanges::into_changes(self.label);
        if __daft_inner.is_some() || __daft_label.is_some() {
            ::core::option::Option::Some(WrapperChanges {
                inner: __daft_inner,
                label: __daft_label,
            })
        } else {
            ::core::option::Option::None
        }
    }
}
#[automatically_derived]
impl<'__daft> ::daft::__private_serde::Serialize for WrapperChanges<'__daft>
where
    <::daft::Leaf<
        &'__daft Inner,
    > as ::daft::IntoChanges>::Changes: ::daft::__private_serde::Serialize,
    <<String as ::daft::Diffable>::Diff<
        '__daft,
    > as ::daft::IntoChanges>::Changes: ::daft::__private_serde::Serialize,
{
    fn serialize<__DaftS>(
        &self,
        serializer: __DaftS,
    ) -> ::core::result::Result<__DaftS::Ok, __DaftS::Error>
    where
        __DaftS: ::daft::__private_serde::Serializer,
    {
        let mut __state = ::daft::__private_serde::Serializer::serialize_struct(
            serializer,
            stringify!(WrapperChanges),
            {
                let mut __count = 0usize;
                if self.inner.is_some() {
                    __count += 1;
                }
                if self.label.is_some() {
                    __count += 1;
                }
                __count
            },
        )?;
        if let ::core::option::Option::Some(__value) = &self.inner {
            ::daft::__private_serde::ser::SerializeStruct::serialize_field(
                &mut __state,
                stringify!(inner),
                __value,
            )?;
        }
        if let ::core::option::Option::Some(__value) = &self.label {
            ::daft::__private_serde::ser::SerializeStruct::serialize_field(
                &mut __state,
                stringify!(label),
                __value,
            )?;
        }
        ::daft::__private_serde::ser::SerializeStruct::end(__state)
    }
}
