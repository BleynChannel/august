use super::{
    generate_function::generate_function,
    utils::{get_attributes, get_externals, get_inputs, get_literal_type},
};

// use std::str::FromStr;

use proc_macro2::{Ident, TokenStream};
use quote::{format_ident, quote, ToTokens};
use syn::{Error, ItemFn, Result, ReturnType, Type, TypePath};

pub(crate) fn generate(ast: &ItemFn, attr: &TokenStream) -> Result<TokenStream> {
    let sig = &ast.sig;
    let ident = &sig.ident;

    let attrs = get_attributes(attr);

    let exts = get_externals(&sig.inputs[0]);
    let ins = get_inputs(sig.inputs.iter().skip(1));

    let exts_args = generate_externals_atributes(&exts);

    let name = generate_name(attrs.get("name"), &ident.to_string());
    //TODO: Внедрить описание функций в August
    // let description = generate_description(
    //     attrs.get("description"),
    //     &"Description is missing".to_string(),
    // );
    let inputs = generate_inputs(&ins)?;
    let output = generate_output(&sig.output)?;
    let externals = generate_externals(&exts);
    let function = generate_function(
        &exts,
        &ins,
        &sig.output,
        sig.inputs.to_token_stream(),
        ast.block.as_ref().to_token_stream(),
    );

    Ok(quote! {
        pub fn #ident(#exts_args) -> august_plugin_system::function::StdFunction {
            august_plugin_system::function::StdFunction::new(
                #name,
                #inputs,
                #output,
                #externals,
                #function
            )
        }
    })
}

fn generate_externals_atributes(exts: &Vec<(Ident, &Type)>) -> TokenStream {
    let exts: Vec<TokenStream> = exts
        .iter()
        .map(|(name, ty)| {
            let ty = match ty {
                Type::Reference(ref_ty) => &*ref_ty.elem,
                _ => panic!("Wrong type"),
            };

            quote! { #name: #ty }
        })
        .collect();

    quote! { #(#exts),* }
}

fn generate_name(name: Option<&String>, or: &String) -> TokenStream {
    let name = name.map(|x| x.clone()).unwrap_or(or.to_string());
    quote! { #name }
}

//TODO: Внедрить описание функций в August
// fn generate_description(description: Option<&String>, or: &String) -> TokenStream {
//     let description = description.map(|x| x.clone()).unwrap_or(or.to_string());
//     quote! { #description }
// }

fn generate_inputs(inputs: &Vec<(String, &Type)>) -> Result<TokenStream> {
    let mut result = vec![];

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

fn generate_externals(exts: &Vec<(Ident, &Type)>) -> TokenStream {
    let exts: Vec<TokenStream> = exts
        .iter()
        .map(|(name, _)| {
            quote! { Box::new(#name) as Box<dyn core::any::Any + Send + Sync> }
        })
        .collect();

    quote! { vec![#(#exts),*] }
}

fn generate_arg(name: &String, ty: &Type) -> Result<TokenStream> {
    let ty = get_variable_type_path(get_literal_type(ty))?;
    Ok(quote! { august_plugin_system::function::Arg::new(#name, #ty) })
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
            let token = format_ident!("{}", token);
            Ok(quote! { august_plugin_system::variable::VariableType::#token })
        }
        None => Err(Error::new_spanned(path, "type is not supported")),
    }
}
