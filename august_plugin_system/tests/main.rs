mod utils;

#[cfg(test)]
mod tests {
    use std::path::PathBuf;

    use august_plugin_system::{PluginLoader, PluginManager};

    use crate::utils::test_manager::TestManagerPlugin;

    fn get_plugin_path(name: &str, format: &str) -> PathBuf {
        std::env::current_dir()
            .unwrap()
            .join(format!("../plugins/{name}/target/debug/plugin.{format}"))
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

    #[test]
    fn get_plugin_manager() {
        let mut loader = loader_init();

        let is_manager = loader.get_manager(0).is_some();

        if let Err(e) = loader.stop() {
            panic!("{:?}: {}", e, e.to_string());
        }

        assert!(is_manager);
    }

    #[test]
    fn register_plugin() {
        let mut loader = loader_init();

        match loader.register_plugin(get_plugin_path("native_plugin", "testpl").to_str().unwrap()) {
            Ok(plugin) => {
                {
                    let pl = plugin.borrow();

                    println!("Path = {:?}, ID = {}", pl.get_path(), pl.get_info().id);
                }

                if let Err(e) = loader.unregister_plugin(&plugin) {
                    panic!("{:?}: {}", e, e.to_string());
                }
            }
            Err(e) => {
                panic!("{:?}: {}", e, e.to_string());
            }
        };

        if let Err(e) = loader.stop() {
            panic!("{:?}: {}", e, e.to_string());
        }
    }

    #[test]
    fn load_plugin() {
        let mut loader = loader_init();

        match loader.register_plugin(get_plugin_path("native_plugin", "testpl").to_str().unwrap()) {
            Ok(plugin) => {
                if let Err(e) = loader.load_plugin(&plugin) {
                    panic!("{:?}: {}", e, e.to_string());
                }

                if let Err(e) = loader.unload_plugin(&plugin) {
                    panic!("{:?}: {}", e, e.to_string());
                }
            }
            Err(e) => {
                panic!("{:?}: {}", e, e.to_string());
            }
        };

        if let Err(e) = loader.stop() {
            panic!("{:?}: {}", e, e.to_string());
        }
    }

    #[test]
    fn load_now_plugin() {
        let mut loader = loader_init();

        match loader.load_plugin_now(get_plugin_path("native_plugin", "testpl").to_str().unwrap()) {
            Ok(plugin) => {
                if let Err(e) = loader.load_plugin(&plugin) {
                    panic!("{:?}: {}", e, e.to_string());
                }
            }
            Err((Some(e), _)) => {
                panic!("{:?}: {}", e, e.to_string());
            }
            Err((_, Some(e))) => {
                panic!("{:?}: {}", e, e.to_string());
            }
            Err((_, _)) => {}
        }

        if let Err(e) = loader.stop() {
            panic!("{:?}: {}", e, e.to_string());
        }
    }
}
