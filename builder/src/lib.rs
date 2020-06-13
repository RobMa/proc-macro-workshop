extern crate proc_macro;
extern crate quote;
extern crate syn;

use proc_macro::TokenStream;

#[proc_macro_derive(Builder)]
pub fn derive(input: TokenStream) -> TokenStream {
    let _ = input;

    let _derive_input: syn::DeriveInput = syn::parse_macro_input!(input);

    println!("_derive_input = {:#?}", _derive_input);

    TokenStream::new()
}
