use syn::{punctuated::Punctuated, token::Comma, FnArg, Pat, Result, Type, TypePath};

pub(crate) fn get_literal_type(ty: &Type) -> &TypePath {
    match ty {
        Type::Path(ty) => ty,
        _ => panic!("Wrong type"),
    }
}

pub(crate) fn get_externals_inputs(
    args: &Punctuated<FnArg, Comma>,
) -> Result<(Vec<&Type>, Vec<(String, &Type)>)> {
    let exts = match args[0] {
        FnArg::Receiver(_) => panic!("Receiver is not supported"),
        FnArg::Typed(ref pat) => match pat.ty.as_ref() {
            Type::Tuple(ty) => ty.elems.iter().collect(),
            ty => vec![ty],
        },
    };

    let inputs = match args[1] {
        FnArg::Receiver(_) => panic!("Receiver is not supported"),
        FnArg::Typed(ref pat) => match pat.ty.as_ref() {
            Type::Tuple(ty) => match pat.pat.as_ref() {
                Pat::Tuple(pat) => pat
                    .elems
                    .iter()
                    .scan(0, |acc, pat| {
                        Some(match pat_to_string(pat) {
                            Some(str) => str,
                            None => {
                                *acc += 1;
                                format!("arg_{acc}")
                            }
                        })
                    })
                    .zip(ty.elems.iter())
                    .collect(),
                _ => panic!("Wrong type"),
            },
            ty => vec![(pat_to_string(&*pat.pat).unwrap_or("arg".to_string()), ty)],
        },
    };

    Ok((exts, inputs))
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
