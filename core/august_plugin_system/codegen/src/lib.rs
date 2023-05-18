mod function;

use syn::{parse_macro_input, DeriveInput};

#[proc_macro_derive(Function, attributes(output))]
pub fn derive_function(input: proc_macro::TokenStream) -> proc_macro::TokenStream {
    let ast = parse_macro_input!(input as DeriveInput);

    function::derive(&ast)
        .unwrap_or_else(|e| e.to_compile_error())
        .into()
}
