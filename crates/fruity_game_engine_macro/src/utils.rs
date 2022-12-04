use quote::quote;
use syn::__private::TokenStream2;

pub fn fruity_crate() -> TokenStream2 {
    let crate_name = std::env::var("CARGO_PKG_NAME").unwrap();

    if crate_name == "fruity_game_engine" {
        quote! { crate }
    } else {
        quote! { ::fruity_game_engine }
    }
}
