use super::{
    generate_function::generate_function,
    utils::{get_externals_inputs, get_literal_type},
};

use std::{collections::HashMap, str::FromStr};

use proc_macro2::TokenStream;
use quote::{quote, ToTokens};
use syn::{Error, ItemFn, Result, ReturnType, Type, TypePath};

pub(crate) fn generate(ast: &ItemFn, attr: &TokenStream) -> Result<TokenStream> {
    let sig = &ast.sig;
    let ident = &sig.ident;

    let attrs = generate_attributes(attr)?;
    let (exts, ins) = get_externals_inputs(&sig.inputs)?;

    let name = generate_name(attrs.get("name"), &ident.to_string())?;
    let description = generate_description(
        attrs.get("description"),
        &"Description is missing".to_string(),
    )?;
    let inputs = generate_inputs(&ins)?;
    let output = generate_output(&sig.output)?;
    let function = generate_function(
        &exts,
        &ins,
        &sig.output,
        sig.inputs.to_token_stream(),
        ast.block.as_ref().to_token_stream(),
    );

    Ok(quote! {
        pub fn #ident() -> august_plugin_system::function::Function {
            august_plugin_system::function::Function::new(
                #name,
                #description,
                #inputs,
                #output,
                #function
            )
        }
    })
}

fn generate_attributes(attr: &TokenStream) -> Result<HashMap<String, String>> {
    let attrs_str = attr.to_string();
    match attrs_str.is_empty() {
        true => Ok(HashMap::new()),
        false => Ok(attrs_str
            .split(',')
            .map(|attr| {
                let attr: Vec<&str> = attr.split('=').map(|token| token.trim()).collect();
                (attr[0].to_string(), attr[1].trim_matches('"').to_string())
            })
            .collect()),
    }
}

fn generate_name(name: Option<&String>, or: &String) -> Result<TokenStream> {
    let name = name.map(|x| x.clone()).unwrap_or(or.to_string());
    Ok(quote! { #name.to_string() })
}

fn generate_description(description: Option<&String>, or: &String) -> Result<TokenStream> {
    let description = description.map(|x| x.clone()).unwrap_or(or.to_string());
    Ok(quote! { #description.to_string() })
}

fn generate_inputs(inputs: &Vec<(String, &Type)>) -> Result<TokenStream> {
    let mut result = Vec::new();

    for (name, ty) in inputs {
        result.push(generate_arg(name, *ty)?);
    }

    Ok(quote! { vec![#(#result),*] })
}

fn generate_output(output: &ReturnType) -> Result<TokenStream> {
    match output {
        syn::ReturnType::Default => Ok(quote! { None }),
        syn::ReturnType::Type(_, ty) => {
            let arg = generate_arg(&"output".to_string(), &*ty)?;
            Ok(quote! { Some(#arg) })
        }
    }
}

fn generate_arg(name: &String, ty: &Type) -> Result<TokenStream> {
    let ty = get_variable_type_path(get_literal_type(ty))?;
    Ok(quote! { august_plugin_system::function::Arg::new(#name.to_string(), #ty) })
}

const VARIABLE_TYPES: [(&str, &str); 15] = [
    ("i8", "I8"),
    ("i16", "I16"),
    ("i32", "I32"),
    ("i64", "I64"),
    ("u8", "U8"),
    ("u16", "U16"),
    ("u32", "U32"),
    ("u64", "U64"),
    ("f32", "F32"),
    ("f64", "F64"),
    ("bool", "Bool"),
    ("char", "Char"),
    ("String", "String"),
    ("Vec", "List"),
    ("Variable", "Let"),
];

fn get_variable_type_path(path: &TypePath) -> Result<TokenStream> {
    let ident = path.path.segments.last().unwrap().ident.to_string();

    match VARIABLE_TYPES.into_iter().find(|(name, _)| **name == ident) {
        Some((_, token)) => {
            let token = TokenStream::from_str(token).unwrap();
            Ok(quote! { august_plugin_system::variable::VariableType::#token })
        }
        None => Err(Error::new_spanned(path, "type is not supported")),
    }
}
