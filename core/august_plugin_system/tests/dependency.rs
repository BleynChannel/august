mod utils;

#[cfg(test)]
mod dependency {
    use std::path::PathBuf;

    use semver::Version;

    use crate::utils::{get_plugin_path, loader_init, VoidPluginManager};

    fn get_dependencys_path() -> Vec<PathBuf> {
        vec![
            get_plugin_path("dependency/dep_1", "1.0.0", "vpl"),
            get_plugin_path("dependency/dep_2", "1.0.0", "vpl"),
            get_plugin_path("dependency/dep_3", "1.0.0", "vpl"),
            get_plugin_path("dependency/dep_4", "1.0.0", "vpl"),
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

        loader
            .load_plugin("dep_3", &Version::parse("1.0.0").unwrap())
            .unwrap();

        loader.stop().unwrap();
    }

    #[test]
    fn load_plugins() {
        let mut loader = loader_init(VoidPluginManager::new());

        let plugins =
            match loader.load_plugins(get_dependencys_path().iter().map(|x| x.to_str().unwrap())) {
                Ok(plugins) => plugins,
                Err((Some(e), _, _)) => panic!("{:?}: {}", e, e.to_string()),
                Err((_, Some(e), _)) => panic!("{:?}: {}", e, e.to_string()),
                Err((_, _, Some(e))) => panic!("{:?}: {}", e, e.to_string()),
                Err((_, _, _)) => panic!("Unexpected error"),
            };

        for bundle in plugins {
            let plugin = loader.get_plugin_by_bundle(&bundle).unwrap();
            println!(
                "Path = {:?}, Bundle = {}",
                plugin.info().path,
                plugin.info().bundle
            );
        }

        loader.stop().unwrap();
    }
}
