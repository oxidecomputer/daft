use super::error_store::{ErrorSink, ErrorStore};
use proc_macro2::{Span, TokenStream};
use quote::{quote, quote_spanned, ToTokens};
use syn::{
    parse_quote, parse_quote_spanned, parse_str, spanned::Spanned,
    visit::Visit, Attribute, Data, DataStruct, DeriveInput, Expr, Field,
    Fields, GenericParam, Generics, Index, Lifetime, LifetimeParam, Path,
    Token, WhereClause, WherePredicate,
};

pub struct DeriveDiffableOutput {
    pub out: Option<TokenStream>,
    pub errors: Vec<syn::Error>,
}

impl ToTokens for DeriveDiffableOutput {
    fn to_tokens(&self, tokens: &mut TokenStream) {
        tokens.extend(self.out.clone());
        tokens.extend(self.errors.iter().map(|error| error.to_compile_error()));
    }
}

pub fn derive_diffable(input: syn::DeriveInput) -> DeriveDiffableOutput {
    let mut error_store = ErrorStore::new();

    match &input.data {
        Data::Enum(_) => {
            // Implement all Enums as `Leaf`s
            let out = make_leaf(&input, AttrPosition::Enum, error_store.sink());
            DeriveDiffableOutput {
                out: Some(out),
                errors: error_store.into_inner(),
            }
        }
        Data::Struct(s) => {
            // This might be None if there are errors.
            let out = make_struct_impl(&input, s, error_store.sink());
            DeriveDiffableOutput { out, errors: error_store.into_inner() }
        }

        Data::Union(_) => {
            // Implement all unions as `Leaf`s
            let out =
                make_leaf(&input, AttrPosition::Union, error_store.sink());
            DeriveDiffableOutput {
                out: Some(out),
                errors: error_store.into_inner(),
            }
        }
    }
}

// TODO: allow the crate name to be passed in as a macro argument
fn daft_crate() -> Path {
    parse_quote! { ::daft }
}

fn daft_lifetime() -> LifetimeParam {
    // Use an underscore to avoid clashing with a user-defined `'daft` lifetime.
    LifetimeParam::new(Lifetime::new("'__daft", Span::call_site()))
}

// We need to add our lifetime parameter 'daft and ensure any other parameters
// live as long as `daft`
fn add_lifetime_to_generics(
    input: &DeriveInput,
    daft_lt: &LifetimeParam,
) -> Generics {
    let mut new_generics = input.generics.clone();
    new_generics
        .lifetimes_mut()
        .for_each(|lt| lt.bounds.push(daft_lt.lifetime.clone()));
    new_generics.type_params_mut().for_each(|lt| {
        lt.bounds.push(syn::TypeParamBound::Lifetime(daft_lt.lifetime.clone()))
    });

    // Add the 'daft lifetime to the beginning of the parameter list -- the
    // exact order is not hugely important, but doing this makes tests simpler
    // (they can just check the first element).
    new_generics.params.insert(0, GenericParam::from(daft_lt.clone()));
    new_generics
}

// Implement `Diffable` as a `Leaf`.
fn make_leaf(
    input: &DeriveInput,
    position: AttrPosition,
    errors: ErrorSink<'_, syn::Error>,
) -> TokenStream {
    // The input should not have any daft attributes.
    for attr in &input.attrs {
        if attr.path().is_ident("daft") {
            let res = attr.parse_nested_meta(|meta| {
                if meta.path.is_ident("leaf") {
                    // Accept this for leaf structs, but not for anything else.
                    if position == AttrPosition::LeafStruct {
                        return Ok(());
                    }

                    errors.push_critical(meta.error(format!(
                        "this is unnecessary: the Diffable \
                         implementation {} is always a leaf",
                        position.as_purpose_str(),
                    )));
                } else {
                    errors.push_critical(meta.error(format!(
                        "daft attributes are not allowed {}",
                        position.as_locative_str(),
                    )));
                }

                Ok(())
            });
            if let Err(err) = res {
                errors.push_critical(err);
            }
        }
    }

    // Variants should not have any daft attributes.
    let mut v = BanDaftAttrsVisitor { position, errors: errors.new_child() };
    v.visit_data(&input.data);

    // Even though errors might have occurred above, we *do* generate the
    // implementation. That allows rust-analyzer to still understand that the
    // `Diffable` impl exists.

    let ident = &input.ident;
    let daft_crate = daft_crate();
    let daft_lt = daft_lifetime();

    // The "where Self: #daft_lt" condition appears to be enough to satisfy
    // Rust's borrow checker, so we don't need to add further constraints via
    // `add_lifetime_to_generics`.
    let (impl_gen, ty_gen, where_clause) = &input.generics.split_for_impl();

    quote! {
        impl #impl_gen #daft_crate::Diffable for #ident #ty_gen #where_clause
        {
            type Diff<#daft_lt> = #daft_crate::Leaf<&#daft_lt Self> where Self: #daft_lt;

            fn diff<#daft_lt>(&#daft_lt self, other: &#daft_lt Self) -> Self::Diff<#daft_lt> {
                #daft_crate::Leaf {before: self, after: other}
            }
        }
    }
}

struct BanDaftAttrsVisitor<'a> {
    position: AttrPosition,
    errors: ErrorSink<'a, syn::Error>,
}

impl Visit<'_> for BanDaftAttrsVisitor<'_> {
    fn visit_attribute(&mut self, attr: &Attribute) {
        if attr.path().is_ident("daft") {
            self.errors.push_critical(syn::Error::new_spanned(
                attr,
                format!(
                    "daft attributes are not allowed {}",
                    self.position.as_locative_str(),
                ),
            ));
        }
    }

    fn visit_variant(&mut self, v: &syn::Variant) {
        let old_position = self.position;
        self.position = self.position.visit_variant();
        syn::visit::visit_variant(self, v);
        self.position = old_position;
    }

    fn visit_field(&mut self, f: &syn::Field) {
        let old_position = self.position;
        self.position = self.position.visit_field();
        syn::visit::visit_field(self, f);
        self.position = old_position;
    }
}

#[derive(Clone, Copy, Debug, PartialEq, Eq)]
enum AttrPosition {
    // Catch-all in case something unexpected happens with the visitor.
    General,
    LeafStruct,
    LeafStructField,
    Enum,
    Variant,
    VariantField,
    Union,
    UnionField,
}

impl AttrPosition {
    fn visit_variant(self) -> Self {
        match self {
            Self::Enum => Self::Variant,
            Self::General
            | Self::LeafStruct
            | Self::LeafStructField
            | Self::Variant
            | Self::VariantField
            | Self::Union
            | Self::UnionField => Self::General,
        }
    }

    fn visit_field(self) -> Self {
        match self {
            Self::LeafStruct => Self::LeafStructField,
            Self::Variant => Self::VariantField,
            Self::Union => Self::UnionField,
            Self::General
            | Self::LeafStructField
            | Self::Enum
            | Self::VariantField
            | Self::UnionField => Self::General,
        }
    }

    // purpose = prepositional phrase to indicate what something applies to,
    // e.g. "the implementation for enums is always"
    fn as_purpose_str(self) -> &'static str {
        match self {
            Self::General => "for this type",
            Self::LeafStruct => "for structs annotated with #[daft(leaf)]",
            Self::LeafStructField => {
                "for fields on structs annotated with #[daft(leaf)]"
            }
            Self::Enum => "for enums",
            Self::Variant => "for enum variants",
            Self::VariantField => "for enum variant fields",
            Self::Union => "for unions",
            Self::UnionField => "for union fields",
        }
    }

    // "locative" = indicating location: "not allowed on enums", etc.
    fn as_locative_str(self) -> &'static str {
        match self {
            Self::General => "here",
            Self::LeafStruct => "on structs annotated with #[daft(leaf)]",
            Self::LeafStructField => {
                "on fields of structs annotated with #[daft(leaf)]"
            }
            Self::Enum => "on enums",
            Self::Variant => "on enum variants",
            Self::VariantField => "on enum variant fields",
            Self::Union => "on unions",
            Self::UnionField => "on union fields",
        }
    }
}

fn make_struct_impl(
    input: &DeriveInput,
    s: &DataStruct,
    errors: ErrorSink<'_, syn::Error>,
) -> Option<TokenStream> {
    let Some(struct_config) =
        StructConfig::parse_from(&input.attrs, errors.new_child())
    else {
        // An error occurred parsing the struct configuration -- don't generate
        // anything.
        return None;
    };

    match struct_config.mode {
        StructMode::Default => make_diff_struct(input, s, errors.new_child())
            .map(|(generated_struct, diff_fields)| {
                let diff_impl = make_diff_impl(input, &diff_fields);
                let changes_items = if struct_config.changes {
                    make_changes_items(input, s, &diff_fields)
                } else {
                    TokenStream::new()
                };
                // Uncomment for some debugging
                // eprintln!("{generated_struct}");
                // eprintln!("{diff_impl}");
                // eprintln!("{changes_items}");
                quote! {
                    #generated_struct
                    #diff_impl
                    #changes_items
                }
            }),
        StructMode::Leaf => {
            Some(make_leaf(input, AttrPosition::LeafStruct, errors.new_child()))
        }
    }
}

/// Emit the `*Changes` struct, its inherent impls, the `IntoChanges` impl
/// on the diff, and — when the `serde` feature is on — a hand-written
/// `Serialize` impl that skips `None` fields.
fn make_changes_items(
    input: &DeriveInput,
    s: &DataStruct,
    diff_fields: &DiffFields,
) -> TokenStream {
    let changes_struct = make_changes_struct(input, s, diff_fields);
    let into_changes_impl = make_into_changes_impl(input, s, diff_fields);
    #[cfg(feature = "serde")]
    let serialize_impl = make_serialize_impl(input, diff_fields);
    #[cfg(not(feature = "serde"))]
    let serialize_impl = TokenStream::new();
    quote! {
        #changes_struct
        #into_changes_impl
        #serialize_impl
    }
}

/// Create the `Diff` struct
fn make_diff_struct(
    input: &DeriveInput,
    s: &DataStruct,
    errors: ErrorSink<'_, syn::Error>,
) -> Option<(TokenStream, DiffFields)> {
    // The name of the original type
    let vis = &input.vis;

    // The name of the generated type
    let name = parse_str::<Path>(&format!("{}Diff", input.ident)).unwrap();

    // Copy over the non-exhaustive attribute from the original struct. (Do we
    // need to copy over other attributes?)
    let non_exhaustive =
        input.attrs.iter().find(|attr| attr.path().is_ident("non_exhaustive"));

    let daft_lt = daft_lifetime();

    // We are creating a new type, so use only generics with our new lifetime
    // and bounds.
    //
    // Most of the other generics users use `split_for_impl`, but that is geared
    // specifically for trait implementations, not type definitions. For type
    // definitions, we use the original `Generics`.
    //
    // The `ToTokens` implementation for `Generics` does not print the `where`
    // clause, so we also include that separately.
    let new_generics = add_lifetime_to_generics(input, &daft_lt);
    let where_clause = &new_generics.where_clause;

    let Some(diff_fields) =
        DiffFields::new(&s.fields, where_clause.as_ref(), errors.new_child())
    else {
        // An error occurred parsing fields -- don't generate the diff struct.
        return None;
    };

    // --- No more errors past this point ---

    // If the diff struct would otherwise be empty, inject a private
    // `PhantomData` field that uses both `'__daft` and the original generics.
    // Without it, those parameters would be declared on the diff struct but
    // unused.
    //
    // We use `fn() -> &'__daft Self`, not `&'__daft Self` an empty diff has no
    // real data, so making it Send/Sync independent of the original type's
    // auto-traits is the best choice. `fn() -> &'__daft Self` is covariant in
    // `'__daft` and the original generics, and always `Send + Sync`.
    let phantom_ty = {
        let ident = &input.ident;
        let (_, orig_ty_gen, _) = input.generics.split_for_impl();
        quote! {
            ::core::marker::PhantomData<fn() -> &#daft_lt #ident #orig_ty_gen>
        }
    };

    let struct_def = if diff_fields.fields.is_empty() {
        match &s.fields {
            Fields::Named(_) | Fields::Unit => quote! {
                #non_exhaustive
                #vis struct #name #new_generics #where_clause {
                    _phantom: #phantom_ty,
                }
            },
            Fields::Unnamed(_) => quote! {
                #non_exhaustive
                #vis struct #name #new_generics (#phantom_ty) #where_clause;
            },
        }
    } else {
        match &s.fields {
            Fields::Named(_) => quote! {
                #non_exhaustive
                #vis struct #name #new_generics #where_clause #diff_fields
            },
            Fields::Unnamed(_) => quote! {
                #non_exhaustive
                #vis struct #name #new_generics #diff_fields #where_clause;
            },
            Fields::Unit => unreachable!(
                "Fields::Unit always produces an empty diff struct"
            ),
        }
    };

    // We can't `#[derive]` Debug/PartialEq/Eq here because the rustc-emitted
    // bounds would apply to the original type parameters; we need bounds on
    // the projected `<T as Diffable>::Diff<'__daft>` types instead.
    let trait_impls = make_projected_trait_impls(
        &name,
        &new_generics,
        &diff_fields.fields,
        non_exhaustive.is_some(),
        |bound| diff_fields.where_clause_with_trait_bound(bound),
    );

    Some((quote! { #struct_def #trait_impls }, diff_fields))
}

/// Emit `Debug`, `PartialEq`, and `Eq` impls for a generated `*Diff` or
/// `*Changes` struct.
///
/// The two callers differ only in how field-type bounds are projected onto
/// the where clause — the diff impls bound `<T as Diffable>::Diff<'__daft>`
/// while the changes impls bound `<<T> as IntoChanges>::Changes`. The
/// `where_clause_for` closure encapsulates that difference; everything else
/// (debug body shape, partial-eq fold, non-exhaustive finish) is shared.
fn make_projected_trait_impls(
    name: &Path,
    generics: &Generics,
    fields: &Fields,
    non_exhaustive: bool,
    mut where_clause_for: impl FnMut(&syn::TraitBound) -> WhereClause,
) -> TokenStream {
    let (impl_gen, ty_gen, _) = generics.split_for_impl();
    let finish = if non_exhaustive {
        quote! { .finish_non_exhaustive() }
    } else {
        quote! { .finish() }
    };

    let debug_impl = {
        let where_clause =
            where_clause_for(&parse_quote! { ::core::fmt::Debug });
        let members = fields.members();
        let body = match fields {
            Fields::Named(_) => quote! {
                f.debug_struct(stringify!(#name))
                #( .field(stringify!(#members), &self.#members) )*
                #finish
            },
            Fields::Unnamed(_) => quote! {
                f.debug_tuple(stringify!(#name))
                #( .field(&self.#members) )*
                #finish
            },
            Fields::Unit => quote! {
                f.debug_struct(stringify!(#name)) #finish
            },
        };
        quote! {
            impl #impl_gen ::core::fmt::Debug for #name #ty_gen #where_clause {
                fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
                    #body
                }
            }
        }
    };

    let partial_eq_impl = {
        let where_clause =
            where_clause_for(&parse_quote! { ::core::cmp::PartialEq });
        let members = fields.members();
        let body: Expr = if fields.is_empty() {
            parse_quote! { true }
        } else {
            parse_quote! { #(self.#members == other.#members) && * }
        };
        quote! {
            impl #impl_gen ::core::cmp::PartialEq for #name #ty_gen #where_clause {
                fn eq(&self, other: &Self) -> bool { #body }
            }
        }
    };

    let eq_impl = {
        let where_clause = where_clause_for(&parse_quote! { ::core::cmp::Eq });
        quote! {
            impl #impl_gen ::core::cmp::Eq for #name #ty_gen #where_clause {}
        }
    };

    quote! { #debug_impl #partial_eq_impl #eq_impl }
}

/// Impl `Diffable` for the original struct
fn make_diff_impl(
    input: &DeriveInput,
    diff_fields: &DiffFields,
) -> TokenStream {
    // The name of the original type
    let ident = &input.ident;

    // The name of the generated type
    let name = parse_str::<Path>(&format!("{}Diff", input.ident)).unwrap();
    let diffs =
        generate_field_diffs(&diff_fields.fields, &diff_fields.field_configs);

    let daft_crate = daft_crate();
    let daft_lt = daft_lifetime();
    let new_generics = add_lifetime_to_generics(input, &daft_lt);

    let (impl_gen, ty_gen, _) = &input.generics.split_for_impl();
    let (_, new_ty_gen, where_clause) = &new_generics.split_for_impl();

    let constructor = if diff_fields.fields.is_empty() {
        match &diff_fields.fields {
            Fields::Named(_) | Fields::Unit => quote! {
                Self::Diff { _phantom: ::core::marker::PhantomData }
            },
            Fields::Unnamed(_) => quote! {
                Self::Diff { 0: ::core::marker::PhantomData }
            },
        }
    } else {
        quote! {
            Self::Diff {
                #diffs
            }
        }
    };

    quote! {
        impl #impl_gen #daft_crate::Diffable for #ident #ty_gen
            #where_clause
        {
            type Diff<#daft_lt> = #name #new_ty_gen where Self: #daft_lt;

            fn diff<#daft_lt>(&#daft_lt self, other: &#daft_lt Self) -> #name #new_ty_gen {
                #constructor
            }
        }
    }
}

/// Emit the `*Changes` struct definition and its `Debug`/`PartialEq`/`Eq`
/// impls. Each field is wrapped in `Option<...>` so unchanged subtrees can
/// be represented as `None`.
fn make_changes_struct(
    input: &DeriveInput,
    s: &DataStruct,
    diff_fields: &DiffFields,
) -> TokenStream {
    let vis = &input.vis;
    let name = parse_str::<Path>(&format!("{}Changes", input.ident)).unwrap();
    let non_exhaustive =
        input.attrs.iter().find(|attr| attr.path().is_ident("non_exhaustive"));

    let daft_lt = daft_lifetime();
    let daft_crate = daft_crate();
    let new_generics = add_lifetime_to_generics(input, &daft_lt);
    let where_clause = diff_fields.where_clause_with_trait_bound(
        &parse_quote! { #daft_crate::IntoChanges },
    );

    let phantom_ty = {
        let ident = &input.ident;
        let (_, orig_ty_gen, _) = input.generics.split_for_impl();
        quote! {
            ::core::marker::PhantomData<fn() -> &#daft_lt #ident #orig_ty_gen>
        }
    };

    let changes_field_ty = |ty: &syn::Type| -> TokenStream {
        quote_spanned! {ty.span()=>
            ::core::option::Option<<#ty as #daft_crate::IntoChanges>::Changes>
        }
    };

    // The struct *shape* (named/unnamed/unit) comes from `s.fields`, but
    // field types come from `diff_fields.fields` since those carry the
    // projected `<T as Diffable>::Diff<'__daft>` form.
    let struct_def = if diff_fields.fields.is_empty() {
        match &s.fields {
            Fields::Named(_) | Fields::Unit => quote! {
                #non_exhaustive
                #vis struct #name #new_generics #where_clause {
                    _phantom: #phantom_ty,
                }
            },
            Fields::Unnamed(_) => quote! {
                #non_exhaustive
                #vis struct #name #new_generics (#phantom_ty) #where_clause;
            },
        }
    } else {
        match &diff_fields.fields {
            Fields::Named(fields) => {
                let entries = fields.named.iter().map(|f| {
                    let field_vis = &f.vis;
                    let field_name = f.ident.as_ref().unwrap();
                    let ty = changes_field_ty(&f.ty);
                    quote_spanned! {f.span()=>
                        #field_vis #field_name: #ty,
                    }
                });
                quote! {
                    #non_exhaustive
                    #vis struct #name #new_generics #where_clause {
                        #(#entries)*
                    }
                }
            }
            Fields::Unnamed(fields) => {
                let entries = fields.unnamed.iter().map(|f| {
                    let field_vis = &f.vis;
                    let ty = changes_field_ty(&f.ty);
                    quote_spanned! {f.span()=>
                        #field_vis #ty
                    }
                });
                quote! {
                    #non_exhaustive
                    #vis struct #name #new_generics (#(#entries),*) #where_clause;
                }
            }
            Fields::Unit => unreachable!(
                "Fields::Unit always produces an empty changes struct"
            ),
        }
    };

    let trait_impls = make_projected_trait_impls(
        &name,
        &new_generics,
        &diff_fields.fields,
        non_exhaustive.is_some(),
        |bound| diff_fields.changes_where_clause_with_trait_bound(bound),
    );

    quote! { #struct_def #trait_impls }
}

/// Implement `IntoChanges` on the generated `*Diff` struct. Projects every
/// field to its changes-only representation and returns `None` if every
/// projection was itself `None` (no leaf changed).
fn make_into_changes_impl(
    input: &DeriveInput,
    s: &DataStruct,
    diff_fields: &DiffFields,
) -> TokenStream {
    let ident = &input.ident;
    let diff_name = parse_str::<Path>(&format!("{ident}Diff")).unwrap();
    let changes_name = parse_str::<Path>(&format!("{ident}Changes")).unwrap();
    let daft_crate = daft_crate();
    let daft_lt = daft_lifetime();
    let new_generics = add_lifetime_to_generics(input, &daft_lt);
    let (impl_gen, ty_gen, _) = &new_generics.split_for_impl();
    let where_clause = diff_fields.where_clause_with_trait_bound(
        &parse_quote! { #daft_crate::IntoChanges },
    );

    if diff_fields.fields.is_empty() {
        return quote! {
            impl #impl_gen #daft_crate::IntoChanges for #diff_name #ty_gen
                #where_clause
            {
                type Changes = #changes_name #ty_gen;

                fn into_changes(self) -> ::core::option::Option<Self::Changes> {
                    ::core::option::Option::None
                }
            }
        };
    }

    // One `__daft_<ident_or_index>` binding per field so the construction
    // step below can name them positionally even for tuple structs.
    let bindings: Vec<TokenStream> = diff_fields
        .fields
        .iter()
        .enumerate()
        .map(|(i, f)| {
            let binding = binding_ident(f, i);
            let access = match &f.ident {
                Some(ident) => quote! { self.#ident },
                None => {
                    let idx: Index = i.into();
                    quote! { self.#idx }
                }
            };
            quote_spanned! {f.span()=>
                let #binding = #daft_crate::IntoChanges::into_changes(#access);
            }
        })
        .collect();

    let binding_names: Vec<syn::Ident> = diff_fields
        .fields
        .iter()
        .enumerate()
        .map(|(i, f)| binding_ident(f, i))
        .collect();

    let constructor = match &s.fields {
        Fields::Named(_) => {
            let entries = diff_fields.fields.iter().zip(&binding_names).map(
                |(f, binding)| {
                    let name = f.ident.as_ref().unwrap();
                    quote! { #name: #binding }
                },
            );
            quote! { #changes_name { #(#entries),* } }
        }
        Fields::Unnamed(_) => {
            quote! { #changes_name(#(#binding_names),*) }
        }
        Fields::Unit => unreachable!(
            "Fields::Unit handled in the empty-fields branch above"
        ),
    };

    quote! {
        impl #impl_gen #daft_crate::IntoChanges for #diff_name #ty_gen
            #where_clause
        {
            type Changes = #changes_name #ty_gen;

            fn into_changes(self) -> ::core::option::Option<Self::Changes> {
                #(#bindings)*
                if #(#binding_names.is_some())||* {
                    ::core::option::Option::Some(#constructor)
                } else {
                    ::core::option::Option::None
                }
            }
        }
    }
}

/// Build a `__daft_<member>` binding identifier. The prefix keeps tuple
/// indices valid as idents and avoids colliding with user field names.
fn binding_ident(f: &Field, index: usize) -> syn::Ident {
    match &f.ident {
        Some(ident) => {
            syn::Ident::new(&format!("__daft_{ident}"), ident.span())
        }
        None => syn::Ident::new(&format!("__daft_{index}"), f.span()),
    }
}

/// Implement `serde::Serialize` on the generated `*Changes` struct.
///
/// Hand-written instead of `#[derive(Serialize)]` because serde's
/// auto-bound generator cannot follow projected associated types like
/// `<<T as Diffable>::Diff<'__daft> as IntoChanges>::Changes`, and the
/// derived impl fails to type-check. We pair an explicit where clause with
/// per-field `if let Some(...)` so unchanged subtrees serialize to nothing.
///
/// This is never called for empty `*Changes` because `into_changes` on the
/// corresponding `*Diff` returns `None` in that case — no value of the
/// empty struct ever reaches `Serialize`.
#[cfg(feature = "serde")]
fn make_serialize_impl(
    input: &DeriveInput,
    diff_fields: &DiffFields,
) -> TokenStream {
    if diff_fields.fields.is_empty() {
        return TokenStream::new();
    }

    let name = parse_str::<Path>(&format!("{}Changes", input.ident)).unwrap();
    let daft_crate = daft_crate();
    let daft_lt = daft_lifetime();
    let new_generics = add_lifetime_to_generics(input, &daft_lt);
    let (impl_gen, ty_gen, _) = &new_generics.split_for_impl();

    let where_clause = diff_fields.changes_where_clause_with_trait_bound(
        &parse_quote! { #daft_crate::__private_serde::Serialize },
    );

    let members: Vec<_> = diff_fields.fields.members().collect();
    let is_tuple = matches!(&diff_fields.fields, Fields::Unnamed(_));

    let count_expr = quote! {{
        let mut __count = 0usize;
        #( if self.#members.is_some() { __count += 1; } )*
        __count
    }};

    let (begin, end_trait) = if is_tuple {
        (
            quote! {
                #daft_crate::__private_serde::Serializer::serialize_tuple_struct(
                    serializer, stringify!(#name), #count_expr,
                )?
            },
            quote! { #daft_crate::__private_serde::ser::SerializeTupleStruct },
        )
    } else {
        (
            quote! {
                #daft_crate::__private_serde::Serializer::serialize_struct(
                    serializer, stringify!(#name), #count_expr,
                )?
            },
            quote! { #daft_crate::__private_serde::ser::SerializeStruct },
        )
    };

    let serialize_fields = members.iter().enumerate().map(|(i, member)| {
        let call = if is_tuple {
            quote! { #end_trait::serialize_field(&mut __state, __value)? }
        } else {
            // Named-field key matches the derived struct's field name.
            let key = match member {
                syn::Member::Named(id) => quote! { stringify!(#id) },
                syn::Member::Unnamed(_) => {
                    let s = i.to_string();
                    quote! { #s }
                }
            };
            quote! { #end_trait::serialize_field(&mut __state, #key, __value)? }
        };
        quote! {
            if let ::core::option::Option::Some(__value) = &self.#member {
                #call;
            }
        }
    });

    quote! {
        #[automatically_derived]
        impl #impl_gen #daft_crate::__private_serde::Serialize for #name #ty_gen
            #where_clause
        {
            fn serialize<__DaftS>(
                &self,
                serializer: __DaftS,
            ) -> ::core::result::Result<__DaftS::Ok, __DaftS::Error>
            where
                __DaftS: #daft_crate::__private_serde::Serializer,
            {
                let mut __state = #begin;
                #( #serialize_fields )*
                #end_trait::end(__state)
            }
        }
    }
}

/// For a `Diff` struct generated by this derive macro, tracks the fields that
/// will be put into that struct.
///
/// This also tracks the `where` clause.
///
/// The goal of this wrapper is to provide helpers to iterate over the fields
/// and members.
struct DiffFields {
    fields: Fields,
    // Configuration for each field -- a vector with the same length as `self.fields`.
    field_configs: Vec<FieldConfig>,
    // The base where clause for the diff struct.
    where_clause: WhereClause,
}

impl DiffFields {
    /// None means there was an error parsing a config.
    fn new(
        fields: &Fields,
        where_clause: Option<&WhereClause>,
        errors: ErrorSink<'_, syn::Error>,
    ) -> Option<Self> {
        let (fields, field_configs) = match fields {
            Fields::Named(fields) => {
                let (named, configs) = fields
                    .named
                    .iter()
                    .filter_map(|field| {
                        Self::diff_field(field, errors.new_child())
                    })
                    .unzip();
                (
                    Fields::Named(syn::FieldsNamed {
                        brace_token: fields.brace_token,
                        named,
                    }),
                    configs,
                )
            }
            Fields::Unnamed(fields) => {
                let (unnamed, configs) = fields
                    .unnamed
                    .iter()
                    .filter_map(|field| {
                        Self::diff_field(field, errors.new_child())
                    })
                    .unzip();
                (
                    Fields::Unnamed(syn::FieldsUnnamed {
                        paren_token: fields.paren_token,
                        unnamed,
                    }),
                    configs,
                )
            }
            Fields::Unit => (Fields::Unit, Vec::new()),
        };

        // Initialize an empty where clause if none was provided.
        let where_clause =
            where_clause.cloned().unwrap_or_else(|| WhereClause {
                where_token: <Token![where]>::default(),
                predicates: Default::default(),
            });

        if errors.has_critical_errors() {
            None
        } else {
            Some(Self { fields, field_configs, where_clause })
        }
    }

    /// Return a field for a diff with the appropriate type.
    ///
    /// If the type is ignored, or if there's an error parsing configuration,
    /// return None.
    fn diff_field(
        f: &Field,
        errors: ErrorSink<'_, syn::Error>,
    ) -> Option<(Field, FieldConfig)> {
        let Some(config) =
            FieldConfig::parse_from(&f.attrs, errors.new_child())
        else {
            // None means there's an error parsing a config -- return None here,
            // we'll emit errors at the top level.
            return None;
        };
        if config.mode == FieldMode::Ignore {
            // Skip over this field if there's an ignore.
            return None;
        }

        // Always use the daft lifetime for the diff -- associations between the
        // daft lifetime and existing parameters (both lifetime and type
        // parameters) are created in `add_lifetime_to_generics`, e.g. `'a:
        // '__daft`, or `T: '__daft`.
        let lt = daft_lifetime();
        let daft_crate = daft_crate();
        let ty = &f.ty;
        let mut f = f.clone();

        f.ty = if config.mode == FieldMode::Leaf {
            parse_quote_spanned! {f.span()=>
                #daft_crate::Leaf<&#lt #ty>
            }
        } else {
            parse_quote_spanned! {f.span()=>
                <#ty as #daft_crate::Diffable>::Diff<#lt>
            }
        };

        // Drop all attributes for now. We may want to carry some over in the
        // future.
        f.attrs = vec![];

        Some((f, config))
    }

    /// Returns an iterator over field types.
    fn types(&self) -> impl Iterator<Item = &syn::Type> {
        self.fields.iter().map(|f| &f.ty)
    }

    /// Returns an expanded where clause where the fields have had a trait bound
    /// applied to them.
    fn where_clause_with_trait_bound(
        &self,
        trait_bound: &syn::TraitBound,
    ) -> WhereClause {
        let predicates = self.types().map(|ty| -> WherePredicate {
            parse_quote_spanned! {ty.span()=>
                #ty: #trait_bound
            }
        });

        let mut where_clause = self.where_clause.clone();
        where_clause.predicates.extend(predicates);

        where_clause
    }

    /// Returns an iterator over the *changes*-projected field types — i.e.
    /// `<DiffType as ::daft::IntoChanges>::Changes` for each field. Used by
    /// the Changes-side struct definition and its inherent trait impls.
    fn changes_types(&self) -> impl Iterator<Item = syn::Type> + '_ {
        let daft_crate = daft_crate();
        self.types().map(move |ty| -> syn::Type {
            parse_quote_spanned! {ty.span()=>
                <#ty as #daft_crate::IntoChanges>::Changes
            }
        })
    }

    /// Like [`Self::where_clause_with_trait_bound`], but each predicate is
    /// applied to the corresponding changes-projected type rather than the
    /// diff type itself.
    fn changes_where_clause_with_trait_bound(
        &self,
        trait_bound: &syn::TraitBound,
    ) -> WhereClause {
        let predicates = self.changes_types().map(|ty| -> WherePredicate {
            parse_quote_spanned! {ty.span()=>
                #ty: #trait_bound
            }
        });

        let mut where_clause = self.where_clause.clone();
        where_clause.predicates.extend(predicates);

        where_clause
    }
}

impl ToTokens for DiffFields {
    fn to_tokens(&self, tokens: &mut TokenStream) {
        self.fields.to_tokens(tokens);
    }
}

/// Generate a call to `diff` for each field of the original struct that isn't
/// ignored.
fn generate_field_diffs(
    fields: &Fields,
    // Should be the same length as `fields`.
    field_configs: &[FieldConfig],
) -> TokenStream {
    let daft_crate = daft_crate();
    let field_diffs =
        fields.iter().zip(field_configs).enumerate().map(|(i, (f, config))| {
            let field_name = match &f.ident {
                Some(ident) => quote! { #ident },
                None => {
                    let ident: Index = i.into();
                    quote! { #ident }
                }
            };
            if config.mode == FieldMode::Leaf {
                quote_spanned! {f.span()=>
                    #field_name: #daft_crate::Leaf {
                        before: &self.#field_name,
                        after: &other.#field_name
                    }
                }
            } else {
                quote_spanned! {f.span()=>
                    #field_name: #daft_crate::Diffable::diff(
                        &self.#field_name,
                        &other.#field_name
                    )
                }
            }
        });
    quote! { #(#field_diffs),* }
}

#[derive(Debug)]
struct StructConfig {
    mode: StructMode,
    /// `#[daft(changes)]`: opt in to emitting the `*Changes` struct, the
    /// `IntoChanges` impl, and (with the `serde` feature) a `Serialize` impl
    /// on the changes type. Off by default to keep the derive's contract
    /// minimal — users with non-`Eq` fields or unbounded type parameters can
    /// continue to derive `Diffable` without picking up extra constraints.
    changes: bool,
}

impl StructConfig {
    fn parse_from(
        attrs: &[Attribute],
        errors: ErrorSink<'_, syn::Error>,
    ) -> Option<Self> {
        let mut mode = StructMode::Default;
        let mut changes = false;

        for attr in attrs {
            {
                if attr.path().is_ident("daft") {
                    let res = attr.parse_nested_meta(|meta| {
                        if meta.path.is_ident("leaf") {
                            match mode {
                                StructMode::Default => {
                                    mode = StructMode::Leaf;
                                }
                                StructMode::Leaf => {
                                    errors.push_warning(meta.error(
                                    "#[daft(leaf)] specified multiple times",
                                ));
                                }
                            }
                        } else if meta.path.is_ident("changes") {
                            if changes {
                                errors.push_warning(meta.error(
                                    "#[daft(changes)] specified multiple times",
                                ));
                            }
                            changes = true;
                        } else {
                            errors.push_critical(meta.error(
                                "unknown attribute \
                                 (supported attributes: leaf, changes)",
                            ));
                        }

                        Ok(())
                    });

                    if let Err(err) = res {
                        errors.push_critical(err);
                    }
                }
            }
        }

        // `leaf` already makes the diff a `Leaf`, which has its own
        // `IntoChanges` impl. Combining the two attributes is a user error.
        if changes && matches!(mode, StructMode::Leaf) {
            let span = attrs
                .iter()
                .find(|a| a.path().is_ident("daft"))
                .map(|a| a.to_token_stream())
                .unwrap_or_default();
            errors.push_critical(syn::Error::new_spanned(
                span,
                "#[daft(changes)] is redundant on `#[daft(leaf)]` structs \
                 (their `Leaf` diff already implements `IntoChanges`)",
            ));
        }

        if errors.has_critical_errors() {
            None
        } else {
            Some(Self { mode, changes })
        }
    }
}

#[derive(Clone, Copy, Debug, PartialEq, Eq)]
enum StructMode {
    // The default mode: do a recursive diff for this struct.
    Default,
    // Use a `Leaf` for this struct.
    Leaf,
}

#[derive(Debug)]
struct FieldConfig {
    mode: FieldMode,
}

impl FieldConfig {
    fn parse_from(
        attrs: &[Attribute],
        errors: ErrorSink<'_, syn::Error>,
    ) -> Option<Self> {
        let mut mode = FieldMode::Default;

        for attr in attrs {
            if attr.path().is_ident("daft") {
                let res = attr.parse_nested_meta(|meta| {
                    if meta.path.is_ident("leaf") {
                        // #[daft(leaf)]
                        match mode {
                            FieldMode::Default => {
                                mode = FieldMode::Leaf;
                            }
                            FieldMode::Leaf => {
                                errors.push_warning(meta.error(
                                    "#[daft(leaf)] specified multiple times",
                                ));
                            }
                            _ => {
                                errors.push_critical(meta.error(
                                    "#[daft(leaf)] conflicts with \
                                     other attributes",
                                ));
                            }
                        }
                    } else if meta.path.is_ident("ignore") {
                        // #[daft(ignore)]
                        match mode {
                            FieldMode::Default => {
                                mode = FieldMode::Ignore;
                            }
                            FieldMode::Ignore => {
                                errors.push_warning(meta.error(
                                    "#[daft(ignore)] specified multiple times",
                                ));
                            }
                            _ => {
                                errors.push_critical(meta.error(
                                    "#[daft(ignore)] conflicts with \
                                     other attributes",
                                ));
                            }
                        }
                    } else {
                        errors.push_critical(meta.error(
                            "unknown attribute \
                             (supported attributes: leaf, ignore)",
                        ));
                    }

                    Ok(())
                });
                // We don't return an error from our callback, but syn might.
                if let Err(err) = res {
                    errors.push_critical(err);
                }
            }
        }

        if errors.has_critical_errors() {
            None
        } else {
            Some(Self { mode })
        }
    }
}

#[derive(Clone, Copy, Debug, PartialEq, Eq)]
enum FieldMode {
    // The default mode: do a recursive diff for this field.
    Default,
    // Use a `Leaf` for this field.
    Leaf,
    // Ignore this field.
    Ignore,
}
