use proc_macro::TokenStream;
use quote::ToTokens;
use syn::{parse_macro_input, ItemFn};

#[proc_macro_attribute]
pub fn trancition(args: TokenStream, input: TokenStream) -> TokenStream {
    eprintln!("{:#?}", args);
    let st = parse_macro_input!(input as ItemFn);
    st.to_token_stream().into()
}
