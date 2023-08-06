use std::fmt::Display;

use crate::utils::ParseVariableError;

use super::{FromVariable, Variable};

macro_rules! impl_from {
    ($ty:ty, $from:ident) => {
        impl From<$ty> for Variable {
            fn from(x: $ty) -> Self {
                Self::$from(x)
            }
        }
    };
}

macro_rules! impl_from_variable {
    ($ty:ty, $from:ident) => {
        impl FromVariable for $ty {
            type Output = Self;
            type RefOutput<'a> = &'a Self;
            type MutOutput<'a> = &'a mut Self;

            fn from_var(var: Variable) -> Result<Self::Output, ParseVariableError> {
                match var {
                    Variable::$from(x) => Ok(x),
                    _ => Err(ParseVariableError::new(stringify!($from))),
                }
            }

            fn from_var_ref(var: &Variable) -> Result<Self::RefOutput<'_>, ParseVariableError> {
                match var {
                    Variable::$from(x) => Ok(x),
                    _ => Err(ParseVariableError::new(stringify!($from))),
                }
            }

            fn from_var_mut(var: &mut Variable) -> Result<Self::MutOutput<'_>, ParseVariableError> {
                match var {
                    Variable::$from(x) => Ok(x),
                    _ => Err(ParseVariableError::new(stringify!($from))),
                }
            }
        }
    };
}

impl From<&str> for Variable {
    fn from(x: &str) -> Self {
        Self::String(x.to_string())
    }
}

impl<T> From<&[T]> for Variable
where
    T: Into<Variable> + Clone,
{
    fn from(x: &[T]) -> Self {
        Self::List(x.iter().cloned().map(|item| item.into()).collect())
    }
}

impl<T> From<Vec<T>> for Variable
where
    T: Into<Variable>,
{
    fn from(x: Vec<T>) -> Self {
        Self::List(x.into_iter().map(|item| item.into()).collect())
    }
}

impl_from!(i8, I8);
impl_from!(i16, I16);
impl_from!(i32, I32);
impl_from!(i64, I64);
impl_from!(u8, U8);
impl_from!(u16, U16);
impl_from!(u32, U32);
impl_from!(u64, U64);
impl_from!(f32, F32);
impl_from!(f64, F64);
impl_from!(bool, Bool);
impl_from!(char, Char);
impl_from!(String, String);

impl Default for Variable {
    fn default() -> Self {
        Self::Null
    }
}

impl Display for Variable {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        f.write_str(
            match self {
                Variable::Null => "Null".to_string(),
                Variable::I8(x) => format!("I8({x})"),
                Variable::I16(x) => format!("I16({x})"),
                Variable::I32(x) => format!("I32({x})"),
                Variable::I64(x) => format!("I64({x})"),
                Variable::U8(x) => format!("U8({x})"),
                Variable::U16(x) => format!("U16({x})"),
                Variable::U32(x) => format!("U32({x})"),
                Variable::U64(x) => format!("U64({x})"),
                Variable::F32(x) => format!("F32({x})"),
                Variable::F64(x) => format!("F64({x})"),
                Variable::Bool(x) => format!("Bool({x})"),
                Variable::Char(x) => format!("Char({x})"),
                Variable::String(x) => format!("String({x})"),
                Variable::List(x) => format!("List({x:?})"),
            }
            .as_str(),
        )
    }
}

impl Variable {
    pub fn is_null(&self) -> bool {
        match self {
            Variable::Null => true,
            _ => false,
        }
    }
}

impl Variable {
    pub fn parse<F>(self) -> F::Output
    where
        F: FromVariable,
    {
        F::from_var(self).unwrap()
    }

    pub fn parse_ref<F>(&self) -> F::RefOutput<'_>
    where
        F: FromVariable,
    {
        F::from_var_ref(self).unwrap()
    }

    pub fn parse_mut<F>(&mut self) -> F::MutOutput<'_>
    where
        F: FromVariable,
    {
        F::from_var_mut(self).unwrap()
    }

    pub fn try_parse<F>(self) -> Result<F::Output, ParseVariableError>
    where
        F: FromVariable,
    {
        F::from_var(self)
    }

    pub fn try_parse_ref<F>(&self) -> Result<F::RefOutput<'_>, ParseVariableError>
    where
        F: FromVariable,
    {
        F::from_var_ref(self)
    }

    pub fn try_parse_mut<F>(&mut self) -> Result<F::MutOutput<'_>, ParseVariableError>
    where
        F: FromVariable,
    {
        F::from_var_mut(self)
    }
}

impl FromVariable for Vec<Variable> {
    type Output = Self;
    type RefOutput<'a> = &'a Self;
    type MutOutput<'a> = &'a mut Self;

    fn from_var(var: Variable) -> Result<Self::Output, ParseVariableError> {
        match var {
            Variable::List(x) => Ok(x),
            _ => Err(ParseVariableError::new("Vec<Variable>")),
        }
    }

    fn from_var_ref(var: &Variable) -> Result<Self::RefOutput<'_>, ParseVariableError> {
        match var {
            Variable::List(x) => Ok(x),
            _ => Err(ParseVariableError::new("Vec<Variable>")),
        }
    }

    fn from_var_mut(var: &mut Variable) -> Result<Self::MutOutput<'_>, ParseVariableError> {
        match var {
            Variable::List(x) => Ok(x),
            _ => Err(ParseVariableError::new("Vec<Variable>")),
        }
    }
}

impl<T> FromVariable for Vec<T>
where
    T: FromVariable,
{
    type Output = Vec<T::Output>;
    type RefOutput<'a> = Vec<T::RefOutput<'a>> where T: 'a;
    type MutOutput<'a> = Vec<T::MutOutput<'a>> where T: 'a;

    fn from_var(var: Variable) -> Result<Self::Output, ParseVariableError> {
        match var {
            Variable::List(x) => {
                let mut arr = vec![];
                for var in x.into_iter() {
                    arr.push(var.try_parse::<T>()?);
                }
                Ok(arr)
            }
            _ => Err(ParseVariableError::new("Vec<T>")),
        }
    }

    fn from_var_ref(var: &Variable) -> Result<Self::RefOutput<'_>, ParseVariableError> {
        match var {
            Variable::List(x) => {
                let mut arr = vec![];
                for var in x.iter() {
                    arr.push(var.try_parse_ref::<T>()?);
                }
                Ok(arr)
            }
            _ => Err(ParseVariableError::new("Vec<T>")),
        }
    }

    fn from_var_mut(var: &mut Variable) -> Result<Self::MutOutput<'_>, ParseVariableError> {
        match var {
            Variable::List(x) => {
                let mut arr = vec![];
                for var in x.iter_mut() {
                    arr.push(var.try_parse_mut::<T>()?);
                }
                Ok(arr)
            }
            _ => Err(ParseVariableError::new("Vec<T>")),
        }
    }
}

impl_from_variable!(i8, I8);
impl_from_variable!(i16, I16);
impl_from_variable!(i32, I32);
impl_from_variable!(i64, I64);
impl_from_variable!(u8, U8);
impl_from_variable!(u16, U16);
impl_from_variable!(u32, U32);
impl_from_variable!(u64, U64);
impl_from_variable!(f32, F32);
impl_from_variable!(f64, F64);
impl_from_variable!(bool, Bool);
impl_from_variable!(char, Char);
impl_from_variable!(String, String);
