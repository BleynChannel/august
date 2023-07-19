use crate::variable::VariableType;

#[derive(Clone, Debug)]
pub struct Arg {
    pub(crate) name: String,
    pub(crate) ty: VariableType,
}

impl Arg {
    pub fn new(name: String, ty: VariableType) -> Self {
        Self { name, ty }
    }

    pub fn name(&self) -> String {
        self.name.clone()
    }

    pub fn ty(&self) -> VariableType {
        self.ty
    }
}
