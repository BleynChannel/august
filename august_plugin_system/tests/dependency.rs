mod utils;

#[cfg(test)]
mod dependency {
    use std::path::PathBuf;

    use crate::utils::{loader_init, managers::VoidPluginManager, get_void_plugin_path};

    fn get_dependencys_path() -> Vec<PathBuf> {
        vec![
            get_void_plugin_path("dependency/dep_1"),
            get_void_plugin_path("dependency/dep_2"),
            get_void_plugin_path("dependency/dep_3"),
            get_void_plugin_path("dependency/dep_4"),
        ]
    }

    #[test]
    fn register_dependency_plugin() {
        let mut loader = loader_init(VoidPluginManager::new());

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
        let mut loader = loader_init(VoidPluginManager::new());

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
