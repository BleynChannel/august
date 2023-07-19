use crate::variable::VariableType;

pub struct Request {
    pub(crate) name: String,
    pub(crate) inputs: Vec<VariableType>,
    pub(crate) output: Option<VariableType>,
}

impl Request {
    pub fn new(name: String, args: Vec<VariableType>, output: Option<VariableType>) -> Self {
        Self { name, inputs: args, output }
    }

	pub fn name(&self) -> String {
		self.name.clone()
	}

	pub fn inputs(&self) -> &Vec<VariableType> {
		&self.inputs
	}

	pub fn output(&self) -> &Option<VariableType> {
		&self.output
	}
}
