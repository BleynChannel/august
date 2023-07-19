use std::fmt::Display;

use crate::utils::ParseVariableError;

use super::{FromVariable, Variable};

impl From<i8> for Variable {
    fn from(x: i8) -> Self {
        Self::I8(x)
    }
}

impl From<i16> for Variable {
    fn from(x: i16) -> Self {
        Self::I16(x)
    }
}

impl From<i32> for Variable {
    fn from(x: i32) -> Self {
        Self::I32(x)
    }
}

impl From<i64> for Variable {
    fn from(x: i64) -> Self {
        Self::I64(x)
    }
}

impl From<u8> for Variable {
    fn from(x: u8) -> Self {
        Self::U8(x)
    }
}

impl From<u16> for Variable {
    fn from(x: u16) -> Self {
        Self::U16(x)
    }
}

impl From<u32> for Variable {
    fn from(x: u32) -> Self {
        Self::U32(x)
    }
}

impl From<u64> for Variable {
    fn from(x: u64) -> Self {
        Self::U64(x)
    }
}

impl From<f32> for Variable {
    fn from(x: f32) -> Self {
        Self::F32(x)
    }
}

impl From<f64> for Variable {
    fn from(x: f64) -> Self {
        Self::F64(x)
    }
}

impl From<bool> for Variable {
    fn from(x: bool) -> Self {
        Self::Bool(x)
    }
}

impl From<char> for Variable {
    fn from(x: char) -> Self {
        Self::Char(x)
    }
}

impl From<&str> for Variable {
    fn from(x: &str) -> Self {
        Self::String(x.to_string())
    }
}

impl From<String> for Variable {
    fn from(x: String) -> Self {
        Self::String(x)
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
    T: Into<Variable> + Clone,
{
    fn from(x: Vec<T>) -> Self {
        Self::List(x.iter().cloned().map(|item| item.into()).collect())
    }
}

impl Default for Variable {
    fn default() -> Self {
        Self::Null
    }
}

impl Display for Variable {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(
            f,
            "{}",
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
        )
    }
}

impl Variable {
    pub fn parse<F>(&self) -> Result<F, ParseVariableError>
    where
        F: FromVariable,
    {
        F::from_var(self)
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

impl FromVariable for i8 {
    fn from_var(var: &Variable) -> Result<Self, ParseVariableError> {
        match var {
            Variable::I8(x) => Ok(x.clone()),
            _ => Err(ParseVariableError::new("I8")),
        }
    }
}

impl FromVariable for i16 {
    fn from_var(var: &Variable) -> Result<Self, ParseVariableError> {
        match var {
            Variable::I16(x) => Ok(x.clone()),
            _ => Err(ParseVariableError::new("I16")),
        }
    }
}

impl FromVariable for i32 {
    fn from_var(var: &Variable) -> Result<Self, ParseVariableError> {
        match var {
            Variable::I32(x) => Ok(x.clone()),
            _ => Err(ParseVariableError::new("I32")),
        }
    }
}

impl FromVariable for i64 {
    fn from_var(var: &Variable) -> Result<Self, ParseVariableError> {
        match var {
            Variable::I64(x) => Ok(x.clone()),
            _ => Err(ParseVariableError::new("I64")),
        }
    }
}

impl FromVariable for u8 {
    fn from_var(var: &Variable) -> Result<Self, ParseVariableError> {
        match var {
            Variable::U8(x) => Ok(x.clone()),
            _ => Err(ParseVariableError::new("U8")),
        }
    }
}

impl FromVariable for u16 {
    fn from_var(var: &Variable) -> Result<Self, ParseVariableError> {
        match var {
            Variable::U16(x) => Ok(x.clone()),
            _ => Err(ParseVariableError::new("U16")),
        }
    }
}

impl FromVariable for u32 {
    fn from_var(var: &Variable) -> Result<Self, ParseVariableError> {
        match var {
            Variable::U32(x) => Ok(x.clone()),
            _ => Err(ParseVariableError::new("U32")),
        }
    }
}

impl FromVariable for u64 {
    fn from_var(var: &Variable) -> Result<Self, ParseVariableError> {
        match var {
            Variable::U64(x) => Ok(x.clone()),
            _ => Err(ParseVariableError::new("U64")),
        }
    }
}

impl FromVariable for f32 {
    fn from_var(var: &Variable) -> Result<Self, ParseVariableError> {
        match var {
            Variable::F32(x) => Ok(x.clone()),
            _ => Err(ParseVariableError::new("F32")),
        }
    }
}

impl FromVariable for f64 {
    fn from_var(var: &Variable) -> Result<Self, ParseVariableError> {
        match var {
            Variable::F64(x) => Ok(x.clone()),
            _ => Err(ParseVariableError::new("F64")),
        }
    }
}

impl FromVariable for bool {
    fn from_var(var: &Variable) -> Result<Self, ParseVariableError> {
        match var {
            Variable::Bool(x) => Ok(x.clone()),
            _ => Err(ParseVariableError::new("Bool")),
        }
    }
}

impl FromVariable for char {
    fn from_var(var: &Variable) -> Result<Self, ParseVariableError> {
        match var {
            Variable::Char(x) => Ok(x.clone()),
            _ => Err(ParseVariableError::new("Char")),
        }
    }
}

impl FromVariable for String {
    fn from_var(var: &Variable) -> Result<Self, ParseVariableError> {
        match var {
            Variable::String(x) => Ok(x.clone()),
            _ => Err(ParseVariableError::new("String")),
        }
    }
}

impl FromVariable for Vec<Variable> {
    fn from_var(var: &Variable) -> Result<Self, ParseVariableError> {
        match var {
            Variable::List(x) => Ok(x.clone()),
            _ => Err(ParseVariableError::new("Vec<Variable>")),
        }
    }
}

impl<T> FromVariable for Vec<T>
where
    T: FromVariable,
{
    fn from_var(var: &Variable) -> Result<Self, ParseVariableError> {
        match var {
            Variable::List(x) => {
                let mut arr = Vec::new();
                for var in x.iter() {
                    arr.push(var.parse::<T>()?);
                }
                Ok(arr)
            }
            _ => Err(ParseVariableError::new("Vec<T>")),
        }
    }
}
