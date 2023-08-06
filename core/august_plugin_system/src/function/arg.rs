use crate::variable::VariableType;

#[derive(Debug, Clone)]
pub struct Arg {
    name: String,
    ty: VariableType,
}

impl Arg {
    pub fn new<S: Into<String>>(name: S, ty: VariableType) -> Self {
        Self {
            name: name.into(),
            ty,
        }
    }

    pub const fn name(&self) -> &String {
        &self.name
    }

    pub const fn ty(&self) -> VariableType {
        self.ty
    }
}
