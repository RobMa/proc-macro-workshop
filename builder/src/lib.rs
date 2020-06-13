extern crate proc_macro;
extern crate quote;
extern crate syn;

use proc_macro::TokenStream;
use quote::{format_ident, quote};

#[proc_macro_derive(Builder)]
pub fn derive(input: TokenStream) -> TokenStream {
    let derive_input: syn::DeriveInput = syn::parse_macro_input!(input);

    println!("derive_input = {:#?}", derive_input);

    let name = derive_input.ident;
    // let builder_name = syn::Ident::new(
    //     &(name.to_string() + "Builder"),
    //     proc_macro::Span::call_site().into(),
    // );
    let builder_name = format_ident!("{}Builder", name);

    let out = quote! {
        #[derive(Debug, PartialEq)]
        struct #builder_name{
            executable: Option<String>,
            args: Option<Vec<String>>,
            env: Option<Vec<String>>,
            current_dir: Option<String>,
        }

        impl #name {
            fn builder() -> #builder_name {
                #builder_name {
                    executable: None,
                    args: None,
                    env: None,
                    current_dir: None,
                }
            }
        }

    };

    out.into()
}
