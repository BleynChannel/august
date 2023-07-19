mod utils;

extern crate codegen;

#[cfg(test)]
mod tests {
    use august_plugin_system::{function::Request, variable::VariableType, LoaderBuilder};
    use codegen::Function;

    use crate::utils::{get_plugin_path, LuaPluginManager, VoidPluginManager};

    #[derive(Function)]
    struct Add(i32, i32, #[output] i32);

    impl Add {
        fn call(_: (), (a, b): (i32, i32)) -> i32 {
            a + b
        }
    }

    #[derive(Function)]
    struct Sub(i32, i32, #[output] i32);

    impl Sub {
        fn call(_: (), (a, b): (i32, i32)) -> i32 {
            a - b
        }
    }

    #[test]
    fn register_function() {
        let mut loader = match LoaderBuilder::new()
            .register_manager(VoidPluginManager::new())
            .register_function(Add::as_function())
            .build()
        {
            Ok(loader) => loader,
            Err(e) => panic!("{:?}: {}", e, e.to_string()),
        };

        if let Err(e) = loader.stop() {
            panic!("{:?}: {}", e, e.to_string());
        }
    }

    #[test]
    fn register_functions() {
        let mut loader = match LoaderBuilder::new()
            .register_manager(VoidPluginManager::new())
            .register_functions(vec![Add::as_function(), Sub::as_function()])
            .build()
        {
            Ok(loader) => loader,
            Err(e) => panic!("{:?}: {}", e, e.to_string()),
        };

        if let Err(e) = loader.stop() {
            panic!("{:?}: {}", e, e.to_string());
        }
    }

    #[test]
    fn register_request() {
        let mut loader = match LoaderBuilder::new()
            .register_manager(LuaPluginManager::new())
            .register_request(Request::new(
                "mul".to_string(),
                vec![VariableType::I32, VariableType::I32],
                Some(VariableType::I32),
            ))
            .build()
        {
            Ok(loader) => loader,
            Err(e) => panic!("{:?}: {}", e, e.to_string()),
        };

        match loader.load_plugin_now(get_plugin_path("function_plugin", "fpl").to_str().unwrap()) {
            Ok(plugin) => plugin,
            Err((Some(e), _)) => panic!("{:?}: {}", e, e.to_string()),
            Err((_, Some(e))) => panic!("{:?}: {}", e, e.to_string()),
            Err((_, _)) => panic!("Unexpected error"),
        };

        loader.stop().unwrap();
    }

    #[test]
    fn call_request() {
        let mut loader = match LoaderBuilder::new()
            .register_manager(LuaPluginManager::new())
            .register_request(Request::new(
                "echo".to_string(),
                vec![VariableType::String],
                Some(VariableType::String),
            ))
            .build()
        {
            Ok(loader) => loader,
            Err(e) => panic!("{:?}: {}", e, e.to_string()),
        };

        let plugin = match loader
            .load_plugin_now(get_plugin_path("function_plugin", "fpl").to_str().unwrap())
        {
            Ok(plugin) => plugin,
            Err((Some(e), _)) => panic!("{:?}: {}", e, e.to_string()),
            Err((_, Some(e))) => panic!("{:?}: {}", e, e.to_string()),
            Err((_, _)) => panic!("Unexpected error"),
        };

        match plugin.borrow().call_request("echo", &["Hello world".into()]) {
            Err(e) => match e.downcast_ref::<rlua::Error>() {
                Some(e) => panic!("[LUA ERROR]: {e:?}"),
                None => panic!("{:?}: {}", e, e.to_string()),
            },
            Ok(Some(result)) => println!("{:?}", result),
            Ok(None) => panic!("Unexpected result"),
        };

        loader.stop().unwrap();
    }

    #[test]
    fn common_call() {
        let mut loader = match LoaderBuilder::new()
            .register_manager(LuaPluginManager::new())
            .register_functions(vec![Add::as_function(), Sub::as_function()])
            .register_request(Request::new("main".to_string(), vec![], None))
            .build()
        {
            Ok(loader) => loader,
            Err(e) => panic!("{:?}: {}", e, e.to_string()),
        };

        let plugin = match loader
            .load_plugin_now(get_plugin_path("function_plugin", "fpl").to_str().unwrap())
        {
            Ok(plugin) => plugin,
            Err((Some(e), _)) => panic!("{:?}: {}", e, e.to_string()),
            Err((_, Some(e))) => panic!("{:?}: {}", e, e.to_string()),
            Err((_, _)) => panic!("Unexpected error"),
        };

        match plugin.borrow().call_request("main", &[]) {
            Err(e) => match e.downcast_ref::<rlua::Error>() {
                Some(e) => panic!("[LUA ERROR]: {e:?}"),
                None => panic!("{:?}: {}", e, e.to_string()),
            },
            Ok(_) => (),
        };
    }
}
