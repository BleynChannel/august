mod utils;

#[cfg(test)]
mod dependency {
    use std::path::PathBuf;

    use crate::utils::{get_plugin_path, loader_init, VoidPluginManager};

    fn get_dependencys_path() -> Vec<PathBuf> {
        vec![
            get_plugin_path("dependency/dep_1"),
            get_plugin_path("dependency/dep_2"),
            get_plugin_path("dependency/dep_3"),
            get_plugin_path("dependency/dep_4"),
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

        let plugin = loader.get_plugin(2).unwrap();
        loader.load_plugin(&plugin).unwrap();

        loader.stop().unwrap();
    }

    #[test]
    fn load_plugins() {
        let mut loader = loader_init(VoidPluginManager::new());

        let plugins = match loader.load_plugins(
            get_dependencys_path()
                .iter()
                .map(|x| x.to_str().unwrap())
                .collect(),
        ) {
            Ok(plugins) => plugins,
            Err((Some(e), _)) => panic!("{:?}: {}", e, e.to_string()),
            Err((_, Some(e))) => panic!("{:?}: {}", e, e.to_string()),
            Err((_, _)) => panic!("Unexpected error"),
        };

        for plugin in plugins {
            println!(
                "Path = {:?}, ID = {}",
                plugin.borrow().get_path(),
                plugin.borrow().get_info().id
            );
        }

        loader.stop().unwrap();
    }
}
