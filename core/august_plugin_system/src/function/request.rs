use crate::variable::VariableType;

pub struct Request {
    name: String,
    inputs: Vec<VariableType>,
    output: Option<VariableType>,
}

impl Request {
    pub fn new<S: Into<String>>(
        name: S,
        inputs: Vec<VariableType>,
        output: Option<VariableType>,
    ) -> Self {
        Self {
            name: name.into(),
            inputs,
            output,
        }
    }

    pub const fn name(&self) -> &String {
        &self.name
    }

    pub const fn inputs(&self) -> &Vec<VariableType> {
        &self.inputs
    }

    pub const fn output(&self) -> &Option<VariableType> {
        &self.output
    }
}
