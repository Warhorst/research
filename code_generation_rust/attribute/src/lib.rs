use proc_macro::TokenStream;
use syn::parse_macro_input::parse;
use syn::{DeriveInput, ItemFn, parse_macro_input, Attribute, FnArg, PatType};
use quote::quote;

/// Currently only prints the given attributes.
/// In a real attribute, the parameters should not be parsed from a string. Instead, syn should
/// be used.
#[proc_macro_attribute]
pub fn awesome_attribute(attributes: TokenStream, item: TokenStream) -> TokenStream {
    item
}

/// Help
/// https://stackoverflow.com/questions/66850967/procedural-attribute-macro-to-inject-code-at-the-beginning-of-a-function
/// https://doc.rust-lang.org/stable/book/ch19-06-macros.html
#[proc_macro_attribute]
pub fn debug(attributes: TokenStream, item: TokenStream) -> TokenStream {
    // parse the function body token stream
    let mut input = parse_macro_input!(item as ItemFn);
    let ItemFn { attrs, vis, mut sig, block } = input;
    let stmts = &block.stmts;

    // parse a token stream from some new tokens
    let foo: TokenStream = quote!{
        n: usize
    }.into();

    let mut inputs = &mut sig.inputs;
    // add the new arg to the existing ones. Parse it to syn structs first.
    inputs.push(parse_macro_input!(foo as FnArg));

    let gen = quote! {
        #(#attrs)* #vis #sig {
            println!("foo {}", n)
            #(#stmts)*
        }
    };
    gen.into()
}