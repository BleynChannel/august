use crate::utils::ParseVariableError;

#[derive(PartialEq, PartialOrd, Clone, Debug)]
pub enum VariableData {
    Null,
    I8(i8),
    I16(i16),
    I32(i32),
    I64(i64),
    U8(u8),
    U16(u16),
    U32(u32),
    U64(u64),
    F32(f32),
    F64(f64),
    Bool(bool),
    Char(char),
    String(String),
    List(Vec<VariableData>),
}

pub trait FromVariable: Sized {
	fn from_var(var: &VariableData) -> Result<Self, ParseVariableError>;
}

#[test]
fn into() {
    let a = 10_i16;

    let b: VariableData = a.into();
    assert_eq!(b, VariableData::I16(10));
}

#[test]
fn parse() {
    let a: VariableData = 10_i16.into();

	let b = a.parse::<i16>();

	assert!(b.is_ok());
	assert_eq!(b.unwrap(), 10);
}
