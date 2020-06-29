extern crate proc_macro;
extern crate quote;
extern crate syn;

use quote::{format_ident, quote};

#[proc_macro_derive(Builder, attributes(builder))]
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
            if x.repeated_name.is_none() {
                quote! {
                    #name: Option<#field_type>
                }
            } else {
                quote! {
                    #name: #field_type
                }
            }
        })
        .collect::<Vec<proc_macro2::TokenStream>>();

    let builder_body = fields
        .iter()
        .map(|x| {
            let name = x.name;
            let initial_value = if let Some(_) = x.repeated_name {
                quote! {vec![]}
            } else {
                quote! {None}
            };
            quote! {
                #name: #initial_value
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
    optional: bool,
    repeated_name: Option<syn::Ident>,
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
        .map(|x| {
            if is_option(&x.ty) {
                Field {
                    name: x.ident.as_ref().expect("Expected identifier"),
                    field_type: get_angle_bracket_arg(&x.ty).expect("Expected Option Type"),
                    optional: true,
                    repeated_name: get_repeated_name(&x.attrs),
                }
            } else {
                Field {
                    name: x.ident.as_ref().expect("Expected identifier"),
                    field_type: &x.ty,
                    optional: false,
                    repeated_name: get_repeated_name(&x.attrs),
                }
            }
        })
        .collect()
}

fn get_repeated_name(attrs: &[syn::Attribute]) -> Option<syn::Ident> {
    for attr in attrs.iter() {
        for segment in attr.path.segments.iter() {
            if segment.ident == "builder" {
                for token in attr.tokens.clone().into_iter() {
                    if let proc_macro2::TokenTree::Group(group) = token {
                        if group.delimiter() == proc_macro2::Delimiter::Parenthesis {
                            let mut stream = group.stream().into_iter();

                            if let (
                                proc_macro2::TokenTree::Ident(ident),
                                proc_macro2::TokenTree::Punct(equals),
                                proc_macro2::TokenTree::Literal(repeated_name),
                            ) = (stream.next()?, stream.next()?, stream.next()?)
                            {
                                if ident == "each" && equals.as_char() == '=' {
                                    if let syn::Lit::Str(repeated_name) =
                                        syn::Lit::new(repeated_name)
                                    {
                                        return Some(format_ident!("{}", repeated_name.value()));
                                    }
                                }
                            }
                        }
                    }
                }
            }
        }
    }

    None
}

fn is_option(t: &syn::Type) -> bool {
    match t {
        syn::Type::Path(t) => match t.path.segments.first() {
            Some(t) => t.ident == "Option",
            _ => false,
        },
        _ => false,
    }
}

fn get_angle_bracket_arg<'f>(t: &'f syn::Type) -> Option<&'f syn::Type> {
    if let syn::Type::Path(t) = t {
        if let Some(t) = t.path.segments.first() {
            if let syn::PathArguments::AngleBracketed(t) = &t.arguments {
                if let Some(t) = t.args.first() {
                    if let syn::GenericArgument::Type(t) = t {
                        return Some(t);
                    }
                }
            }
        }
    }
    None
}

fn derive_setter_functions(fields: &[Field]) -> proc_macro2::TokenStream {
    let setter_functions = fields
        .iter()
        .map(|field| {
            let name = field.name;
            if let Some(repeated_name) = field.repeated_name.as_ref() {
                let repeated_type =
                    get_angle_bracket_arg(field.field_type).expect("Expected vector type");
                quote! {
                    fn #repeated_name(&mut self, x: #repeated_type) -> &mut Self{
                        self.#name.push(x);
                        self
                    }
                }
            } else {
                let field_type = field.field_type;
                quote! {
                    fn #name(&mut self, x: #field_type) -> &mut Self{
                        self.#name = Some(x);
                        self
                    }
                }
            }
        })
        .collect::<Vec<proc_macro2::TokenStream>>();

    quote! {
        #(#setter_functions)*
    }
}

fn derive_build_function(name: &syn::Ident, fields: &[Field]) -> proc_macro2::TokenStream {
    let field_assignments: Vec<proc_macro2::TokenStream> = fields
        .iter()
        .map(|field| {
            let field_name = field.name;
            let field_error_msg = format!("Field '{}' not initialized.", field_name);
            if !field.optional && field.repeated_name.is_none() {
                quote! {
                    #field_name: self.#field_name.take().ok_or(#field_error_msg)?
                }
            } else if let Some(_) = field.repeated_name {
                quote! {
                    #field_name: self.#field_name.clone()
                }
            } else {
                quote! {
                    #field_name: self.#field_name.take()
                }
            }
        })
        .collect();

    quote! {
        fn build(&mut self) -> Result<#name, Box<dyn std::error::Error>> {
            Ok(#name {
                #(#field_assignments),*
            })
        }
    }
}
