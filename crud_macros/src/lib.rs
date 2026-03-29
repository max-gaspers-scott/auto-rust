extern crate proc_macro;

use proc_macro::TokenStream;

use quote::quote;

#[proc_macro]
pub fn select_all(_input: TokenStream) -> TokenStream {
    quote! {
        fn select_all() -> i32 {
            42
        }
    }
    .into()
}

