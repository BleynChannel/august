use syn::{punctuated::Punctuated, token::Comma, Error, Field, Fields, Result, Type, TypePath};

pub(crate) fn get_literal_type(field: &Field) -> &TypePath {
    match field.ty {
        Type::Path(ref ty) => ty,
        _ => panic!("Wrong type"),
    }
}

pub(crate) fn get_externals_inputs_output_fields(
    fields: &Fields,
) -> Result<(Vec<&Field>, Vec<&Field>, Option<&Field>)> {
    match fields {
        Fields::Named(fields) => Ok(get_externals_inputs_output_fields_common(&fields.named)),
        Fields::Unnamed(fields) => Ok(get_externals_inputs_output_fields_common(&fields.unnamed)),
        Fields::Unit => Err(Error::new_spanned(
            fields,
            "structure can have only named and unnamed fields",
        )),
    }
}

fn get_externals_inputs_output_fields_common(
    fields: &Punctuated<Field, Comma>,
) -> (Vec<&Field>, Vec<&Field>, Option<&Field>) {
    let mut externals = Vec::new();
    let mut inputs = Vec::new();
    let mut output = None;
    for field in fields.iter() {
        if let Some(attr) = field.attrs.first() {
            let attr = attr.path().get_ident().unwrap().to_string();
            if attr == "output" {
                output = Some(field);
                continue;
            } else if attr == "external" {
                externals.push(field);
                continue;
            }
        }

        inputs.push(field);
    }
    (externals, inputs, output)
}
