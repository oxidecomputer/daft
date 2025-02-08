//! Derive macro for daft (internal crate).
//!
//! For more information about daft, see [its documentation](https://docs.rs/daft).

mod internals;

use syn::parse_macro_input;

// NOTE: We do not define documentation here -- only in daft while re-exporting
// these items. This is so that doctests that depend on daft work.

#[proc_macro_derive(Diffable, attributes(daft))]
pub fn derive_diffable(
    input: proc_macro::TokenStream,
) -> proc_macro::TokenStream {
    let input = parse_macro_input!(input as syn::DeriveInput);
    internals::derive_diffable(input).into()
}
