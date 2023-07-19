use super::{
    generate_function::generate_function,
    utils::{get_externals_inputs_output_fields, get_literal_type},
};

use std::str::FromStr;

use proc_macro2::TokenStream;
use quote::{format_ident, quote};
use syn::{Data, DataStruct, DeriveInput, Error, Field, Ident, Result, TypePath};

pub(crate) fn generate(ast: &DeriveInput) -> Result<TokenStream> {
    Ok(match &ast.data {
        Data::Struct(struct_ast) => generate_struct(&ast.ident, struct_ast)?,
        _ => TokenStream::new(),
    })
}

fn generate_struct(struct_ident: &Ident, struct_ast: &DataStruct) -> Result<TokenStream> {
    let ident = format_ident!("{}", struct_ident);

    let (exts, ins, out) = get_externals_inputs_output_fields(&struct_ast.fields)?;

    let name = generate_struct_name(struct_ident)?;
    let description = generate_struct_description(struct_ast)?;
    let inputs = generate_struct_inputs(&ins)?;
    let output = generate_struct_output(&out)?;
    let function = generate_function(&exts, &ins, &out);

    Ok(quote! {
        impl #ident {
            fn as_function() -> august_plugin_system::function::Function {
                august_plugin_system::function::Function::new(
                    #name,
                    #description,
                    #inputs,
                    #output,
                    #function
                )
            }
        }
    })
}

fn generate_struct_name(struct_ident: &Ident) -> Result<TokenStream> {
    let name = struct_ident.to_string().to_lowercase();
    Ok(quote! { #name.to_string() })
}

fn generate_struct_description(_struct_ast: &DataStruct) -> Result<TokenStream> {
    //TODO: Сделать описание
    let description = "It's plugin";
    Ok(quote! { #description.to_string() })
}

fn build_arg(index: &usize, field: &Field) -> Result<TokenStream> {
    let ident = field.ident.as_ref();

    let name = match ident {
        Some(ident) => ident.to_string(),
        None => format!("arg_{}", index),
    };

    let ty = get_variable_type_path(get_literal_type(field))?;

    Ok(quote! {
        august_plugin_system::function::Arg::new(#name.to_string(), #ty)
    })
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

    match VARIABLE_TYPES.iter().find(|(name, _)| **name == ident) {
        Some((_, token)) => Ok(TokenStream::from_str(
            format!("august_plugin_system::variable::VariableType::{}", *token).as_str(),
        )
        .unwrap()),
        None => Err(Error::new_spanned(path, "type is not supported")),
    }
}

fn generate_struct_inputs(inputs: &Vec<&Field>) -> Result<TokenStream> {
    let mut result = Vec::new();

    for (index, field) in inputs.iter().enumerate() {
        if let Some(attr) = field.attrs.first() {
            let attr_name = attr.path().get_ident().unwrap().to_string();

            if attr_name == "output" {
                continue;
            }
        }

        result.push(build_arg(&index, *field)?);
    }

    Ok(quote! { vec![#(#result),*] })
}

fn generate_struct_output(output: &Option<&Field>) -> Result<TokenStream> {
    match output {
        Some(field) => {
            let arg = build_arg(&1, *field)?;
            Ok(quote! { Some(#arg) })
        }
        None => Ok(quote! { None }),
    }
}
