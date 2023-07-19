use std::{fmt::Write, any::Any};

use crate::{utils::FunctionResult, variable::Variable};

use super::Arg;

pub struct Function {
    pub(crate) name: String,
    pub(crate) description: String,
    pub(crate) inputs: Vec<Arg>,
    pub(crate) output: Option<Arg>,
	ptr: Box<dyn Fn(&[Box<dyn Any>], &[Variable]) -> FunctionResult<Option<Variable>> + Send + Sync>,
}

impl Function {
    pub fn new<F>(
        name: String,
        description: String,
        inputs: Vec<Arg>,
        output: Option<Arg>,
        ptr: F,
    ) -> Self
    where
        F: Fn(&[Box<dyn Any>], &[Variable]) -> FunctionResult<Option<Variable>> + Send + Sync + 'static,
    {
        Self {
            name,
            description,
            inputs,
            output,
            ptr: Box::new(ptr),
        }
    }

    pub fn name(&self) -> String {
        self.name.clone()
    }

    pub fn description(&self) -> String {
        self.description.clone()
    }

    pub fn inputs(&self) -> &[Arg] {
        self.inputs.as_slice()
    }

    pub fn output(&self) -> &Option<Arg> {
        &self.output
    }

    pub fn call(&self, externals: &[Box<dyn Any>], args: &[Variable]) -> FunctionResult<Option<Variable>> {
        (*self.ptr)(externals, args)
    }
}

impl std::fmt::Debug for Function {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        // Комментарий в виде описания функции
        f.write_str("# ")?;
        f.write_str(self.description.as_str())?;
        f.write_char('\n')?;

        // Функция
        f.write_str(self.name.as_str())?;
        f.write_str(format!("({:?})", self.inputs).as_str())?;
        f.write_str(" -> ")?;

        if let Some(arg) = &self.output {
            return f.write_str(format!("{:?}", arg).as_str());
        } else {
            return f.write_str("void");
        }
    }
}

#[test]
fn run_function() {
    use crate::variable::VariableType;

    // Создание функции
    let func = Function::new(
        "add".to_string(),
        "add two numbers".to_string(),
        vec![
            Arg::new("a".to_string(), VariableType::I32),
            Arg::new("b".to_string(), VariableType::I32),
        ],
        Some(Arg::new("c".to_string(), VariableType::I32)),
        |_, args| -> FunctionResult<Option<Variable>> {
			let a = args[0].parse::<i32>()?;
			let b = args[1].parse::<i32>()?;
	
			let c = a + b;
	
			println!("{} + {} = {}", a, b, c);
	
			Ok(Some(c.into()))
		},
    );

    println!("{:?}", func);

    // Запуск функции
    let c = func.call(&[], &[1.into(), 2.into()]);

    assert!(c.is_ok());
    assert_eq!(c.unwrap(), Some(3.into()));
}
