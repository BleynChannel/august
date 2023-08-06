use crate::utils::ParseVariableError;

#[derive(PartialEq, PartialOrd, Clone, Debug)]
pub enum Variable {
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
    List(Vec<Variable>),
}

pub trait FromVariable {
    type Output;
    type RefOutput<'a>
    where
        Self: 'a;
    type MutOutput<'a>
    where
        Self: 'a;

    fn from_var(var: Variable) -> Result<Self::Output, ParseVariableError>;
    fn from_var_ref(var: &Variable) -> Result<Self::RefOutput<'_>, ParseVariableError>;
    fn from_var_mut(var: &mut Variable) -> Result<Self::MutOutput<'_>, ParseVariableError>;
}

#[test]
fn into() {
    let a = 10_i16;

    let b: Variable = a.into();
    assert_eq!(b, Variable::I16(10));
}

#[test]
fn parse() {
    let mut a: Variable = 10_i16.into();

    assert_eq!(a.clone().parse::<i16>(), 10);
    assert_eq!(a.parse_ref::<i16>(), &10);
    assert_eq!(a.parse_mut::<i16>(), &mut 10);

    match a.clone().try_parse::<i16>() {
        Ok(b) => assert_eq!(b, 10),
        Err(e) => panic!("{}", e),
    };

	match a.try_parse_ref::<i16>() {
        Ok(b) => assert_eq!(b, &10),
        Err(e) => panic!("{}", e),
    };

	match a.try_parse_mut::<i16>() {
        Ok(b) => assert_eq!(b, &mut 10),
        Err(e) => panic!("{}", e),
    };
}

#[test]
fn parse_vec() {
    let mut a: Variable = vec![10_i16].into();

    let b = a.parse_mut::<Vec<i16>>();

    assert_eq!(b, vec![&mut 10]);
}
