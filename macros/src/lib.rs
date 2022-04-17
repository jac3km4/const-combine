use proc_macro::TokenStream;
use quote::{quote, ToTokens};
use syn::{parse_macro_input, Expr, LitStr};

#[proc_macro]
pub fn str_len(item: TokenStream) -> TokenStream {
    match syn::parse::<LitStr>(item.clone()) {
        Ok(lit) => {
            let len = lit.value().len();
            quote!(#len).into_token_stream().into()
        }
        Err(_) => {
            let exp: Expr = parse_macro_input!(item);
            quote!(#exp.len()).into_token_stream().into()
        }
    }
}
