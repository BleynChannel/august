mod utils;

#[cfg(test)]
mod tests {
    use crate::utils::{get_void_plugin_path, loader_init, managers::VoidPluginManager};

    #[test]
    fn get_plugin_manager() {
        let mut loader = loader_init(VoidPluginManager::new());

        let is_manager = loader.get_manager(0).is_some();

        if let Err(e) = loader.stop() {
            panic!("{:?}: {}", e, e.to_string());
        }

        assert!(is_manager);
    }

    #[test]
    fn register_plugin() {
        let mut loader = loader_init(VoidPluginManager::new());

        match loader.register_plugin(get_void_plugin_path("void_plugin").to_str().unwrap()) {
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
        let mut loader = loader_init(VoidPluginManager::new());

        match loader.register_plugin(get_void_plugin_path("void_plugin").to_str().unwrap()) {
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
        let mut loader = loader_init(VoidPluginManager::new());

        match loader.load_plugin_now(get_void_plugin_path("void_plugin").to_str().unwrap()) {
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
