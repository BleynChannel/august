#[cfg(test)]
mod tests {
    use std::collections::HashMap;

    use august_plugin_system::{PluginLoader, PluginManager};

    struct TestManagerPlugin;

    impl PluginManager for TestManagerPlugin {
        fn register_manager(&mut self) {
			println!("TestManagerPlugin::register_manager");
		}

        fn unregister_manager(&mut self) {
            println!("TestManagerPlugin::unregister_manager");
        }
    }

    #[test]
    fn get_plugin_manager() {
        let mut plugin_managers: HashMap<String, Box<dyn PluginManager>> = HashMap::new();
        plugin_managers.insert("testpl".to_string(), Box::new(TestManagerPlugin));

        let loader = PluginLoader::init(plugin_managers);

        assert!(loader.get_plugin_manager("testpl".to_string()).is_some());
    }

	#[test]
	fn register_plugin() {
		let mut plugin_managers: HashMap<String, Box<dyn PluginManager>> = HashMap::new();
        plugin_managers.insert("testpl".to_string(), Box::new(TestManagerPlugin));

        let mut loader = PluginLoader::init(plugin_managers);

		let plugins_path = std::env::current_dir().unwrap().join("../plugins/native_plugin/target/debug/plugin");
		assert!(loader.register_plugin(plugins_path.to_str().unwrap()).is_ok());
	}
}
