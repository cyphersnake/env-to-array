#![doc = include_str!("../README.md")]

use itertools::Itertools;
use proc_macro::TokenStream;
use syn::parse_macro_input;

fn slice_to_array_token(input: &[u8]) -> TokenStream {
    format!("[{}]", input.iter().join(", "))
        .parse::<proc_macro2::TokenStream>()
        .expect("Failed to parse array")
        .into()
}

#[cfg(feature = "bs58")]
/// Get from variable string, decode it from bs58 and write array as result
/// ```
/// const ID: [u8; 65] = env_to_array::bs58_to_array!("7Ax7AxYSahRegVSuU76JGWNxzdwVAPpaonY26V6JH17ToUQYSahRegVSuU76JGWNxzdwVAPpaonY26V6JH17ToUQ");
/// ```
#[proc_macro]
pub fn bs58_to_array(input: TokenStream) -> TokenStream {
    slice_to_array_token(
        bs58::decode(parse_macro_input!(input as syn::LitStr).value())
            .into_vec()
            .expect("Can't decode bs58")
            .as_slice(),
    )
}

#[cfg(feature = "bs64")]
/// Get from variable string, decode it from bs64 and write array as result
/// ```
/// const ID: [u8; 29] = env_to_array::bs64_to_array!("W7MmhbfqLQc4LbN0TUPfiflxSO6uVZ7E0NHueJ0=");
/// ```
#[proc_macro]
pub fn bs64_to_array(input: TokenStream) -> TokenStream {
    slice_to_array_token(
        base64::decode(parse_macro_input!(input as syn::LitStr).value())
            .expect("Can't decode bs64")
            .as_slice(),
    )
}

#[cfg(feature = "hex")]
/// Get value from variable string, decode it from hex and write array as result
/// ```
/// const ID: [u8; 31] = env_to_array::hex_to_array!("5bb32685b7ea2d07382db3744d43df89f97148eeae559ec4d0d1feefa2ee78");
/// ```
#[proc_macro]
pub fn hex_to_array(input: TokenStream) -> TokenStream {
    slice_to_array_token(
        hex::decode(parse_macro_input!(input as syn::LitStr).value())
            .expect("Can't decode hex")
            .as_slice(),
    )
}

#[cfg(feature = "bs58")]
/// Get from env variable string, decode it from bs58 and write array as result
/// ```
/// const ID: [u8; 17] = env_to_array::bs58_env_to_array!("_ENV_TO_ARRAY_BS58");
/// ```
#[proc_macro]
pub fn bs58_env_to_array(input: TokenStream) -> TokenStream {
    slice_to_array_token(
        bs58::decode(
            std::env::var(parse_macro_input!(input as syn::LitStr).value()).expect("Env variable"),
        )
        .into_vec()
        .expect("Can't decode bs58")
        .as_slice(),
    )
}

#[cfg(feature = "bs64")]
/// Get from env variable string, decode it from bs64 and write array as result
/// ```
/// const ID: [u8; 32] = env_to_array::bs64_env_to_array!("_ENV_TO_ARRAY_BS64");
/// ```
#[proc_macro]
pub fn bs64_env_to_array(input: TokenStream) -> TokenStream {
    slice_to_array_token(
        base64::decode(
            std::env::var(parse_macro_input!(input as syn::LitStr).value())
                .expect("This env not found"),
        )
        .expect("Can't decode bs64")
        .as_slice(),
    )
}

#[cfg(feature = "hex")]
/// Get from env variable string, decode it from hex and write array as result
/// ```
/// const ID: [u8; 64] = env_to_array::hex_env_to_array!("_ENV_TO_ARRAY_HEX");
/// ```
#[proc_macro]
pub fn hex_env_to_array(input: TokenStream) -> TokenStream {
    slice_to_array_token(
        hex::decode(
            std::env::var(parse_macro_input!(input as syn::LitStr).value())
                .expect("This env not found"),
        )
        .expect("Can't decode hex")
        .as_slice(),
    )
}

#[cfg(feature = "bs32")]
/// Get from variable string, decode it from bs58 and write array as result
/// ```
/// const ID: [u8; 5] = env_to_array::bs32_to_array!("Z0Z0Z0Z0");
/// ```
#[proc_macro]
pub fn bs32_to_array(input: TokenStream) -> TokenStream {
    slice_to_array_token(
        base32::decode(
            base32::Alphabet::Crockford,
            &parse_macro_input!(input as syn::LitStr).value(),
        )
        .expect("Can't decode bs32")
        .as_slice(),
    )
}

#[cfg(feature = "bs32")]
/// Get from env variable string, decode it from bs58 and write array as result
/// ```
/// const ID: [u8; 5] = env_to_array::bs32_env_to_array!("_ENV_TO_ARRAY_BS32");
/// ```
#[proc_macro]
pub fn bs32_env_to_array(input: TokenStream) -> TokenStream {
    slice_to_array_token(
        base32::decode(
            base32::Alphabet::Crockford,
            &std::env::var(parse_macro_input!(input as syn::LitStr).value()).expect("Env variable"),
        )
        .expect("Can't decode bs32")
        .as_slice(),
    )
}

#[cfg(feature = "bs85")]
/// Get from variable string, decode it from bs58 and write array as result
/// ```
/// const ID: [u8; 7] = env_to_array::bs85_to_array!("VPRomVPRn");
/// ```
#[proc_macro]
pub fn bs85_to_array(input: TokenStream) -> TokenStream {
    slice_to_array_token(
        base85::decode(&parse_macro_input!(input as syn::LitStr).value())
            .expect("Can't decode bs85")
            .as_slice(),
    )
}

#[cfg(feature = "bs85")]
/// Get from env variable string, decode it from bs58 and write array as result
/// ```
/// const ID: [u8; 7] = env_to_array::bs85_env_to_array!("_ENV_TO_ARRAY_BS85");
/// ```
#[proc_macro]
pub fn bs85_env_to_array(input: TokenStream) -> TokenStream {
    slice_to_array_token(
        base85::decode(
            &std::env::var(parse_macro_input!(input as syn::LitStr).value()).expect("Env variable"),
        )
        .expect("Can't decode bs85")
        .as_slice(),
    )
}
