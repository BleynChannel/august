use syn::{DeriveInput, Result};

mod generate_function;
mod generator;
mod utils;
mod validator;

pub fn derive(ast: &DeriveInput) -> Result<proc_macro2::TokenStream> {
    validator::validate(ast)?;
    Ok(generator::generate(ast)?)
}
