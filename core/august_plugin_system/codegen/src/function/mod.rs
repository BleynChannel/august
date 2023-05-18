use syn::{DeriveInput, Result};

mod utils;
mod validator;
mod generator;
mod generate_function;

pub fn derive(ast: &DeriveInput) -> Result<proc_macro2::TokenStream> {
    validator::validate(ast)?;
    Ok(generator::generate(ast)?)
}