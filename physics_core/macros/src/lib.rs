#[macro_use]
extern crate quote;

#[macro_use]
extern crate syn;

extern crate proc_macro;

use proc_macro::TokenStream;
use syn::{DeriveInput, parse::Parser};

#[proc_macro_attribute]
pub fn physics_entity(_args: TokenStream, input: TokenStream) -> TokenStream {
    let mut ast = parse_macro_input!(input as DeriveInput);

    match &mut ast.data {
        syn::Data::Struct(ref mut struct_data) => {
            match &mut struct_data.fields {
                syn::Fields::Named(fields) => {
                    fields.named.push(
                        syn::Field::parse_named.parse2(quote! {pub position: (u32, u32)}).unwrap()
                    );
                },
                _ => {()}
            }
        },
        _ => panic!("Entity must be derived on a struct!")
    }

    return quote! {
        #[derive(Entity)]
        #ast
    }.into();
}

#[proc_macro_derive(Entity)]
pub fn entity(input: TokenStream) -> TokenStream {
    let ast = parse_macro_input!(input as DeriveInput);

    let name = &ast.ident;

    quote! {
        impl EntityType for #name {
            fn set_position(&mut self, pos: (u32, u32)) {
                self.position = pos;
            }
            fn get_position(&self) -> (u32, u32) {
                self.position
            }
        }
    }.into()
}