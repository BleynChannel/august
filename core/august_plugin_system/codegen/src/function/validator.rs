use syn::{
    Data, DataStruct, DeriveInput, Error, Field, GenericArgument, PathArguments, Result, Type,
    TypePath,
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
    validate_field_type(&field.ty)
}

fn validate_field_type(ty: &Type) -> Result<()> {
    match ty {
        Type::Path(path) => validate_field_path(path),
        _ => Err(Error::new_spanned(
            ty,
            "field must contain only literals (T)",
        )),
    }
}

const VALIDATE_TYPE: [&str; 15] = [
    "i8",
    "i16",
    "i32",
    "i64",
    "u8",
    "u16",
    "u32",
    "u64",
    "f32",
    "f64",
    "bool",
    "char",
    "String",
    "Vec",
    "VariableData",
];

fn validate_field_path(path: &TypePath) -> Result<()> {
    let segment = path.path.segments.last().unwrap();
    let ty = segment.ident.to_string();

    if VALIDATE_TYPE.contains(&ty.as_str()) {
        if ty == "Vec" {
            match &segment.arguments {
                PathArguments::AngleBracketed(args) => {
                    let arg = args.args.first().unwrap();
                    match arg {
                        GenericArgument::Type(ty) => return validate_field_type(ty),
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
