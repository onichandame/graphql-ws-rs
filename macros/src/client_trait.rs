use proc_macro2::TokenStream;
use quote::quote;
use syn::DeriveInput;

pub fn client_trait_expand(input: DeriveInput) -> TokenStream {
    let name = input.ident;
    quote! {
        impl ClientTrait for #name {}
    }
}
