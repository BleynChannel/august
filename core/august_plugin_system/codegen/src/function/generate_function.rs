use std::str::FromStr;

use proc_macro2::TokenStream;
use quote::quote;
use syn::{Field, TypePath};

use crate::function::utils::get_literal_type;

pub(crate) fn generate_function(
    externals: &Vec<&Field>,
    inputs: &Vec<&Field>,
    output: &Option<&Field>,
) -> TokenStream {
    let exts = serialize_exts(externals);
    let args = serialize_args(inputs);
    let call = function_call(exts, args, output);
    let output = return_output(output);

    quote! {
        move |exts, args| ->
            august_plugin_system::utils::FunctionResult<Option<august_plugin_system::variable::Variable>> {
            #call
            #output
        }
    }
}

fn serialize_exts(externals: &Vec<&Field>) -> TokenStream {
    let exts: Vec<TokenStream> = externals
        .iter()
        .enumerate()
        .map(|(index, _)| {
            quote! { exts[#index].downcast_ref().ok_or("Failed to downcast")? }
        })
        .collect();

    quote! { (#(#exts), *) }
}

fn serialize_args(inputs: &Vec<&Field>) -> TokenStream {
    let args: Vec<TokenStream> = inputs
        .iter()
        .enumerate()
        .map(|(index, _)| quote! { args[#index].parse()? })
        .collect();

    quote! { (#(#args), *) }
}

fn function_call(exts: TokenStream, args: TokenStream, output: &Option<&Field>) -> TokenStream {
    let output_token = output.map(|output| {
        let ty = get_literal_type(output);
        quote! { let result: #ty = }
    });

    quote! { #output_token Self::call(#exts, #args); }
}

fn return_output(output: &Option<&Field>) -> TokenStream {
    output.map_or(quote! { Ok(None) }, |field| {
        let result = serialize_output(get_literal_type(field));

        quote! { Ok(Some(#result)) }
    })
}

const VARIABLE_DATAS: [(&str, &str); 13] = [
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
];

fn serialize_output(ty: &TypePath) -> TokenStream {
    let type_name = ty.path.segments.last().unwrap().ident.to_string();

    if let Some((_, token)) = VARIABLE_DATAS.iter().find(|(name, _)| **name == type_name) {
        let ser_token = TokenStream::from_str(
            format!("august_plugin_system::variable::Variable::{}", *token).as_str(),
        )
        .unwrap();

        quote! { #ser_token (result) }
    } else if type_name == "Vec" {
        quote! { august_plugin_system::variable::Variable::List(result.into_iter().map(|item| item.into()).collect()) }
    } else if type_name == "Variable" {
        quote! { result }
    } else {
        TokenStream::new()
    }
}
