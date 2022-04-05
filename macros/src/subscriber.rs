use std::error::Error;

use proc_macro2::TokenStream;
use quote::quote;
use syn::{DeriveInput, Lit, Meta, NestedMeta};

pub fn subscriber_expand(input: DeriveInput) -> Result<TokenStream, Box<dyn Error + Send + Sync>> {
    let name = input.ident;
    let attr = input
        .attrs
        .into_iter()
        .find(|v| v.path.is_ident("graphql_ws"))
        .ok_or("helper attribute for graphql_ws not found")
        .unwrap();
    let attr = attr.parse_meta().unwrap();
    let url: TokenStream = match attr {
        Meta::List(v) => v
            .nested
            .into_iter()
            .find_map(|v| {
                if let NestedMeta::Meta(Meta::NameValue(v)) = v {
                    if v.path.is_ident("url") {
                        if let Lit::Str(v) = v.lit {
                            Some(v.value())
                        } else {
                            None
                        }
                    } else {
                        None
                    }
                } else {
                    None
                }
            })
            .ok_or("url for graphql_ws subscriber not found")
            .unwrap(),
        _other => {
            panic!("graphql_ws attribute must be a list")
        }
    }
    .parse()
    .unwrap();
    Ok(quote! {
        impl Subscriber for #name{
            fn url(&self)->String{
                #url
            }
        }
    })
}
