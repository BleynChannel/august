mod utils;

#[cfg(test)]
mod dependency {
    use std::path::PathBuf;

    use crate::utils::{get_plugin_path, loader_init, VoidPluginManager};

    fn get_dependencys_path() -> Vec<PathBuf> {
        vec![
            get_plugin_path("dependency/dep_1", "vpl"),
            get_plugin_path("dependency/dep_2", "vpl"),
            get_plugin_path("dependency/dep_3", "vpl"),
            get_plugin_path("dependency/dep_4", "vpl"),
        ]
    }

    #[test]
    fn register_dependency_plugin() {
        let mut loader = loader_init(VoidPluginManager::new());

        for path in get_dependencys_path() {
            loader.register_plugin(path.to_str().unwrap()).unwrap();
        }

        loader.stop().unwrap();
    }

    #[test]
    fn load_dependency_plugin() {
        let mut loader = loader_init(VoidPluginManager::new());

        for path in get_dependencys_path() {
            loader.register_plugin(path.to_str().unwrap()).unwrap();
        }

        loader.load_plugin(&"dep_3".to_string()).unwrap();

        loader.stop().unwrap();
    }

    #[test]
    fn load_plugins() {
        let mut loader = loader_init(VoidPluginManager::new());

        let plugins =
            match loader.load_plugins(get_dependencys_path().iter().map(|x| x.to_str().unwrap())) {
                Ok(plugins) => plugins,
                Err((Some(e), _)) => panic!("{:?}: {}", e, e.to_string()),
                Err((_, Some(e))) => panic!("{:?}: {}", e, e.to_string()),
                Err((_, _)) => panic!("Unexpected error"),
            };

        for plugin_id in plugins {
            let plugin = loader.get_plugin(&plugin_id).unwrap();
            println!("Path = {:?}, ID = {}", plugin.path(), plugin.info().id);
        }

        loader.stop().unwrap();
    }
}
