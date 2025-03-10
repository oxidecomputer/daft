//! Derive macro for daft (internal crate).
//!
//! For more information about daft, see [its documentation](https://docs.rs/daft).
// Setting html_root_url allows daft's readme to have links to daft-derive. This
// line is updated by cargo-release.
#![doc(html_root_url = "https://docs.rs/daft-derive/0.1.2")]
mod internals;

use quote::ToTokens;
use syn::parse_macro_input;

// NOTE: We do not define documentation here -- only in daft while re-exporting
// these items. This is so that doctests that depend on daft work.

#[proc_macro_derive(Diffable, attributes(daft))]
pub fn derive_diffable(
    input: proc_macro::TokenStream,
) -> proc_macro::TokenStream {
    let input = parse_macro_input!(input as syn::DeriveInput);
    internals::derive_diffable(input).into_token_stream().into()
}
