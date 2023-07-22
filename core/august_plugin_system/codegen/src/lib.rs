mod function;

use proc_macro::TokenStream;
use syn::{parse_macro_input, DeriveInput, ItemFn};

#[proc_macro_derive(Function, attributes(output, external))]
pub fn derive_function(input: TokenStream) -> TokenStream {
    let ast = parse_macro_input!(input as DeriveInput);

    function::derive(&ast)
        .unwrap_or_else(|e| e.to_compile_error())
        .into()
}

#[proc_macro_attribute]
pub fn function(attr: TokenStream, input: TokenStream) -> TokenStream {
	let ast = parse_macro_input!(input as ItemFn);

	function::derive_2(ast, attr.into())
        .unwrap_or_else(|e| e.to_compile_error())
        .into()
}