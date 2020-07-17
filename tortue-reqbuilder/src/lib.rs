use proc_macro::TokenStream;


#[proc_macro_derive(IntoRequest)]
pub fn derive_request(_item: TokenStream) -> TokenStream {



    "".parse().unwrap()
}

#[proc_macro_derive(IntoResponse)]
pub fn derive_response(_item: TokenStream) -> TokenStream {



    "".parse().unwrap()
}