#[derive(Clone, Copy, Debug)]
pub enum VariableType {
	Let,
    Int(VariableIntType),
    Float(VariableFloatType),
    Bool,
    Char,
    String,
    List,
}

#[derive(Clone, Copy, Debug)]
pub enum VariableIntType {
    Signed(VariableSignedIntType),
    Unsigned(VariableUnsignedIntType),
}

#[derive(Clone, Copy, Debug)]
pub enum VariableSignedIntType {
    I8,
    I16,
    I32,
    I64,
}

#[derive(Clone, Copy, Debug)]
pub enum VariableUnsignedIntType {
    U8,
    U16,
    U32,
    U64,
}

#[derive(Clone, Copy, Debug)]
pub enum VariableFloatType {
    F32,
    F64,
}

impl VariableType {
    pub const I8: VariableType =
        VariableType::Int(VariableIntType::Signed(VariableSignedIntType::I8));
    pub const I16: VariableType =
        VariableType::Int(VariableIntType::Signed(VariableSignedIntType::I16));
    pub const I32: VariableType =
        VariableType::Int(VariableIntType::Signed(VariableSignedIntType::I32));
    pub const I64: VariableType =
        VariableType::Int(VariableIntType::Signed(VariableSignedIntType::I64));
    pub const U8: VariableType =
        VariableType::Int(VariableIntType::Unsigned(VariableUnsignedIntType::U8));
    pub const U16: VariableType =
        VariableType::Int(VariableIntType::Unsigned(VariableUnsignedIntType::U16));
    pub const U32: VariableType =
        VariableType::Int(VariableIntType::Unsigned(VariableUnsignedIntType::U32));
    pub const U64: VariableType =
        VariableType::Int(VariableIntType::Unsigned(VariableUnsignedIntType::U16));
    pub const F32: VariableType = VariableType::Float(VariableFloatType::F32);
    pub const F64: VariableType = VariableType::Float(VariableFloatType::F64);
}
