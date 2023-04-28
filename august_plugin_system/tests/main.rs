// #[cfg(test)]
mod tests {
    use august_plugin_system::{Plugin, PluginLoader, PluginManager};

    struct TestManagerPlugin;

    impl PluginManager for TestManagerPlugin {
        fn format(&self) -> &str {
            "testpl"
        }

        fn register_manager(&mut self) -> anyhow::Result<()> {
            println!("TestManagerPlugin::register_manager");
            Ok(())
        }

        fn unregister_manager(&mut self) -> anyhow::Result<()> {
            println!("TestManagerPlugin::unregister_manager");
            Ok(())
        }

        fn register_plugin(&mut self, plugin: &Plugin) -> anyhow::Result<()> {
            println!(
                "TestManagerPlugin::register_plugin - {}",
                plugin.get_config().plugin.name
            );
            Ok(())
        }

        fn unregister_plugin(&mut self, plugin: &Plugin) -> anyhow::Result<()> {
            println!(
                "TestManagerPlugin::unregister_plugin - {:?}",
                plugin.get_path()
            );
            Ok(())
        }
    }

    impl TestManagerPlugin {
        fn new() -> Box<Self> {
            Box::new(Self)
        }
    }

    #[test]
    fn get_plugin_manager() {
        let mut plugin_managers: Vec<Box<dyn PluginManager>> = Vec::new();
        plugin_managers.push(TestManagerPlugin::new());

        match PluginLoader::init(plugin_managers) {
            Ok(mut loader) => {
                assert!(loader.get_plugin_manager("testpl".to_string()).is_some());
            }
            Err(e) => {
                println!("{:?}: {}", e, e.to_string());
                assert!(false);
            }
        }
    }

    #[test]
    fn register_plugin() {
        let mut plugin_managers: Vec<Box<dyn PluginManager>> = Vec::new();
        plugin_managers.push(TestManagerPlugin::new());

        match PluginLoader::init(plugin_managers) {
            Ok(mut loader) => {
                let plugins_path = std::env::current_dir()
                    .unwrap()
                    .join("../plugins/native_plugin/target/debug/plugin.testpl");

                match loader.register_plugin(plugins_path.to_str().unwrap()) {
                    Ok(plugin_name) => {
						let plugin_name_str = plugin_name.as_str();
                        let plugin = loader.get_plugin(plugin_name_str).unwrap();

                        println!(
                            "Path = {:?}, Name = {}, Author = {}",
                            plugin.get_path(),
                            plugin.get_config().plugin.name,
                            plugin.get_config().plugin.author
                        );

                        if let Err(e) = loader.unregister_plugin(plugin_name_str) {
							println!("{:?}: {}", e, e.to_string());
							assert!(false);
						}
                    }
                    Err(e) => {
                        println!("{:?}: {}", e, e.to_string());
                        assert!(false);
                    }
                };
            }
            Err(e) => {
                println!("{:?}: {}", e, e.to_string());
                assert!(false);
            }
        }

        assert!(true);
    }
}
