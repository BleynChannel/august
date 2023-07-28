mod utils;

#[cfg(test)]
mod tests {
    use crate::utils::{get_plugin_path, loader_init, VoidPluginManager};

    #[test]
    fn get_plugin_manager() {
        let mut loader = loader_init(VoidPluginManager::new());

        let is_manager = loader.get_manager_ref("vpl").is_some();

        loader.stop().unwrap();

        assert!(is_manager);
    }

    #[test]
    fn register_plugin() {
        let mut loader = loader_init(VoidPluginManager::new());

        let plugin_id = loader
            .register_plugin(get_plugin_path("void_plugin", "vpl").to_str().unwrap())
            .unwrap();

        let plugin = loader.get_plugin(&plugin_id).unwrap();
        println!("Path = {:?}, ID = {}", plugin.path(), plugin.info().id);

        loader.unregister_plugin(&plugin_id).unwrap();
        loader.stop().unwrap();
    }

    #[test]
    fn load_plugin() {
        let mut loader = loader_init(VoidPluginManager::new());

        let plugin_id = loader
            .register_plugin(get_plugin_path("void_plugin", "vpl").to_str().unwrap())
            .unwrap();

        loader.load_plugin(&plugin_id).unwrap();
        loader.unload_plugin(&plugin_id).unwrap();

        loader.stop().unwrap();
    }

    #[test]
    fn load_now_plugin() {
        let mut loader = loader_init(VoidPluginManager::new());

        let plugin_id =
            match loader.load_plugin_now(get_plugin_path("void_plugin", "vpl").to_str().unwrap()) {
                Ok(plugin) => plugin,
                Err((Some(e), _)) => panic!("{:?}: {}", e, e.to_string()),
                Err((_, Some(e))) => panic!("{:?}: {}", e, e.to_string()),
                Err((_, _)) => panic!("Unexpected error"),
            };

        loader.unload_plugin(&plugin_id).unwrap();
        loader.stop().unwrap();
    }
}
