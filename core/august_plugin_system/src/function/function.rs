use std::fmt::Write;

use crate::{
    utils::FunctionResult,
    variable::{VariableData, VariableType},
};

#[derive(Debug)]
pub struct Arg {
    name: &'static str,
    var_type: VariableType,
}

impl Arg {
    pub fn new(name: &'static str, var_type: VariableType) -> Self {
        Self { name, var_type }
    }

    pub fn get_name(&self) -> &'static str {
        self.name
    }

    pub fn get_type(&self) -> VariableType {
        self.var_type
    }
}

pub struct Function {
    name: &'static str,
    description: &'static str,
    inputs: Vec<Arg>,
    ouputs: Option<Arg>,
    data: Box<dyn Fn(&[VariableData]) -> FunctionResult<Option<VariableData>>>,
}

impl Function {
    pub fn new<F>(
        name: &'static str,
        description: &'static str,
        inputs: Vec<Arg>,
        output: Option<Arg>,
        data: F,
    ) -> Self
    where
        F: Fn(&[VariableData]) -> FunctionResult<Option<VariableData>> + 'static,
    {
        Self {
            name,
            description,
            inputs,
            ouputs: output,
            data: Box::new(data),
        }
    }

    pub fn get_name(&self) -> &'static str {
        self.name
    }

    pub fn get_description(&self) -> &'static str {
        self.description
    }

    pub fn get_inputs(&self) -> &[Arg] {
        self.inputs.as_slice()
    }

    pub fn get_output(&self) -> &Option<Arg> {
        &self.ouputs
    }

    pub fn run(&self, args: &[VariableData]) -> FunctionResult<Option<VariableData>> {
        (*self.data)(args)
    }
}

impl std::fmt::Debug for Function {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        // Комментарий в виде описания функции
        f.write_str("# ")?;
        f.write_str(self.description)?;
        f.write_char('\n')?;

        // Функция
        f.write_str(self.name)?;
        f.write_str(format!("({:?})", self.inputs).as_str())?;
        f.write_str(" -> ")?;

        if let Some(arg) = &self.ouputs {
            return f.write_str(format!("{:?}", arg).as_str());
        } else {
            return f.write_str("void");
        }
    }
}

#[test]
fn run_function() {
    // Создание функции
    let add = |args: &[VariableData]| -> FunctionResult<Option<VariableData>> {
		let a = args[0].parse::<i32>()?;
		let b = args[1].parse::<i32>()?;

        let c = a + b;

        println!("{} + {} = {}", a, b, c);

        Ok(Some(c.into()))
    };

    let func = Function::new(
        "add",
        "add two numbers",
        vec![
            Arg::new("a", VariableType::I32),
            Arg::new("b", VariableType::I32),
        ],
        Some(Arg::new("c", VariableType::I32)),
        add,
    );

    println!("{:?}", func);

    // Запуск функции
    let c = func.run(vec![1.into(), 2.into()].as_slice());

	assert!(c.is_ok());
	assert_eq!(c.unwrap(), Some(3.into()));
}
