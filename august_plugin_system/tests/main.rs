// #[cfg(test)]
mod tests {
    use std::collections::HashMap;

    use august_plugin_system::{Plugin, PluginLoader, PluginManager};
    use thiserror::__private::AsDynError;

    struct TestManagerPlugin;

    impl PluginManager for TestManagerPlugin {
        fn register_manager(&mut self) {
            println!("TestManagerPlugin::register_manager");
        }

        fn unregister_manager(&mut self) {
            println!("TestManagerPlugin::unregister_manager");
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
        let mut plugin_managers: HashMap<String, Box<dyn PluginManager>> = HashMap::new();
        plugin_managers.insert("testpl".to_string(), TestManagerPlugin::new());

        let loader = PluginLoader::init(plugin_managers);

        assert!(loader.get_plugin_manager("testpl".to_string()).is_some());
    }

    #[test]
    fn register_plugin() {
        let mut plugin_managers: HashMap<String, Box<dyn PluginManager>> = HashMap::new();
        plugin_managers.insert("testpl".to_string(), TestManagerPlugin::new());

        let mut loader = PluginLoader::init(plugin_managers);

        let plugins_path = std::env::current_dir()
            .unwrap()
            .join("../plugins/native_plugin/target/debug/plugin.testpl");

        match loader.register_plugin(plugins_path.to_str().unwrap()) {
            Ok(plugin) => {
                println!(
                    "Path = {:?}, Name = {}, Author = {}",
                    plugin.get_path(),
                    plugin.get_config().plugin.name,
                    plugin.get_config().plugin.author
                );
            }
            Err(e) => {
                println!("{:?}: {}", e, e.to_string());
                assert!(false);
            }
        }
    }
}
