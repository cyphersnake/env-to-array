use itertools::Itertools;
use proc_macro::TokenStream;
use syn::parse_macro_input;

#[cfg(feature = "bs58")]
#[proc_macro]
pub fn bs58_env_to_array(input: TokenStream) -> TokenStream {
    let bytes = format!(
        "[{}]",
        bs58::decode(
            std::env::var(parse_macro_input!(input as syn::LitStr).value())
                .expect("This env not found")
        )
        .into_vec()
        .expect("Can't decode bs58")
        .into_iter()
        .join(", ")
    );
    TokenStream::from(quote::quote! {
        #bytes
    })
}

#[cfg(feature = "bs64")]
#[proc_macro]
pub fn bs64_env_to_array(input: TokenStream) -> TokenStream {
    let bytes = format!(
        "[{}]",
        base64::decode(
            std::env::var(parse_macro_input!(input as syn::LitStr).value())
                .expect("This env not found")
        )
        .expect("Can't decode bs64")
        .into_iter()
        .join(", ")
    );
    TokenStream::from(quote::quote! {
        #bytes
    })
}

#[cfg(feature = "hex")]
#[proc_macro]
pub fn hex_env_to_array(input: TokenStream) -> TokenStream {
    let bytes = format!(
        "[{}]",
        hex::decode(
            std::env::var(parse_macro_input!(input as syn::LitStr).value())
                .expect("This env not found")
        )
        .expect("Can't decode hex")
        .into_iter()
        .join(", ")
    );
    TokenStream::from(quote::quote! {
        #bytes
    })
}
