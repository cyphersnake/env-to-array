use itertools::Itertools;
use proc_macro::TokenStream;
use syn::parse_macro_input;

#[cfg(feature = "bs58")]
/// Get from env variable string, decode it from bs58 and write array as result
/// ```
/// const ID: [u8; 32] = env_to_array::bs58_env_to_array!("_ENV_TO_ARRAY_BS58");
/// ```
#[proc_macro]
pub fn bs58_env_to_array(input: TokenStream) -> TokenStream {
    format!(
        "[{}]",
        bs58::decode(
            std::env::var(parse_macro_input!(input as syn::LitStr).value()).expect("Env variable")
        )
        .into_vec()
        .expect("Can't decode bs58")
        .into_iter()
        .join(", ")
    )
    .parse::<proc_macro2::TokenStream>()
    .unwrap()
    .into()
}

#[cfg(feature = "bs64")]
/// Get from env variable string, decode it from bs64 and write array as result
/// ```
/// const ID: [u8; 32] = env_to_array::bs64_env_to_array!("_ENV_TO_ARRAY_BS64");
/// ```
#[proc_macro]
pub fn bs64_env_to_array(input: TokenStream) -> TokenStream {
    format!(
        "[{}]",
        base64::decode(
            std::env::var(parse_macro_input!(input as syn::LitStr).value())
                .expect("This env not found")
        )
        .expect("Can't decode bs64")
        .into_iter()
        .join(", ")
    )
    .parse::<proc_macro2::TokenStream>()
    .unwrap()
    .into()
}

#[cfg(feature = "hex")]
/// Get from env variable string, decode it from hex and write array as result
/// ```
/// const ID: [u8; 32] = env_to_array::hex_env_to_array!("_ENV_TO_ARRAY_HEX");
/// ```
#[proc_macro]
pub fn hex_env_to_array(input: TokenStream) -> TokenStream {
    format!(
        "[{}]",
        hex::decode(
            std::env::var(parse_macro_input!(input as syn::LitStr).value())
                .expect("This env not found")
        )
        .expect("Can't decode hex")
        .into_iter()
        .join(", ")
    )
    .parse::<proc_macro2::TokenStream>()
    .unwrap()
    .into()
}
