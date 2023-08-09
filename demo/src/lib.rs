use proc_macro::TokenStream;
use syn::{parse_macro_input, DeriveInput};

#[proc_macro_derive(Builder)]
pub fn derive(input: TokenStream) -> TokenStream {
    let _parsed = parse_macro_input!(input as DeriveInput);

    TokenStream::new()
}
