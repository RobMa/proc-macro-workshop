extern crate proc_macro;
extern crate quote;
extern crate syn;

// use proc_macro::TokenStream;
use quote::{format_ident, quote};

#[proc_macro_derive(Builder)]
pub fn derive(input: proc_macro::TokenStream) -> proc_macro::TokenStream {
    let derive_input: syn::DeriveInput = syn::parse_macro_input!(input);

    let name = derive_input.ident;
    let builder_name = format_ident!("{}Builder", name);

    let fields = {
        if let syn::Data::Struct(data_struct) = derive_input.data {
            if let syn::Fields::Named(fields) = data_struct.fields {
                fields.named
            } else {
                unimplemented!()
            }
        } else {
            unimplemented!()
        }
    };

    let struct_body = fields
        .iter()
        .map(|x| {
            let name = x.ident.as_ref().expect("Expected field identifier");
            let field_type = &x.ty;
            quote! {
                #name: Option<#field_type>
            }
        })
        .collect::<Vec<proc_macro2::TokenStream>>();

    let builder_body = fields
        .iter()
        .map(|x| {
            let name = x.ident.as_ref().expect("Expected field identifier");
            quote! {
                #name: None
            }
        })
        .collect::<Vec<proc_macro2::TokenStream>>();

    let out = quote! {
        #[derive(Debug, PartialEq)]
        struct #builder_name{
            #(#struct_body),*
        }

        impl #name {
            fn builder() -> #builder_name {
                #builder_name {
                    #(#builder_body),*
                }
            }
        }

    };

    out.into()
}
