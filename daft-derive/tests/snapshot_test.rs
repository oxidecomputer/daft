use datatest_stable::Utf8Path;
use quote::ToTokens;
use syn::{parse_quote, DeriveInput};

// We need access to the proc-macro's internals for this test. An alternative
// would be to make this a unit test, but the integration test harness gives us
// automatic discovery of tests in the `fixtures/` directory, along with
// separate reporting for each test. Those are nice benefits.
#[path = "../src/internals/mod.rs"]
mod internals;

datatest_stable::harness! {
    // The pattern matches all .rs files that aren't .output.rs files.
    { test = daft_snapshot, root = "tests/fixtures/valid", pattern = r"^.*(?<!\.output)\.rs$" },
    { test = daft_snapshot_invalid, root = "tests/fixtures/invalid", pattern = r"^.*(?<!\.output)\.rs$" },
}

/// Snapshot tests for valid inputs.
fn daft_snapshot(
    path: &Utf8Path,
    input: String,
) -> datatest_stable::Result<()> {
    let data = syn::parse_str::<syn::File>(&input)?;

    let output = run_derive_macro(&data);
    assert_derive_output(path, output);

    Ok(())
}

/// Snapshot tests for invalid inputs.
fn daft_snapshot_invalid(
    path: &Utf8Path,
    input: String,
) -> datatest_stable::Result<()> {
    let data = syn::parse_str::<syn::File>(&input)?;

    let output = run_derive_macro(&data).map(|output| {
        // Drop the errors for snapshot tests -- only use the output.
        output.out
    });
    assert_derive_output(path, output);

    Ok(())
}

fn run_derive_macro(
    data: &syn::File,
) -> impl Iterator<Item = internals::DeriveDiffableOutput> + '_ {
    // Look for structs and enums in the input -- give them to the derive macro.
    let items = data.items.iter().filter_map(|item| match item {
        syn::Item::Struct(item) => {
            has_derive_diffable(&item.attrs).then(|| item.to_token_stream())
        }
        syn::Item::Enum(item) => {
            has_derive_diffable(&item.attrs).then(|| item.to_token_stream())
        }
        syn::Item::Union(item) => {
            has_derive_diffable(&item.attrs).then(|| item.to_token_stream())
        }
        _ => None,
    });

    // Turn each item into a `syn::DeriveInput` and run the derive macro on it.
    items.enumerate().map(|(i, item)| {
        let data = syn::parse2::<DeriveInput>(item).unwrap_or_else(|err| {
            panic!("failed to parse item {}: {}", i, err);
        });
        internals::derive_diffable(data)
    })
}

fn has_derive_diffable(attrs: &[syn::Attribute]) -> bool {
    attrs.iter().any(|attr| {
        if !attr.path().is_ident("derive") {
            return false;
        }

        let mut is_diffable = false;
        attr.parse_nested_meta(|meta| {
            if meta.path.is_ident("Diffable") {
                is_diffable = true;
            }
            Ok(())
        })
        .expect("derive attributes parsed correctly");
        is_diffable
    })
}

fn assert_derive_output<T: ToTokens>(
    path: &Utf8Path,
    output: impl IntoIterator<Item = T>,
) {
    // Read the output as a `syn::File`.
    let output = output.into_iter();
    let file = parse_quote! {
        #(#output)*
    };

    // Format the output.
    let output = prettyplease::unparse(&file);

    // Compare the output with the snapshot. The new filename is the same as the
    // input, but with ".output.rs" at the end.
    let mut output_path = path.parent().unwrap().to_owned();
    output_path.push("output");
    output_path.push(path.file_name().unwrap());
    output_path.set_extension("output.rs");

    expectorate::assert_contents(&output_path, &output);
}
