extern crate proc_macro;

use proc_macro::TokenStream;

#[proc_macro_derive(FromRow)]
pub fn derive_fn_from_row(_items: TokenStream) -> TokenStream {
    //"pub fn dummy_from_row() -> &'static str { \"dummy\" }"
    "/* dummy */".parse().unwrap()
}
#[proc_macro_derive(Type)]
pub fn derive_fn_type(_items: TokenStream) -> TokenStream {
    //"pub fn dummy_type() -> &'static str { \"dummy\" }"
    "/* dummy */".parse().unwrap()
}

#[proc_macro_attribute]
pub fn sqlx(_attr: TokenStream, item: TokenStream) -> TokenStream {
    item
}
