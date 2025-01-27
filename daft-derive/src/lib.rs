use proc_macro2::TokenStream;
use quote::quote;
use syn::{
    parse_macro_input, parse_str, Data, DataStruct, DeriveInput, Fields, Index,
    Path,
};

#[proc_macro_derive(Diff, attributes(daft))]
pub fn derive_diff(input: proc_macro::TokenStream) -> proc_macro::TokenStream {
    let input = parse_macro_input!(input as syn::DeriveInput);

    let name = &input.ident;

    match &input.data {
        Data::Enum(_) => quote! {
            // Implement all Enums as `Leaf`s
            daft::leaf!(#name);

        }
        .into(),
        Data::Struct(s) => {
            let generated_struct = make_diff_struct(&input, &s);
            let diff_impl = make_diff_impl(&input, &s);
            quote! {
                #generated_struct
                #diff_impl
            }
            .into()
        }

        Data::Union(_) => quote! {
            // Implement all Unions as `Leaf`s
            daft::leaf!(#name);
        }
        .into(),
    }
}

/// Create the `Diff` struct
//
// TODO: Handle generics:
// see https://docs.rs/syn/latest/syn/struct.Generics.html#method.split_for_impl
fn make_diff_struct(input: &DeriveInput, s: &DataStruct) -> TokenStream {
    // The name of the original type
    let vis = &input.vis;

    // The name of the generated type
    let name = parse_str::<Path>(&format!("{}Diff", input.ident)).unwrap();
    let fields = generate_fields(&s.fields);

    match &s.fields {
        Fields::Named(_) => quote! {
            #[derive(Debug, PartialEq, Eq)]
            #vis struct #name<'a> {
                #fields
            }
        },
        Fields::Unnamed(_) => quote! {
            #[derive(Debug, PartialEq, Eq)]
            #vis struct #name<'a>(#fields);
        },
        Fields::Unit => quote! {
            // This is kinda silly
            #vis struct #name {}
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

    quote! {
        impl<'a> daft::Diffable<'a> for #ident {
            type Diff = #name<'a>;

            fn diff(&'a self, other: &'a Self) -> Self::Diff {
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
        match &f.ident {
            Some(ident) => quote! {
                #vis #ident: <#ty as daft::Diffable<'a>>::Diff
            },
            None => quote! {
                #vis <#ty as daft::Diffable<'a>>::Diff
            },
        }
    });
    quote! { #(#fields),* }
}

/// Generate a call to `diff` for each field of the original struct that isn't
/// ignored.
fn generate_field_diffs(fields: &Fields) -> TokenStream {
    let field_diffs = fields
        .iter()
        .enumerate()
        .filter(|(_, f)| !has_ignore_attr(f))
        .map(|(i, f)| {
            let field_name = match &f.ident {
                Some(ident) => quote! { #ident },
                None => {
                    let ident: Index = i.into();
                    quote! { #ident }
                }
            };
            quote! {
                #field_name: daft::Diffable::diff(&self.#field_name, &other.#field_name)
            }
        });
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
