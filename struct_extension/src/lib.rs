#[macro_use]
extern crate quote;

#[macro_use]
extern crate syn;

extern crate proc_macro;

use proc_macro::TokenStream;
use syn::{DeriveInput, parse::Parser};

#[proc_macro_attribute]
pub fn extendable(_args: TokenStream, input: TokenStream) -> TokenStream {
    let mut ast = parse_macro_input!(input as DeriveInput);
    let original_ident = ast.ident;


    ast.ident = syn::parse_str::<syn::Ident>(&format!("{original_ident}_")).unwrap();

    let fields = match &ast.data {
        syn::Data::Struct(syn::DataStruct { fields: syn::Fields::Named(fields), .. }) => &fields.named,
        _ => panic!("extendable can only be implemented on structs with named fields"),
    };

    let func_definitions = fields.iter().map(|field| { 
        let ty = &field.ty;
        let ident = &field.ident.clone().unwrap();

        let setter_ident = syn::Ident::new(&format!("set_{ident}"), ident.span());
        let getter_ident = syn::Ident::new(&format!("get_{ident}"), ident.span());

        quote! {
            fn #setter_ident (&mut self, #ident: #ty);
            fn #getter_ident (&self) -> #ty;
        }
    }).flatten();

    let field_defs: String = fields.iter().map(|field| {
        let ident = &field.ident.clone().unwrap();
        let ty = &field.ty;
        
        let field_ident = syn::Ident::new(&format!("_EXTENDABLE_{ident}_"), ident.span()); 
        quote! {
            #field_ident: #ty,
        }.to_string()
    }).collect();

    let field_defs_ident = syn::Ident::new(&format!("_EXTEND_FIELD_DEFS_{original_ident}_"), original_ident.span());

    quote! {
        #ast
        pub trait #original_ident {
            #(#func_definitions)*
        }

        #[allow(non_camel_case_types, non_upper_case_globals)]
        pub const #field_defs_ident: &'static str = #field_defs;
    }.into()
}

#[proc_macro_attribute]
pub fn extend(args: TokenStream, input: TokenStream) -> TokenStream {
    let ast = parse_macro_input!(input as DeriveInput);
    let extendable = args.to_string();
    let extendable_ident = syn::parse_str::<syn::Ident>(&extendable).unwrap();

    let extendable_const = syn::parse_str::<syn::Ident>(&format!("_EXTEND_FIELD_DEFS_{extendable}_")).unwrap();

    let extendable_fields = quote! {
        [_extend(#extendable_ident, #extendable_const)]
    };

    quote! {
        // #[extend_fields( #extendable_fields )]
        ##extendable_fields
        #ast
    }.into()
}

#[proc_macro_attribute]
pub fn _extend(args: TokenStream, input: TokenStream) -> TokenStream {
    let mut ast = parse_macro_input!(input as DeriveInput);

    let binding = args.to_string();
    let args_vec: Vec<&str> = binding.split(",").collect();
    let extendable_ident = syn::parse_str::<syn::Ident>(args_vec[0]).unwrap().to_string();
    let ident = &ast.ident;

    let ligmaballs: String = args_vec[1..].iter().map(|f| f.to_string()).collect();

    let ident_str = ident.to_string();

    let mut func_impls = Vec::<quote::__private::TokenStream>::new();

    let impl_str: String = func_impls.iter().map(|u| u.to_string()).collect();

    match &mut ast.data {
        syn::Data::Struct(ref mut struct_data) => {
            match &mut struct_data.fields {
                syn::Fields::Named(fields) => {
                    func_impls = args_vec[1..].iter().map(|arg| {
                        if let Ok(field) = syn::Field::parse_named.parse_str(arg) {
                            fields.named.push(field.clone());

                            let ty = &field.ty;
                            let ident = &field.ident.clone().unwrap();
                            let binding = ident.to_string();
                            let cleaned_ident_str = binding[19..].trim_end_matches("_");
                            let cleaned_ident = &syn::Ident::new(cleaned_ident_str, ident.span());

                            let setter_ident = syn::Ident::new(&format!("set_{cleaned_ident}"), ident.span());
                            let getter_ident = syn::Ident::new(&format!("get_{cleaned_ident}"), ident.span());

                            quote! {
                                pub fn #setter_ident (&mut self, #ident: #ty) {
                                    self.#ident = #ident;
                                }
                                pub fn #getter_ident (&self) -> #ty {
                                    self.#ident
                                }
                            }
                        }
                        else {
                            quote! {
                                pub fn sugma(&self) {
                                    ()
                                }
                            }
                        }
                    }).collect();
                },
                _ => {()}
            }
        },
        _ => panic!("extend_fields must be implemented on a struct")
    }

    quote! {
        #ast
        impl #ident {
            pub fn ligma(&self) {
                ()
            }
            #(#func_impls)*
        }
        const G: &'static str = #binding;
    }.into()
}