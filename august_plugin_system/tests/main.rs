mod test_manager;

#[cfg(test)]
mod tests {
    use std::path::PathBuf;

    use august_plugin_system::{PluginLoader, PluginManager};

    use crate::test_manager::TestManagerPlugin;

    fn native_plugin_path() -> PathBuf {
        std::env::current_dir()
            .unwrap()
            .join("../plugins/native_plugin/target/debug/plugin.testpl")
    }

    #[test]
    fn get_plugin_manager() {
        let mut plugin_managers: Vec<Box<dyn PluginManager>> = Vec::new();
        plugin_managers.push(TestManagerPlugin::new());

        match PluginLoader::init(plugin_managers) {
            Ok(loader) => {
                assert!(loader.get_manager(0).is_some());
            }
            Err(e) => {
                println!("{:?}: {}", e, e.to_string());
                assert!(false);
            }
        }
    }

    #[test]
    fn register_plugin() {
        let mut plugin_managers: Vec<Box<dyn PluginManager>> = Vec::new();
        plugin_managers.push(TestManagerPlugin::new());

        match PluginLoader::init(plugin_managers) {
            Ok(mut loader) => {
                match loader.register_plugin(native_plugin_path().to_str().unwrap()) {
                    Ok(index) => {
                        let plugin = loader.get_plugin(index).unwrap();

                        println!(
                            "Path = {:?}, ID = {}",
                            plugin.get_path(),
                            plugin.get_info().id,
                        );

                        if let Err(e) = loader.unregister_plugin(index) {
                            println!("{:?}: {}", e, e.to_string());
                            assert!(false);
                        }
                    }
                    Err(e) => {
                        println!("{:?}: {}", e, e.to_string());
                        assert!(false);
                    }
                };
            }
            Err(e) => {
                println!("{:?}: {}", e, e.to_string());
                assert!(false);
            }
        }

        assert!(true);
    }

    // #[test]
    // fn load_plugin() {
    //     let mut plugin_managers: Vec<Box<dyn PluginManager>> = Vec::new();
    //     plugin_managers.push(TestManagerPlugin::new());

    //     match PluginLoader::init(plugin_managers) {
    //         Ok(mut loader) => {
    //             match loader.register_plugin(native_plugin_path().to_str().unwrap()) {
    //                 Ok(index) => {
    // 					if let Err(e) = loader.load_plugin(index) {
    //                         println!("{:?}: {}", e, e.to_string());
    //                         assert!(false);
    //                     }
    //                 }
    //                 Err(e) => {
    //                     println!("{:?}: {}", e, e.to_string());
    //                     assert!(false);
    //                 }
    //             };
    //         }
    //         Err(e) => {
    //             println!("{:?}: {}", e, e.to_string());
    //             assert!(false);
    //         }
    //     }
    // }
}
