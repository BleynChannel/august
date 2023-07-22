use proc_macro2::TokenStream;
use syn::{DeriveInput, Result, ItemFn};

mod generate_function;
mod generator;
mod utils;
mod validator;

pub fn derive(ast: &DeriveInput) -> Result<TokenStream> {
    validator::validate(ast)?;
    Ok(generator::generate(ast)?)
}

pub fn derive_2(ast: ItemFn, attr: TokenStream) -> Result<TokenStream> {
    validator::validate_2(&ast, &attr)?;
    Ok(generator::generate_2(&ast, &attr)?)
}