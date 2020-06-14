extern crate proc_macro;
extern crate quote;
extern crate syn;

use quote::{format_ident, quote};

#[proc_macro_derive(Builder)]
pub fn derive(input: proc_macro::TokenStream) -> proc_macro::TokenStream {
    let derive_input: syn::DeriveInput = syn::parse_macro_input!(input);

    let name = &derive_input.ident;
    let builder_name = format_ident!("{}Builder", name);
    let fields = get_fields(&derive_input);

    let struct_body = fields
        .iter()
        .map(|x| {
            let name = x.name;
            let field_type = x.field_type;
            quote! {
                #name: Option<#field_type>
            }
        })
        .collect::<Vec<proc_macro2::TokenStream>>();

    let builder_body = fields
        .iter()
        .map(|x| {
            let name = x.name;
            quote! {
                #name: None
            }
        })
        .collect::<Vec<proc_macro2::TokenStream>>();

    let setter_functions = derive_setter_functions(&fields);

    let build_function = derive_build_function(name, &fields);

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

        impl #builder_name {
            #setter_functions

            #build_function
        }

    };

    out.into()
}

struct Field<'f> {
    name: &'f syn::Ident,
    field_type: &'f syn::Type,
}

fn get_fields<'f>(derive_input: &'f syn::DeriveInput) -> Vec<Field<'f>> {
    let fields = {
        if let syn::Data::Struct(data_struct) = &derive_input.data {
            if let syn::Fields::Named(fields) = &data_struct.fields {
                &fields.named
            } else {
                unimplemented!()
            }
        } else {
            unimplemented!()
        }
    };

    fields
        .iter()
        .map(|x| Field {
            name: x.ident.as_ref().expect("Expected identifier"),
            field_type: &x.ty,
        })
        .collect()
}

fn derive_setter_functions(fields: &[Field]) -> proc_macro2::TokenStream {
    let setter_functions = fields
        .iter()
        .map(|field| {
            let name = field.name;
            let field_type = field.field_type;
            quote! {
                fn #name(&mut self, x: #field_type) -> &mut Self{
                    self.#name = Some(x);
                    self
                }
            }
        })
        .collect::<Vec<proc_macro2::TokenStream>>();

    quote! {
        #(#setter_functions)*
    }
}

fn derive_build_function(name: &syn::Ident, fields: &[Field]) -> proc_macro2::TokenStream {
    let mut field_assignments: Vec<proc_macro2::TokenStream> = vec![];

    for field in fields {
        let field_name = field.name;
        let field_error_msg = format!("Field '{}' not initialized.", field_name);
        field_assignments.push(quote! {
            #field_name: self.#field_name.take().ok_or(#field_error_msg)?
        });
    }

    quote! {
        fn build(&mut self) -> Result<#name, Box<dyn std::error::Error>> {
            Ok(#name {
                #(#field_assignments),*
            })
        }
    }
}
