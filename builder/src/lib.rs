extern crate proc_macro;
extern crate quote;
extern crate syn;

use proc_macro::TokenStream;
use quote::quote;

#[proc_macro_derive(Builder)]
pub fn derive(input: TokenStream) -> TokenStream {
    let derive_input: syn::DeriveInput = syn::parse_macro_input!(input);

    // println!("derive_input = {:#?}", derive_input);

    let name = derive_input.ident;

    let out = quote! {
        impl #name {
            fn builder() {}
        }
    };

    out.into()
}
