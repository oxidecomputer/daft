//! Test that default field values don't make their way into generated diff
//! structs.
//!
//! Default field values ([Rust RFC 3681]) let a struct field carry an
//! initializer, as in `struct Foo { a: u32 = 5 }`. Daft drops these in its own
//! struct definitions.
//!
//! These tests live here rather than as snapshot tests for two reasons:
//! because trybuild runs against stable where (as of Rust 1.97) default values
//! aren't supported, and because prettyplease 0.3.0 seems to eat default field
//! values. Worth revisiting in the future.
//!
//! [Rust RFC 3681]: https://rust-lang.github.io/rfcs/3681-default-field-values.html

use quote::ToTokens;

#[path = "../../src/internals/mod.rs"]
mod internals;

/// Parse `input`, run the derive macro over it, and return the generated diff
/// struct.
fn derive_diff_struct(input: &str) -> syn::ItemStruct {
    let parsed: syn::DeriveInput =
        syn::parse_str(input).expect("input parsed as a DeriveInput");
    let diff_name = format!("{}Diff", parsed.ident);

    let generated = internals::derive_diffable(parsed).into_token_stream();
    let file: syn::File =
        syn::parse2(generated.clone()).unwrap_or_else(|err| {
            panic!("generated code parsed as a file: {err}\n{generated}");
        });

    file.items
        .into_iter()
        .find_map(|item| match item {
            syn::Item::Struct(s) if s.ident == diff_name => Some(s),
            _ => None,
        })
        .unwrap_or_else(|| {
            panic!("generated code defines {diff_name}:\n{generated}");
        })
}

/// Assert that no field of `s` carries a default value.
fn assert_no_field_defaults(s: &syn::ItemStruct) {
    let kept: Vec<_> = s
        .fields
        .iter()
        .enumerate()
        .filter(|(_, f)| f.default.is_some())
        .map(|(i, f)| match &f.ident {
            Some(ident) => ident.to_string(),
            None => i.to_string(),
        })
        .collect();

    assert!(
        kept.is_empty(),
        "fields of {} kept a default value from the original struct: {:?}\n{}",
        s.ident,
        kept,
        s.to_token_stream(),
    );
}

/// Return the names of `s`'s fields, in declaration order.
fn field_names(s: &syn::ItemStruct) -> Vec<String> {
    s.fields
        .iter()
        .map(|f| f.ident.as_ref().expect("named field").to_string())
        .collect()
}

/// Test that default on a named field is dropped from the diff struct.
///
/// The rest of the struct still diffs normally when a default is present.
#[test]
fn named_field_default_is_dropped() {
    let diff =
        derive_diff_struct("struct HasDefault { a: u32 = 5, b: String }");
    assert_no_field_defaults(&diff);
    assert_eq!(field_names(&diff), ["a", "b"]);
}

/// Test that a default on a `#[daft(leaf)]` field is dropped too.
#[test]
fn leaf_field_default_is_dropped() {
    let diff = derive_diff_struct(
        "struct HasLeafDefault { #[daft(leaf)] a: u32 = 5 }",
    );
    assert_no_field_defaults(&diff);
    assert_eq!(field_names(&diff), ["a"]);
}

/// Test that a default on an ignored field doesn't reappear.
///
/// `#[daft(ignore)]` drops the field entirely, so the diff struct is empty and
/// there is nothing for a default to attach to.
#[test]
fn ignored_field_default_is_dropped() {
    let diff = derive_diff_struct(
        "struct HasIgnoredDefault { #[daft(ignore)] a: u32 = 5, b: String }",
    );
    assert_no_field_defaults(&diff);
    assert_eq!(field_names(&diff), ["b"]);
}
