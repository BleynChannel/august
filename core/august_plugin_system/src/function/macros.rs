macro_rules! function_run {
	($function: ident, $($args:expr), +) => {
		$function.run(vec![$($args.into()), +].as_slice())
	};
}

#[test]
fn run() {
	use crate::{
		function::{Arg, Function},
		utils::FunctionResult,
		variable::{VariableData, VariableType},
	};

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

	// Запуск функции
    let c = function_run!(func, 1, 2);

    assert!(c.is_ok());
	assert_eq!(c.unwrap(), Some(3.into()));
}
