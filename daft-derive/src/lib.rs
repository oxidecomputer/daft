//! Derive macro for daft (internal crate).
//!
//! For more information about daft, see [its documentation](https://docs.rs/daft).

mod internals;

use syn::parse_macro_input;

#[proc_macro_derive(Diffable, attributes(daft))]
pub fn derive_diffable(
    input: proc_macro::TokenStream,
) -> proc_macro::TokenStream {
    let input = parse_macro_input!(input as syn::DeriveInput);
    internals::derive_diffable(input).into()
}
