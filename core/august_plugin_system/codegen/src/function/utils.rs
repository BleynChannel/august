use syn::{punctuated::Punctuated, token::Comma, Error, Field, Fields, Result, Type, TypePath};

pub(crate) fn get_literal_type(field: &Field) -> &TypePath {
    match field.ty {
        Type::Path(ref ty) => ty,
        _ => panic!("Wrong type"),
    }
}

pub(crate) fn get_inputs_output_fields(
    fields: &Fields,
) -> Result<(Vec<(usize, &Field)>, Option<(usize, &Field)>)> {
    match fields {
        Fields::Named(fields) => Ok(get_inputs_output_fields_common(&fields.named)),
        Fields::Unnamed(fields) => Ok(get_inputs_output_fields_common(&fields.unnamed)),
        Fields::Unit => Err(Error::new_spanned(
            fields,
            "structure can have only named and unnamed fields",
        )),
    }
}

fn get_inputs_output_fields_common(
    fields: &Punctuated<Field, Comma>,
) -> (Vec<(usize, &Field)>, Option<(usize, &Field)>) {
    let mut inputs = Vec::new();
    let mut output = None;
    for (index, field) in fields.iter().enumerate() {
        if let Some(attr) = field.attrs.first() {
            if attr.path().get_ident().unwrap().to_string() == "output" {
                output = Some((index, field));
                continue;
            }
        }

        inputs.push((index, field));
    }
    (inputs, output)
}
