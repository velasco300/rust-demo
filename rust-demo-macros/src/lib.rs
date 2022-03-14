mod derives;

use crate::derives::generate_vo::expand_derive_generate_vo;
use proc_macro::TokenStream;
use quote::ToTokens;
use syn::{parse_macro_input, DeriveInput, ItemFn};

#[proc_macro_attribute]
pub fn trancition(args: TokenStream, input: TokenStream) -> TokenStream {
    eprintln!("{:#?}", args);
    let st = parse_macro_input!(input as ItemFn);
    eprintln!("{:#?}", st);
    st.to_token_stream().into()
}

#[proc_macro_derive(GenerateVO)]
pub fn derive_generate_vo(input: TokenStream) -> TokenStream {
    let ts = parse_macro_input!(input as DeriveInput);
    match expand_derive_generate_vo(&ts) {
        Ok(token_stream) => token_stream.into(),
        Err(e) => e.to_compile_error().into(),
    }
}
