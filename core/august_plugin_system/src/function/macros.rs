#[allow(unused_macros)]
macro_rules! function_call {
	($function: ident, ($($exts:expr), +), ($($args:expr), +)) => {
		$function.call(&[&$($exts), +], &[$($args.into()), +])
	};
	($function: ident, ($($args:expr), +)) => {
		$function.call(&[], &[$($args.into()), +])
	};
	($function: ident) => {
		$function.call(&[], &[])
	};
}

#[test]
fn run() {
	use std::any::Any;
    use crate::{
        function::{Arg, Function},
        utils::FunctionResult,
        variable::{Variable, VariableType},
    };

    // Создание функции
    let add = |_: &[Box<dyn Any>], args: &[Variable]| -> FunctionResult<Option<Variable>> {
        let a = args[0].parse::<i32>()?;
        let b = args[1].parse::<i32>()?;

        let c = a + b;

        println!("{} + {} = {}", a, b, c);

        Ok(Some(c.into()))
    };

    let func = Function::new(
        "add".to_string(),
        "add two numbers".to_string(),
        vec![
            Arg::new("a".to_string(), VariableType::I32),
            Arg::new("b".to_string(), VariableType::I32),
        ],
        Some(Arg::new("c".to_string(), VariableType::I32)),
        add,
    );

    // Запуск функции
    let c = function_call!(func, (1, 2));

    assert!(c.is_ok());
    assert_eq!(c.unwrap(), Some(3.into()));
}
