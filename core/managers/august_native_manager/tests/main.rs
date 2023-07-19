mod utils;

#[cfg(test)]
mod main {
    use august_native_manager::NativePluginManager;
    use august_plugin_system::LoaderBuilder;

    use crate::utils::{get_plugin_path, loader_init};

    #[test]
    fn load_manager() {
		let mut loader = match LoaderBuilder::new().register_manager(NativePluginManager::new()).build() {
			Ok(loader) => loader,
			Err(e) => {
				panic!("{:?}: {}", e, e.to_string())
			}
		};

        if let Err(e) = loader.stop() {
            panic!("{:?}: {}", e, e.to_string());
        }
    }

    #[test]
    fn load_plugin() {
        let mut loader = loader_init();

        loader
            .load_plugin_now(get_plugin_path("native_plugin").to_str().unwrap())
            .unwrap();

        if let Err(e) = loader.stop() {
            panic!("{:?}: {}", e, e.to_string());
        }
    }
}
