use proc_macro2::TokenStream;
use syn::{
    Data, DataStruct, DeriveInput, Error, Field, FnArg, GenericArgument, ItemFn, PathArguments,
    Result, Signature, Type, TypePath,
};

pub(crate) fn validate(ast: &DeriveInput) -> Result<()> {
    if !ast.generics.params.is_empty() {
        return Err(Error::new_spanned(ast, "generics are not supported"));
    }

    match &ast.data {
        Data::Struct(ast) => validate_struct(ast),
        _ => Err(Error::new_spanned(
            ast,
            "enum or union as functions are not supported",
        )),
    }
}

pub(crate) fn validate_2(ast: &ItemFn, attr: &TokenStream) -> Result<()> {
    if !ast.sig.generics.params.is_empty() {
        return Err(Error::new_spanned(ast, "generics are not supported"));
    }

    validate_attributes(attr)?;
    validate_function(&ast.sig)
}

const VALIDATE_ATTRIBUTES: [&str; 2] = ["name", "description"];
const VALIDATE_STRING_ATTRIBUTES: [&str; 2] = ["name", "description"];

fn validate_attributes(attrs: &TokenStream) -> Result<()> {
    let attrs_str = attrs.to_string();
    if !attrs_str.is_empty() {
        for attr in attrs_str.split(',') {
            let attr: Vec<&str> = attr.split('=').map(|token| token.trim()).collect();

            if attr.len() != 2 {
                return Err(Error::new_spanned(
                    attrs,
                    "attributes must have the format `path = data`",
                ));
            }

            let path = attr[0];

            if !VALIDATE_ATTRIBUTES.iter().any(|attr| *attr == path) {
                return Err(Error::new_spanned(
                    attrs,
                    format!("attribute `{}` does not exist", path),
                ));
            }

            if VALIDATE_STRING_ATTRIBUTES.iter().any(|attr| *attr == path) {
                let data: Vec<char> = attr[1].chars().collect();
                if data.first() != Some(&'"') || data.last() != Some(&'"') {
                    return Err(Error::new_spanned(
                        attrs,
                        format!("attribute `{}` must contain string", path),
                    ));
                }
            }
        }
    }

    Ok(())
}

fn validate_function(sig: &Signature) -> Result<()> {
    if sig.inputs.len() != 2 {
        return Err(Error::new_spanned(
            sig,
            "function must have only 2 arguments",
        ));
    }

    validate_externals(&sig.inputs[0])?;
    validate_args(&sig.inputs[1])?;

    if let syn::ReturnType::Type(_, ref ty) = sig.output {
        validate_type(ty.as_ref())?;
    }

    Ok(())
}

fn validate_externals(arg: &FnArg) -> Result<()> {
    match arg {
        FnArg::Receiver(_) => Err(Error::new_spanned(arg, "Receiver is not supported")),
        FnArg::Typed(pat) => validate_tuple(&*pat.ty, |_| Ok(())),
    }
}

fn validate_args(arg: &FnArg) -> Result<()> {
    match arg {
        FnArg::Receiver(_) => Err(Error::new_spanned(arg, "Receiver is not supported")),
        FnArg::Typed(pat) => validate_tuple(&*pat.ty, |ty| validate_type(ty)),
    }
}

fn validate_tuple<F>(ty: &Type, validate: F) -> Result<()>
where
    F: Fn(&Type) -> Result<()>,
{
    match ty {
        Type::Tuple(ty_tuple) => {
            for ty in ty_tuple.elems.iter() {
                validate(ty)?;
            }

            Ok(())
        }
        ty => validate(ty),
    }
}

fn validate_struct(ast: &DataStruct) -> Result<()> {
    match &ast.fields {
        syn::Fields::Named(fields) => {
            for field in fields.named.iter() {
                validate_field(field)?;
            }

            Ok(())
        }
        syn::Fields::Unnamed(fields) => {
            for field in fields.unnamed.iter() {
                validate_field(field)?;
            }

            Ok(())
        }
        syn::Fields::Unit => Err(Error::new_spanned(
            &ast.fields,
            "structure can have only named and unnamed fields",
        )),
    }
}

fn validate_field(field: &Field) -> Result<()> {
    // Если есть external и нет output, то проверяем тип
    let mut attrs = field.attrs.iter();
    if let Some(_) = attrs.find(|attr| attr.path().is_ident("external")) {
        if let None = attrs.find(|attr| attr.path().is_ident("output")) {
            return Ok(());
        }
    }

    validate_type(&field.ty)
}

fn validate_type(ty: &Type) -> Result<()> {
    match ty {
        Type::Path(path) => validate_type_path(&path),
        _ => Err(Error::new_spanned(
            ty,
            "type must contain only literals (T)",
        )),
    }
}

const VALIDATE_TYPE: [&str; 15] = [
    "i8", "i16", "i32", "i64", "u8", "u16", "u32", "u64", "f32", "f64", "bool", "char", "String",
    "Vec", "Variable",
];

fn validate_type_path(path: &TypePath) -> Result<()> {
    let segment = path.path.segments.last().unwrap();
    let ty = segment.ident.to_string();

    if VALIDATE_TYPE.contains(&ty.as_str()) {
        if ty == "Vec" {
            match &segment.arguments {
                PathArguments::AngleBracketed(args) => {
                    let arg = args.args.first().unwrap();
                    match arg {
                        GenericArgument::Type(ty) => return validate_type(ty),
                        _ => return Err(Error::new_spanned(arg, "Vec must contain only a type")),
                    }
                }
                _ => {}
            }
        }
    } else {
        return Err(Error::new_spanned(path, "type is not supported"));
    }

    Ok(())
}
