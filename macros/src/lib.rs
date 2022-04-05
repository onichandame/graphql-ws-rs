use client_trait::client_trait_expand;
use proc_macro::TokenStream;
use subscriber::subscriber_expand;
use syn::{parse_macro_input, DeriveInput};

mod client_trait;
mod subscriber;

#[proc_macro_derive(ClientTrait)]
pub fn client_trait_derive(input: TokenStream) -> TokenStream {
    let input = parse_macro_input!(input as DeriveInput);
    client_trait_expand(input).into()
}

#[proc_macro_derive(Subscriber, attributes(graphql_ws))]
pub fn subscriber_derive(input: TokenStream) -> TokenStream {
    let input = parse_macro_input!(input as DeriveInput);
    subscriber_expand(input).unwrap().into()
}
