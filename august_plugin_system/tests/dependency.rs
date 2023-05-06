mod utils;

#[cfg(test)]
mod dependency {
    use std::path::PathBuf;

    use august_plugin_system::{PluginLoader, PluginManager};

    use crate::utils::test_manager::TestManagerPlugin;

    fn get_plugin_path(name: &str, format: &str) -> PathBuf {
        std::env::current_dir().unwrap().join(format!(
            "../plugins/dependency/{name}/target/debug/plugin.{format}"
        ))
    }

    fn loader_init() -> PluginLoader {
        let mut plugin_managers: Vec<Box<dyn PluginManager>> = Vec::new();
        plugin_managers.push(TestManagerPlugin::new());

        match PluginLoader::init(plugin_managers) {
            Ok(loader) => loader,
            Err(e) => {
                panic!("{:?}: {}", e, e.to_string())
            }
        }
    }

    fn get_dependencys_path() -> Vec<PathBuf> {
        vec![
            get_plugin_path("dep_1", "testpl"),
            get_plugin_path("dep_2", "testpl"),
            get_plugin_path("dep_3", "testpl"),
            get_plugin_path("dep_4", "testpl"),
        ]
    }

    #[test]
    fn register_dependency_plugin() {
        let mut loader = loader_init();

        for path in get_dependencys_path() {
            match loader.register_plugin(path.to_str().unwrap()) {
                Ok(_) => {}
                Err(e) => {
                    panic!("{:?}: {}", e, e.to_string());
                }
            }
        }

        if let Err(e) = loader.stop() {
            panic!("{:?}: {}", e, e.to_string());
        }
    }

    #[test]
    fn load_dependency_plugin() {
        let mut loader = loader_init();

        for path in get_dependencys_path() {
            match loader.register_plugin(path.to_str().unwrap()) {
                Ok(_) => {}
                Err(e) => {
                    panic!("{:?}: {}", e, e.to_string());
                }
            }
        }

        let plugin = loader.get_plugin(2).unwrap();
        if let Err(e) = loader.load_plugin(&plugin) {
            panic!("{:?}: {}", e, e.to_string());
        }

        if let Err(e) = loader.stop() {
            panic!("{:?}: {}", e, e.to_string());
        }
    }
}
