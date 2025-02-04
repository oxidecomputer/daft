//! Internals for daft-derive.
//!
//! This is imported both by this crate's lib.rs and by tests/snapshot_test.rs.

use proc_macro2::{Span, TokenStream};
use quote::quote;
use syn::{
    parse_str, Data, DataStruct, DeriveInput, Fields, GenericParam, Generics,
    Index, Lifetime, LifetimeParam, Path,
};

pub fn derive_diff(input: syn::DeriveInput) -> TokenStream {
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

        Data::Union(_) => quote! {
            // Implement all Unions as `Leaf`s
            daft::leaf!(#name);
        },
    }
}

fn daft_lifetime() -> LifetimeParam {
    LifetimeParam::new(Lifetime::new("'daft", Span::call_site()))
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
    let daft_lt = daft_lifetime();

    // The "where Self: #daft_lt" condition appears to be enough to satisfy
    // Rust's borrow checker, so we don't need to add further constraints via
    // `add_lifetime_to_generics`.
    let (impl_gen, ty_gen, where_clause) = &input.generics.split_for_impl();

    quote! {
        impl #impl_gen daft::Diffable for #ident #ty_gen #where_clause
        {
            type Diff<#daft_lt> = daft::Leaf<#daft_lt, Self> where Self: #daft_lt;

            fn diff<#daft_lt>(&#daft_lt self, other: &#daft_lt Self) -> Self::Diff<#daft_lt> {
                daft::Leaf {before: self, after: other}
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
    let fields = generate_fields(&s.fields);

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

    match &s.fields {
        Fields::Named(_) => quote! {
            #[derive(Debug, PartialEq, Eq)]
            #vis struct #name #new_generics #where_clause {
                #fields
            }
        },
        Fields::Unnamed(_) => quote! {
            #[derive(Debug, PartialEq, Eq)]
            #vis struct #name #new_generics (#fields) #where_clause;
        },
        Fields::Unit => quote! {
            // This is kinda silly
            #vis struct #name #new_generics {} #where_clause
        },
    }
}

/// Impl `Diffable` for the original struct
fn make_diff_impl(input: &DeriveInput, s: &DataStruct) -> TokenStream {
    // The name of the original type
    let ident = &input.ident;

    // The name of the generated type
    let name = parse_str::<Path>(&format!("{}Diff", input.ident)).unwrap();
    let diffs = generate_field_diffs(&s.fields);

    let daft_lt = daft_lifetime();
    let new_generics = add_lifetime_to_generics(input, &daft_lt);

    let (impl_gen, ty_gen, _) = &input.generics.split_for_impl();
    let (_, new_ty_gen, where_clause) = &new_generics.split_for_impl();

    quote! {
        impl #impl_gen daft::Diffable for #ident #ty_gen
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

// Generate fields for the generated struct
fn generate_fields(fields: &Fields) -> TokenStream {
    let fields = fields.iter().filter(|f| !has_ignore_attr(f)).map(|f| {
        let vis = &f.vis;
        let ty = &f.ty;

        // Always use the daft lifetime for the diff -- associations between
        // that and existing parameters are handled in
        // `add_lifetime_to_generics`.
        let lt = daft_lifetime();

        match &f.ident {
            Some(ident) => quote! {
                #vis #ident: <#ty as daft::Diffable>::Diff<#lt>
            },
            None => quote! {
                #vis <#ty as daft::Diffable>::Diff<#lt>
            },
        }
    });
    quote! { #(#fields),* }
}

/// Generate a call to `diff` for each field of the original struct that isn't
/// ignored.
fn generate_field_diffs(fields: &Fields) -> TokenStream {
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

                quote! {
                    #field_name: daft::Diffable::diff(
                        &self.#field_name,
                        &other.#field_name
                    )
                }
            },
        );
    quote! { #(#field_diffs),* }
}

// Is the field tagged with `#[daft(ignore)]` ?
fn has_ignore_attr(field: &syn::Field) -> bool {
    field.attrs.iter().any(|attr| {
        if attr.path().is_ident("daft") {
            // Ignore failures
            if let Ok(meta) = attr.parse_args::<syn::Meta>() {
                if meta.path().is_ident("ignore") {
                    return true;
                }
            }
        }
        false
    })
}
