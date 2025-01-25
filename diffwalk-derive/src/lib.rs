use proc_macro::TokenStream;
use syn::parse_macro_input;

#[proc_macro_derive(DiffWalk, attributes(diffwalk))]
pub fn derive_diffwalk(input: proc_macro::TokenStream) -> proc_macro::TokenStream {
    input
}
