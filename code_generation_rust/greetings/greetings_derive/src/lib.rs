extern crate proc_macro;

use proc_macro::TokenStream;
use quote::quote;
use syn::parse_macro_input::parse;
use syn::DeriveInput;

#[proc_macro_derive(Greetings)]
pub fn greetings_derive(input: TokenStream) -> TokenStream  {
    let ast = parse(input).unwrap();
    impl_greetings(&ast)
}

fn impl_greetings(ast: &DeriveInput) -> TokenStream {
    let name = &ast.ident;
    let gen = quote! {
        impl Greetings for #name {
            fn greet(&self) {
                println!("Greetings Sir! My name is {}!", stringify!(#name));
            }
        }
    };
    gen.into()
}
