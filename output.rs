impl Add {
    fn as_function() -> august_plugin_system::function::Function {
        august_plugin_system::function::Function::new(
            "add",
            "It's plugin",
            vec![
                august_plugin_system::function::Arg::new(
                    "a",
                    august_plugin_system::variable::VariableType::List,
                ),
                august_plugin_system::function::Arg::new(
                    "b",
                    august_plugin_system::variable::VariableType::String,
                ),
            ],
            Some(august_plugin_system::function::Arg::new(
                "c",
                august_plugin_system::variable::VariableType::List,
            )),
            move |args: &[august_plugin_system::variable::VariableData]| -> august_plugin_system::utils::FunctionResult<
                Option<august_plugin_system::variable::VariableData>,
            > {
                let tmp_0: Vec<i32> = args[0usize].parse()?;
                let tmp_1: String = args[1usize].parse()?;
                let result: Vec<i32> = Self::run(tmp_0, tmp_1);
                Ok(Some(august_plugin_system::variable::VariableData::List(
                    result.iter().map(|item| (*item).into()).collect(),
                )))
            },
        )
    }
}
