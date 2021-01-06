use proc_macro::TokenStream;
use syn::parse_macro_input::parse;
use syn::DeriveInput;

/// Currently only prints the given attributes.
/// In a real attribute, the parameters should not be parsed from a string. Instead, syn should
/// be used.
#[proc_macro_attribute]
pub fn awesome_attribute(attributes: TokenStream, item: TokenStream) -> TokenStream {
    println!("{}", attributes.to_string());
    item
}
