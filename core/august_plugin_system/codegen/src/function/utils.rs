use std::collections::HashMap;

use proc_macro2::{Ident, Span, TokenStream};
use quote::format_ident;
use syn::{FnArg, Pat, Type, TypePath};

pub(crate) fn get_literal_type(ty: &Type) -> &TypePath {
    match ty {
        Type::Path(path) => path,
        Type::Reference(r) => match &*r.elem {
            Type::Path(path) => path,
            _ => panic!("Wrong type"),
        },
        _ => panic!("Wrong type"),
    }
}

pub(crate) fn get_attributes(attr: &TokenStream) -> HashMap<String, String> {
    let attrs_str = attr.to_string();
    match attrs_str.is_empty() {
        true => HashMap::new(),
        false => attrs_str
            .split(',')
            .map(|attr| {
                let attr: Vec<&str> = attr.split('=').map(|token| token.trim()).collect();
                (attr[0].to_string(), attr[1].trim_matches('"').to_string())
            })
            .collect(),
    }
}

pub(crate) fn get_externals(arg: &FnArg) -> Vec<(Ident, &Type)> {
    match arg {
        FnArg::Receiver(_) => panic!("Receiver is not supported"),
        FnArg::Typed(pat) => match &*pat.ty {
            Type::Tuple(tuple) => tuple
                .elems
                .iter()
                .enumerate()
                .map(|(index, ty)| (format_ident!("ext_{}", index), ty))
                .collect(),
            ty => vec![(Ident::new("ext_0", Span::call_site()), ty)],
        },
    }
}

pub(crate) fn get_inputs<'a, I>(args: I) -> Vec<(String, &'a Type)>
where
    I: Iterator<Item = &'a FnArg>,
{
    args.map(|arg| match arg {
        FnArg::Receiver(_) => panic!("Receiver is not supported"),
        FnArg::Typed(pat) => (
            pat_to_string(&*pat.pat).unwrap_or("arg".to_string()),
            pat.ty.as_ref(),
        ),
    })
    .collect()
}

fn pat_to_string(pat: &Pat) -> Option<String> {
    match pat {
        Pat::Const(_) => None,
        Pat::Ident(pat) => Some(pat.ident.to_string()),
        Pat::Lit(_) => None,
        Pat::Macro(_) => None,
        Pat::Or(_) => None,
        Pat::Paren(_) => None,
        Pat::Path(pat) => Some(pat.path.get_ident().unwrap().to_string()),
        Pat::Range(_) => None,
        Pat::Reference(pat) => pat_to_string(&pat.pat),
        Pat::Rest(_) => None,
        Pat::Type(pat) => pat_to_string(&pat.pat),
        Pat::Wild(_) => None,
        _ => panic!("Wrong type"),
    }
}

pub(crate) fn clear_ref(ty: &Type) -> Type {
    match ty {
        Type::Path(path) => {
            let mut path = path.clone();
            match &mut path.path.segments.last_mut().unwrap().arguments {
                syn::PathArguments::AngleBracketed(args) => {
                    args.args.iter_mut().for_each(|arg| match arg {
                        syn::GenericArgument::Type(ty) => {
                            *arg = syn::GenericArgument::Type(clear_ref(ty))
                        }
                        _ => panic!("Wrong type"),
                    });
                }
                _ => panic!("Wrong type"),
            }
            Type::Path(path)
        }
        Type::Reference(r) => match &*r.elem {
            Type::Path(path) => Type::Path(path.clone()),
            _ => panic!("Wrong type"),
        },
        _ => panic!("Wrong type"),
    }
}
