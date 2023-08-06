use std::any::Any;

use crate::variable::Variable;

use super::Arg;

pub trait Function: Send + Sync {
    type CallResult: Send + Sync;

    fn name(&self) -> &String;
    fn inputs(&self) -> &Vec<Arg>;
    fn output(&self) -> &Option<Arg>;
    fn call(&self, args: &[Variable]) -> Self::CallResult;
}

pub type StdFunctionResult = Result<Option<Variable>, Box<dyn std::error::Error + Send + Sync>>;

pub struct StdFunction {
    name: String,
    inputs: Vec<Arg>,
    output: Option<Arg>,
    externals: Vec<Box<dyn Any + Send + Sync>>,
    ptr: Box<dyn Fn(&[Box<dyn Any + Send + Sync>], &[Variable]) -> StdFunctionResult + Send + Sync>,
}

impl StdFunction {
    pub fn new<S, F>(
        name: S,
        inputs: Vec<Arg>,
        output: Option<Arg>,
        externals: Vec<Box<dyn Any + Send + Sync>>,
        ptr: F,
    ) -> Self
    where
        S: Into<String>,
        F: Fn(&[Box<dyn Any + Send + Sync>], &[Variable]) -> StdFunctionResult
            + Send
            + Sync
            + 'static,
    {
        Self {
            name: name.into(),
            inputs,
            output,
            externals,
            ptr: Box::new(ptr),
        }
    }

    pub const fn externals(&self) -> &Vec<Box<dyn Any + Send + Sync>> {
        &self.externals
    }
}

impl Function for StdFunction {
    type CallResult = StdFunctionResult;

    fn name(&self) -> &String {
        &self.name
    }

    fn inputs(&self) -> &Vec<Arg> {
        &self.inputs
    }

    fn output(&self) -> &Option<Arg> {
        &self.output
    }

    fn call(&self, args: &[Variable]) -> Self::CallResult {
        (*self.ptr)(self.externals.as_slice(), args)
    }
}

impl std::fmt::Debug for StdFunction {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        //TODO: Внедрить описание функций в August
        // // Комментарий в виде описания функции
        // f.write_str("# ")?;
        // f.write_str(self.description.as_str())?;
        // f.write_char('\n')?;

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
fn function_call() {
    use crate::variable::VariableType;

    // Создание функции
    let func = StdFunction::new(
        "add",
        vec![
            Arg::new("a", VariableType::I32),
            Arg::new("b", VariableType::I32),
        ],
        Some(Arg::new("c", VariableType::I32)),
        vec![],
        |_, args| -> StdFunctionResult {
            let a = args[0].parse_ref::<i32>();
            let b = args[1].parse_ref::<i32>();

            let c = a + b;

            println!("{} + {} = {}", a, b, c);

            Ok(Some(c.into()))
        },
    );

    // Запуск функции
    let c = func.call(&[1.into(), 2.into()]);

    assert!(c.is_ok());
    assert_eq!(c.unwrap(), Some(3.into()));
}

#[test]
fn parallel_call() {
    use crate::variable::VariableType;
    use std::{sync::Arc, thread, time::Duration};

    // Создание функции
    let func = StdFunction::new(
        "log",
        vec![Arg::new("n", VariableType::I32)],
        None,
        vec![],
        |_, args| -> StdFunctionResult {
            let n = args[0].parse_ref::<i32>();

            println!("Step {n}");

            Ok(None)
        },
    );

    // Вызов функции в нескольких потоках
    let func = Arc::new(func);
    let mut handles = vec![];
    for i in 0..10 {
        let func = func.clone();
        handles.push(thread::spawn(move || {
            thread::sleep(Duration::from_secs(1));

            let result = func.call(&[i.into()]);

            assert!(result.is_ok());
            assert_eq!(result.unwrap(), None);
        }));
    }

    for handle in handles {
        handle.join().unwrap();
    }
}
