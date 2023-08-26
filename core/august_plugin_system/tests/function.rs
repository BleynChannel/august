mod utils;

extern crate codegen;

#[cfg(test)]
mod tests {
    use august_plugin_system::{function::Request, variable::VariableType, Loader};
    use codegen::function;

    use crate::utils::{benchmark, get_plugin_path, LuaPluginManager, VoidPluginManager};

    #[function]
    fn add(_: (), a: &i32, b: &i32) -> i32 {
        a + b
    }

    #[function]
    fn sub(_: (), a: &i32, b: &i32) -> i32 {
        a - b
    }

    #[test]
    fn register_function() {
        let mut loader = Loader::new();
        loader.context(move |mut ctx| {
            ctx.register_function(add());
            ctx.register_manager(VoidPluginManager::new()).unwrap();
        });
    }

    #[test]
    fn register_functions() {
        let mut loader = Loader::new();
        loader.context(move |mut ctx| {
            ctx.register_function(add());
            ctx.register_function(sub());
            ctx.register_manager(VoidPluginManager::new()).unwrap();
        });
    }

    #[test]
    fn register_request() {
        let mut loader = Loader::new();
        loader.context(move |mut ctx| {
            ctx.register_request(Request::new(
                "mul".to_string(),
                vec![VariableType::I32, VariableType::I32],
                Some(VariableType::I32),
            ));
            ctx.register_manager(LuaPluginManager::new()).unwrap();
        });

        loader
            .load_plugin_now(
                get_plugin_path("function_plugin", "1.0.0", "fpl")
                    .to_str()
                    .unwrap(),
            )
            .unwrap();
    }

    #[test]
    fn call_request() {
        let mut loader = Loader::new();
        loader.context(move |mut ctx| {
            ctx.register_request(Request::new(
                "echo".to_string(),
                vec![VariableType::String],
                Some(VariableType::String),
            ));
            ctx.register_manager(LuaPluginManager::new()).unwrap();
        });

        let plugin = loader
            .load_plugin_now(
                get_plugin_path("function_plugin", "1.0.0", "fpl")
                    .to_str()
                    .unwrap(),
            )
            .map(|bundle| loader.get_plugin_by_bundle(&bundle).unwrap())
            .unwrap();

        match plugin
            .call_request("echo", &["Hello world".into()])
            .unwrap()
        {
            Err(e) => match e.downcast_ref::<rlua::Error>() {
                Some(e) => panic!("[LUA ERROR]: {e:?}"),
                None => panic!("{:?}: {}", e, e.to_string()),
            },
            Ok(Some(result)) => println!("{:?}", result),
            Ok(None) => panic!("Unexpected result"),
        };
    }

    #[test]
    fn common_call() {
        let mut loader = Loader::new();
        loader.context(move |mut ctx| {
            ctx.register_function(add());
            ctx.register_function(sub());
            ctx.register_request(Request::new("main".to_string(), vec![], None));
            ctx.register_manager(LuaPluginManager::new()).unwrap();
        });

        let plugin = loader
            .load_plugin_now(
                get_plugin_path("function_plugin", "1.0.0", "fpl")
                    .to_str()
                    .unwrap(),
            )
            .map(|bundle| loader.get_plugin_by_bundle(&bundle).unwrap())
            .unwrap();

        match plugin.call_request("main", &[]).unwrap() {
            Err(e) => match e.downcast_ref::<rlua::Error>() {
                Some(e) => panic!("[LUA ERROR]: {e:?}"),
                None => panic!("{:?}: {}", e, e.to_string()),
            },
            Ok(_) => (),
        };
    }

    #[test]
    fn loader_call_request() {
        let mut loader = Loader::new();
        loader.context(move |mut ctx| {
            ctx.register_request(Request::new(
                "echo".to_string(),
                vec![VariableType::String],
                Some(VariableType::String),
            ));
            ctx.register_manager(LuaPluginManager::new()).unwrap();
        });

        loader
            .load_plugin_now(
                get_plugin_path("function_plugin", "1.0.0", "fpl")
                    .to_str()
                    .unwrap(),
            )
            .unwrap();

        match loader
            .call_request("echo", &["Hello world".into()])
            .unwrap()
            .get(0)
            .unwrap()
        {
            Err(e) => match e.downcast_ref::<rlua::Error>() {
                Some(e) => panic!("[LUA ERROR]: {e:?}"),
                None => panic!("{:?}: {}", e, e.to_string()),
            },
            Ok(Some(result)) => println!("{:?}", result),
            Ok(None) => panic!("Unexpected result"),
        };
    }

    #[test]
    fn parallel_call_request() {
        let mut loader = Loader::new();
        loader.context(move |mut ctx| {
            ctx.register_request(Request::new(
                "main".to_string(),
                vec![VariableType::I32],
                None,
            ));
            ctx.register_manager(LuaPluginManager::new()).unwrap();
        });

        loader
            .load_plugins([
                get_plugin_path("parallel_plugins/one_plugin", "1.0.0", "fpl")
                    .to_str()
                    .unwrap(),
                get_plugin_path("parallel_plugins/two_plugin", "1.0.0", "fpl")
                    .to_str()
                    .unwrap(),
            ])
            .unwrap();

        let (duration, result) = benchmark(|| loader.call_request("main", &[10.into()]));
        println!("Single: {duration:?}");

        if let Err(e) = result.unwrap().get(0).unwrap() {
            match e.downcast_ref::<rlua::Error>() {
                Some(e) => panic!("[LUA ERROR]: {e:?}"),
                None => panic!("{:?}: {}", e, e.to_string()),
            }
        }

        let (duration, result) = benchmark(|| loader.par_call_request("main", &[10.into()]));
        println!("Parallel: {duration:?}");

        if let Err(e) = result.unwrap().get(0).unwrap() {
            match e.downcast_ref::<rlua::Error>() {
                Some(e) => panic!("[LUA ERROR]: {e:?}"),
                None => panic!("{:?}: {}", e, e.to_string()),
            }
        }
    }
}
