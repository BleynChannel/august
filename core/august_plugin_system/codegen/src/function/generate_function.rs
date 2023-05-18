use std::str::FromStr;

use proc_macro2::{Ident, TokenStream};
use quote::{format_ident, quote};
use syn::{Field, Result, TypePath};

use crate::function::utils::get_literal_type;

pub(crate) fn generate_function(
    inputs: &Vec<(usize, &Field)>,
    output: &Option<(usize, &Field)>,
) -> Result<TokenStream> {
    let args = serialize_args(inputs);
    let run = function_run(inputs, output);
    let output = return_output(output);

    Ok(quote! {
        move |args: &[august_plugin_system::variable::VariableData]| ->
            august_plugin_system::utils::FunctionResult<Option<august_plugin_system::variable::VariableData>> {
            #args
            #run
            #output
        }
    })
}

fn serialize_args(inputs: &Vec<(usize, &Field)>) -> TokenStream {
    let args: Vec<TokenStream> = inputs
        .iter()
        .enumerate()
        .map(|(index, (_, field))| {
            let var_name = format_ident!("tmp_{}", index);
            let ty = get_literal_type(*field);

            quote! { let #var_name: #ty = args[#index].parse()?; }
        })
        .collect();

    quote! {
        #(#args)
        *
    }
}

fn function_run(inputs: &Vec<(usize, &Field)>, output: &Option<(usize, &Field)>) -> TokenStream {
    let output_token = output.map(|(_, output)| {
        let ty = get_literal_type(output);
        quote! { let result: #ty = }
    });

    let inputs_token: Vec<Ident> = inputs
        .iter()
        .enumerate()
        .map(|(index, _)| format_ident!("tmp_{}", index))
        .collect();

    quote! { #output_token Self::run(#(#inputs_token),*); }
}

fn return_output(output: &Option<(usize, &Field)>) -> TokenStream {
    output.map_or(quote! { Ok(None) }, |(_, field)| {
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
            format!("august_plugin_system::variable::VariableData::{}", *token).as_str(),
        )
        .unwrap();

		quote! { #ser_token (result) }
    } else if type_name == "Vec" {
		quote! { august_plugin_system::variable::VariableData::List(result.iter().map(|item| (*item).into()).collect()) }
	} else if type_name == "VariableData" {
		quote! { result }
	} else {
		TokenStream::new()
	}
}
