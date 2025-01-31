mod internals;

use syn::parse_macro_input;

#[proc_macro_derive(Diff, attributes(daft))]
pub fn derive_diff(input: proc_macro::TokenStream) -> proc_macro::TokenStream {
    let input = parse_macro_input!(input as syn::DeriveInput);
    internals::derive_diff(input).into()
}
