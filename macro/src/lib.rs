extern crate proc_macro;
extern crate quote;
extern crate syn;

use proc_macro::TokenStream;

use gauss_core::{Matrix, FloatMatrix};


#[proc_macro]
pub fn example(input: TokenStream) -> TokenStream {

    //let _ast = syn::parse_macro_input!(input as syn::DeriveInput);
    
    "println!(\"Hello World!\");".parse().unwrap()
}