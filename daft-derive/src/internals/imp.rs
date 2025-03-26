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

                    errors.push(meta.error(format!(
                        "this is unnecessary: the Diffable \
                         implementation {} is always a leaf",
                        position.as_purpose_str(),
                    )));
                } else {
                    errors.push(meta.error(format!(
                        "daft attributes are not allowed {}",
                        position.as_locative_str(),
                    )));
                }

                Ok(())
            });
            if let Err(err) = res {
                errors.push(err);
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
            self.errors.push(syn::Error::new_spanned(
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
                // Uncomment for some debugging
                // eprintln!("{generated_struct}");
                // eprintln!("{diff_impl}");
                quote! {
                    #generated_struct
                    #diff_impl
                }
            }),
        StructMode::Leaf => {
            Some(make_leaf(input, AttrPosition::LeafStruct, errors.new_child()))
        }
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

    let struct_def = match &s.fields {
        Fields::Named(_) => quote! {
            #non_exhaustive
            #vis struct #name #new_generics #where_clause #diff_fields

        },
        Fields::Unnamed(_) => quote! {
            #non_exhaustive
            #vis struct #name #new_generics #diff_fields #where_clause;
        },
        Fields::Unit => quote! {
            // This is kinda silly
            #non_exhaustive
            #vis struct #name #new_generics {} #where_clause
        },
    };

    // Generate PartialEq, Eq, and Debug implementations for the diff struct. We
    // can't rely on `#[derive] because we want to put bounds on the
    // Diffable::Diff types, not on the original types.
    let (impl_gen, ty_gen, _) = &new_generics.split_for_impl();

    let debug_impl = {
        let where_clause = diff_fields.where_clause_with_trait_bound(
            &parse_quote! { ::core::fmt::Debug },
        );
        let members = diff_fields.fields.members();

        let finish = if non_exhaustive.is_some() {
            quote! { .finish_non_exhaustive() }
        } else {
            quote! { .finish() }
        };

        let debug_body = match &s.fields {
            Fields::Named(_) => {
                quote! {
                    f.debug_struct(stringify!(#name))
                    #(
                        .field(stringify!(#members), &self.#members)
                    )*
                    #finish
                }
            }
            Fields::Unnamed(_) => quote! {
                f.debug_tuple(stringify!(#name))
                #(
                    .field(&self.#members)
                )*
                #finish
            },
            Fields::Unit => quote! {
                f.debug_struct(stringify!(#name))
                    #finish
            },
        };
        quote! {
            impl #impl_gen ::core::fmt::Debug for #name #ty_gen #where_clause {
                fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
                    #debug_body
                }
            }
        }
    };

    let partial_eq_impl = {
        let where_clause = diff_fields.where_clause_with_trait_bound(
            &parse_quote! { ::core::cmp::PartialEq },
        );
        let members = diff_fields.fields.members();

        let partial_eq_body: Expr = parse_quote! {
            #(self.#members == other.#members) && *
        };

        quote! {
            impl #impl_gen ::core::cmp::PartialEq for #name #ty_gen #where_clause {
                fn eq(&self, other: &Self) -> bool {
                    #partial_eq_body
                }
            }
        }
    };

    let eq_impl = {
        let where_clause = diff_fields
            .where_clause_with_trait_bound(&parse_quote! { ::core::cmp::Eq });

        quote! {
            impl #impl_gen ::core::cmp::Eq for #name #ty_gen #where_clause {}
        }
    };

    Some((
        quote! {
            #struct_def
            #debug_impl
            #partial_eq_impl
            #eq_impl
        },
        diff_fields,
    ))
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

    quote! {
        impl #impl_gen #daft_crate::Diffable for #ident #ty_gen
            #where_clause
        {
            type Diff<#daft_lt> = #name #new_ty_gen where Self: #daft_lt;

            fn diff<#daft_lt>(&#daft_lt self, other: &#daft_lt Self) -> #name #new_ty_gen {
                Self::Diff {
                    #diffs
                }
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

        if errors.has_errors() {
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
}

impl StructConfig {
    fn parse_from(
        attrs: &[Attribute],
        errors: ErrorSink<'_, syn::Error>,
    ) -> Option<Self> {
        let mut duplicates = Vec::new();
        let mut mode = StructMode::Default;

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
                                    duplicates.push(meta.error(
                                    "#[daft(leaf)] specified multiple times",
                                ));
                                }
                            }
                        } else {
                            errors.push(meta.error(
                                "unknown attribute \
                                 (supported attributes: leaf)",
                            ));
                        }

                        Ok(())
                    });

                    if let Err(err) = res {
                        errors.push(err);
                    }
                }
            }
        }

        if errors.has_errors() {
            for error in duplicates {
                errors.push(error);
            }
            None
        } else {
            for error in duplicates {
                errors.push(error);
            }
            Some(Self { mode })
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
                                errors.push(meta.error(
                                    "#[daft(leaf)] specified multiple times",
                                ));
                            }
                            _ => {
                                errors.push(meta.error(
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
                                errors.push(meta.error(
                                    "#[daft(ignore)] specified multiple times",
                                ));
                            }
                            _ => {
                                errors.push(meta.error(
                                    "#[daft(ignore)] conflicts with \
                                     other attributes",
                                ));
                            }
                        }
                    } else {
                        errors.push(meta.error(
                            "unknown attribute \
                             (supported attributes: leaf, ignore)",
                        ));
                    }

                    Ok(())
                });
                // We don't return an error from our callback, but syn might.
                if let Err(err) = res {
                    errors.push(err);
                }
            }
        }

        if errors.has_errors() {
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
