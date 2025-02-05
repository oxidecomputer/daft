//! Internals for daft-derive.
//!
//! This is imported both by this crate's lib.rs and by tests/snapshot_test.rs.

use proc_macro2::{Span, TokenStream};
use quote::{quote, ToTokens};
use syn::{
    parse_quote, parse_str, Data, DataStruct, DeriveInput, Expr, Field, Fields,
    GenericParam, Generics, Index, Lifetime, LifetimeParam, Path, Token,
    WhereClause, WherePredicate,
};

pub fn derive_diffable(input: syn::DeriveInput) -> TokenStream {
    let name = &input.ident;

    match &input.data {
        Data::Enum(_) => {
            // Implement all Enums as `Leaf`s
            let out = make_leaf_for_enum(&input);
            quote! {
                #out
            }
        }
        Data::Struct(s) => {
            let generated_struct = make_diff_struct(&input, s);
            let diff_impl = make_diff_impl(&input, s);
            // Uncomment for some debugging
            // eprintln!("{generated_struct}");
            // eprintln!("{diff_impl}");
            quote! {
                #generated_struct
                #diff_impl
            }
        }

        Data::Union(_) => {
            let daft_crate = daft_crate();
            quote! {
                // Implement all Unions as `Leaf`s
                #daft_crate::leaf!(#name);
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

// Implement `Diffable` for an enum
//
// Return a `Leaf` as a Diff
fn make_leaf_for_enum(input: &DeriveInput) -> TokenStream {
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
            type Diff<#daft_lt> = #daft_crate::Leaf<#daft_lt, Self> where Self: #daft_lt;

            fn diff<#daft_lt>(&#daft_lt self, other: &#daft_lt Self) -> Self::Diff<#daft_lt> {
                #daft_crate::Leaf {before: self, after: other}
            }
        }
    }
}

/// Create the `Diff` struct
fn make_diff_struct(input: &DeriveInput, s: &DataStruct) -> TokenStream {
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

    let diff_fields = DiffFields::new(&s.fields, where_clause.as_ref());

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
        let where_clause = diff_fields
            .where_clause_with_trait_bound(&parse_quote! { ::std::fmt::Debug });
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
            impl #impl_gen ::std::fmt::Debug for #name #ty_gen #where_clause {
                fn fmt(&self, f: &mut ::std::fmt::Formatter<'_>) -> ::std::fmt::Result {
                    #debug_body
                }
            }
        }
    };

    let partial_eq_impl = {
        let where_clause = diff_fields.where_clause_with_trait_bound(
            &parse_quote! { ::std::cmp::PartialEq },
        );
        let members = diff_fields.fields.members();

        let partial_eq_body: Expr = parse_quote! {
            #(self.#members == other.#members) && *
        };

        quote! {
            impl #impl_gen ::std::cmp::PartialEq for #name #ty_gen #where_clause {
                fn eq(&self, other: &Self) -> bool {
                    #partial_eq_body
                }
            }
        }
    };

    let eq_impl = {
        let where_clause = diff_fields
            .where_clause_with_trait_bound(&parse_quote! { ::std::cmp::Eq });

        quote! {
            impl #impl_gen ::std::cmp::Eq for #name #ty_gen #where_clause {}
        }
    };

    quote! {
        #struct_def
        #debug_impl
        #partial_eq_impl
        #eq_impl
    }
}

/// Impl `Diffable` for the original struct
fn make_diff_impl(input: &DeriveInput, s: &DataStruct) -> TokenStream {
    // The name of the original type
    let ident = &input.ident;

    // The name of the generated type
    let name = parse_str::<Path>(&format!("{}Diff", input.ident)).unwrap();
    let diffs = generate_field_diffs(&s.fields);

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
    // The base where clause for the diff struct.
    where_clause: WhereClause,
}

impl DiffFields {
    fn new(fields: &Fields, where_clause: Option<&WhereClause>) -> Self {
        let fields = match fields {
            Fields::Named(fields) => {
                let diff_fields = fields
                    .named
                    .iter()
                    .filter(|f| !has_ignore_attr(f))
                    .map(Self::diff_field);
                Fields::Named(syn::FieldsNamed {
                    brace_token: fields.brace_token,
                    named: diff_fields.collect(),
                })
            }
            Fields::Unnamed(fields) => {
                let diff_fields = fields
                    .unnamed
                    .iter()
                    .filter(|f| !has_ignore_attr(f))
                    .map(Self::diff_field);
                Fields::Unnamed(syn::FieldsUnnamed {
                    paren_token: fields.paren_token,
                    unnamed: diff_fields.collect(),
                })
            }
            Fields::Unit => Fields::Unit,
        };

        // Initialize an empty where clause if none was provided.
        let where_clause =
            where_clause.cloned().unwrap_or_else(|| WhereClause {
                where_token: <Token![where]>::default(),
                predicates: Default::default(),
            });

        Self { fields, where_clause }
    }

    /// Return a field for a diff with the appropriate type
    fn diff_field(f: &Field) -> Field {
        // Always use the daft lifetime for the diff -- associations between the
        // daft lifetime and existing parameters (both lifetime and type
        // parameters) are created in `add_lifetime_to_generics`, e.g. `'a:
        // '__daft`, or `T: '__daft`.
        let lt = daft_lifetime();
        let daft_crate = daft_crate();
        let ty = &f.ty;
        let mut f = f.clone();

        f.ty = if has_leaf_attr(&f) {
            parse_quote! {
                #daft_crate::Leaf<#lt, #ty>
            }
        } else {
            parse_quote! {
                <#ty as #daft_crate::Diffable>::Diff<#lt>
            }
        };

        // Drop all attributes for now. We may want to carry some over in the
        // future.
        f.attrs = vec![];

        f
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
            parse_quote! {
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
fn generate_field_diffs(fields: &Fields) -> TokenStream {
    let daft_crate = daft_crate();
    let field_diffs =
        fields.iter().enumerate().filter(|(_, f)| !has_ignore_attr(f)).map(
            |(i, f)| {
                let field_name = match &f.ident {
                    Some(ident) => quote! { #ident },
                    None => {
                        let ident: Index = i.into();
                        quote! { #ident }
                    }
                };
                if has_leaf_attr(f) {
                    quote! {
                        #field_name: #daft_crate::Leaf {
                            before: &self.#field_name,
                            after: &other.#field_name
                        }
                    }
                } else {
                    quote! {
                        #field_name: #daft_crate::Diffable::diff(
                            &self.#field_name,
                            &other.#field_name
                        )
                    }
                }
            },
        );
    quote! { #(#field_diffs),* }
}

// Is the field tagged with `#[daft(ignore)]` ?
fn has_ignore_attr(field: &syn::Field) -> bool {
    has_attr(field, "ignore")
}

// Is the field tagged with `#[daft(leaf)]`
fn has_leaf_attr(field: &syn::Field) -> bool {
    has_attr(field, "leaf")
}

// Is the field tagged with a given attribute?
fn has_attr(field: &syn::Field, attribute_name: &str) -> bool {
    field.attrs.iter().any(|attr| {
        if attr.path().is_ident("daft") {
            // Ignore failures
            if let Ok(meta) = attr.parse_args::<syn::Meta>() {
                if meta.path().is_ident(attribute_name) {
                    return true;
                }
            }
        }
        false
    })
}
