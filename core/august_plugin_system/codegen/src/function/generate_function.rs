use std::str::FromStr;

use proc_macro2::TokenStream;
use quote::quote;
use syn::{ReturnType, Type, TypePath};

use crate::function::utils::get_literal_type;

pub(crate) fn generate_function(
    externals: &Vec<&Type>,
    inputs: &Vec<(String, &Type)>,
    output: &ReturnType,
    args: TokenStream,
    block: TokenStream,
) -> TokenStream {
    let exts = serialize_exts(externals);
    let ins = serialize_inputs(inputs);
    let call = function_call(exts, ins, output);
    let out = return_output(output);

    quote! {
        move |exts, args| -> august_plugin_system::utils::FunctionResult<Option<august_plugin_system::variable::Variable>> {
            let func = move |#args| #output #block;
            #call
            #out
        }
    }
}

fn serialize_exts(externals: &Vec<&Type>) -> TokenStream {
    let exts: Vec<TokenStream> = (0..externals.len())
        .map(|index| {
            quote! { exts[#index].downcast_ref().ok_or("Failed to downcast")? }
        })
        .collect();

    quote! { (#(#exts), *) }
}

fn serialize_inputs(inputs: &Vec<(String, &Type)>) -> TokenStream {
    let args: Vec<TokenStream> = (0..inputs.len())
        .map(|index| quote! { args[#index].parse()? })
        .collect();

    quote! { (#(#args), *) }
}

fn function_call(exts: TokenStream, args: TokenStream, output: &ReturnType) -> TokenStream {
    let output_token = match output {
        syn::ReturnType::Default => None,
        syn::ReturnType::Type(_, _) => Some(quote! { let result = }),
    };

    quote! { #output_token func(#exts, #args); }
}

fn return_output(output: &ReturnType) -> TokenStream {
    match output {
        syn::ReturnType::Default => quote! { Ok(None) },
        syn::ReturnType::Type(_, ty) => {
            let result = serialize_output(get_literal_type(&*ty));
            quote! { Ok(Some(#result)) }
        }
    }
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
