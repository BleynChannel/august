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

        let bundle = loader
            .register_plugin(
                get_plugin_path("void_plugin", "1.0.0", "vpl")
                    .to_str()
                    .unwrap(),
            )
            .unwrap();

        let plugin = loader.get_plugin_by_bundle(&bundle).unwrap();
        println!(
            "Path = {:?}, Bundle = {}",
            plugin.info().path,
            plugin.info().bundle
        );

        loader.unregister_plugin_by_bundle(&bundle).unwrap();
        loader.stop().unwrap();
    }

    #[test]
    fn load_plugin() {
        let mut loader = loader_init(VoidPluginManager::new());

        let bundle = loader
            .register_plugin(
                get_plugin_path("void_plugin", "1.0.0", "vpl")
                    .to_str()
                    .unwrap(),
            )
            .unwrap();

        loader.load_plugin_by_bundle(&bundle).unwrap();
        loader.unload_plugin_by_bundle(&bundle).unwrap();

        loader.stop().unwrap();
    }

    #[test]
    fn load_now_plugin() {
        let mut loader = loader_init(VoidPluginManager::new());

        let bundle = match loader.load_plugin_now(
            get_plugin_path("void_plugin", "1.0.0", "vpl")
                .to_str()
                .unwrap(),
        ) {
            Ok(plugin) => plugin,
            Err((Some(e), _)) => panic!("{:?}: {}", e, e.to_string()),
            Err((_, Some(e))) => panic!("{:?}: {}", e, e.to_string()),
            Err((_, _)) => panic!("Unexpected error"),
        };

        loader.unload_plugin_by_bundle(&bundle).unwrap();
        loader.stop().unwrap();
    }
}
