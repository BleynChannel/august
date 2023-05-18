use std::fmt::Display;

use crate::utils::ParseVariableError;

use super::{FromVariable, VariableData};

impl From<i8> for VariableData {
    fn from(x: i8) -> Self {
        Self::I8(x)
    }
}

impl From<i16> for VariableData {
    fn from(x: i16) -> Self {
        Self::I16(x)
    }
}

impl From<i32> for VariableData {
    fn from(x: i32) -> Self {
        Self::I32(x)
    }
}

impl From<i64> for VariableData {
    fn from(x: i64) -> Self {
        Self::I64(x)
    }
}

impl From<u8> for VariableData {
    fn from(x: u8) -> Self {
        Self::U8(x)
    }
}

impl From<u16> for VariableData {
    fn from(x: u16) -> Self {
        Self::U16(x)
    }
}

impl From<u32> for VariableData {
    fn from(x: u32) -> Self {
        Self::U32(x)
    }
}

impl From<u64> for VariableData {
    fn from(x: u64) -> Self {
        Self::U64(x)
    }
}

impl From<f32> for VariableData {
    fn from(x: f32) -> Self {
        Self::F32(x)
    }
}

impl From<f64> for VariableData {
    fn from(x: f64) -> Self {
        Self::F64(x)
    }
}

impl From<bool> for VariableData {
    fn from(x: bool) -> Self {
        Self::Bool(x)
    }
}

impl From<char> for VariableData {
    fn from(x: char) -> Self {
        Self::Char(x)
    }
}

impl From<&str> for VariableData {
    fn from(x: &str) -> Self {
        Self::String(x.to_string())
    }
}

impl From<String> for VariableData {
    fn from(x: String) -> Self {
        Self::String(x)
    }
}

impl<T> From<&[T]> for VariableData
where
    T: Into<VariableData> + Clone,
{
    fn from(x: &[T]) -> Self {
        Self::List(x.iter().cloned().map(|item| item.into()).collect())
    }
}

impl<T> From<Vec<T>> for VariableData
where
    T: Into<VariableData> + Clone,
{
    fn from(x: Vec<T>) -> Self {
        Self::List(x.iter().cloned().map(|item| item.into()).collect())
    }
}

impl Default for VariableData {
    fn default() -> Self {
        Self::Null
    }
}

impl Display for VariableData {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(
            f,
            "{}",
            match self {
                VariableData::Null => "Null".to_string(),
                VariableData::I8(x) => format!("I8({x})"),
                VariableData::I16(x) => format!("I16({x})"),
                VariableData::I32(x) => format!("I32({x})"),
                VariableData::I64(x) => format!("I64({x})"),
                VariableData::U8(x) => format!("U8({x})"),
                VariableData::U16(x) => format!("U16({x})"),
                VariableData::U32(x) => format!("U32({x})"),
                VariableData::U64(x) => format!("U64({x})"),
                VariableData::F32(x) => format!("F32({x})"),
                VariableData::F64(x) => format!("F64({x})"),
                VariableData::Bool(x) => format!("Bool({x})"),
                VariableData::Char(x) => format!("Char({x})"),
                VariableData::String(x) => format!("String({x})"),
                VariableData::List(x) => format!("List({x:?})"),
            }
        )
    }
}

impl VariableData {
    pub fn parse<F>(&self) -> Result<F, ParseVariableError>
    where
        F: FromVariable,
    {
        F::from_var(self)
    }
}

impl VariableData {
    pub fn is_null(&self) -> bool {
        match self {
            VariableData::Null => true,
            _ => false,
        }
    }
}

impl FromVariable for i8 {
    fn from_var(var: &VariableData) -> Result<Self, ParseVariableError> {
        match var {
            VariableData::I8(x) => Ok(x.clone()),
            _ => Err(ParseVariableError::new("I8".to_string())),
        }
    }
}

impl FromVariable for i16 {
    fn from_var(var: &VariableData) -> Result<Self, ParseVariableError> {
        match var {
            VariableData::I16(x) => Ok(x.clone()),
            _ => Err(ParseVariableError::new("I16".to_string())),
        }
    }
}

impl FromVariable for i32 {
    fn from_var(var: &VariableData) -> Result<Self, ParseVariableError> {
        match var {
            VariableData::I32(x) => Ok(x.clone()),
            _ => Err(ParseVariableError::new("I32".to_string())),
        }
    }
}

impl FromVariable for i64 {
    fn from_var(var: &VariableData) -> Result<Self, ParseVariableError> {
        match var {
            VariableData::I64(x) => Ok(x.clone()),
            _ => Err(ParseVariableError::new("I64".to_string())),
        }
    }
}

impl FromVariable for u8 {
    fn from_var(var: &VariableData) -> Result<Self, ParseVariableError> {
        match var {
            VariableData::U8(x) => Ok(x.clone()),
            _ => Err(ParseVariableError::new("U8".to_string())),
        }
    }
}

impl FromVariable for u16 {
    fn from_var(var: &VariableData) -> Result<Self, ParseVariableError> {
        match var {
            VariableData::U16(x) => Ok(x.clone()),
            _ => Err(ParseVariableError::new("U16".to_string())),
        }
    }
}

impl FromVariable for u32 {
    fn from_var(var: &VariableData) -> Result<Self, ParseVariableError> {
        match var {
            VariableData::U32(x) => Ok(x.clone()),
            _ => Err(ParseVariableError::new("U32".to_string())),
        }
    }
}

impl FromVariable for u64 {
    fn from_var(var: &VariableData) -> Result<Self, ParseVariableError> {
        match var {
            VariableData::U64(x) => Ok(x.clone()),
            _ => Err(ParseVariableError::new("U64".to_string())),
        }
    }
}

impl FromVariable for f32 {
    fn from_var(var: &VariableData) -> Result<Self, ParseVariableError> {
        match var {
            VariableData::F32(x) => Ok(x.clone()),
            _ => Err(ParseVariableError::new("F32".to_string())),
        }
    }
}

impl FromVariable for f64 {
    fn from_var(var: &VariableData) -> Result<Self, ParseVariableError> {
        match var {
            VariableData::F64(x) => Ok(x.clone()),
            _ => Err(ParseVariableError::new("F64".to_string())),
        }
    }
}

impl FromVariable for bool {
    fn from_var(var: &VariableData) -> Result<Self, ParseVariableError> {
        match var {
            VariableData::Bool(x) => Ok(x.clone()),
            _ => Err(ParseVariableError::new("Bool".to_string())),
        }
    }
}

impl FromVariable for char {
    fn from_var(var: &VariableData) -> Result<Self, ParseVariableError> {
        match var {
            VariableData::Char(x) => Ok(x.clone()),
            _ => Err(ParseVariableError::new("Char".to_string())),
        }
    }
}

impl FromVariable for String {
    fn from_var(var: &VariableData) -> Result<Self, ParseVariableError> {
        match var {
            VariableData::String(x) => Ok(x.clone()),
            _ => Err(ParseVariableError::new("String".to_string())),
        }
    }
}

impl FromVariable for Vec<VariableData> {
    fn from_var(var: &VariableData) -> Result<Self, ParseVariableError> {
        match var {
            VariableData::List(x) => Ok(x.clone()),
            _ => Err(ParseVariableError::new("Vec<VariableData>".to_string())),
        }
    }
}

impl<T> FromVariable for Vec<T>
where
    T: FromVariable,
{
    fn from_var(var: &VariableData) -> Result<Self, ParseVariableError> {
        match var {
            VariableData::List(x) => {
                let mut arr = Vec::new();
                for var in x.iter() {
                    arr.push(var.parse::<T>()?);
                }
                Ok(arr)
            }
            _ => Err(ParseVariableError::new("Vec<T>".to_string())),
        }
    }
}
