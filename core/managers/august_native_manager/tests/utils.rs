use std::path::PathBuf;

use august_native_manager::NativePluginManager;
use august_plugin_system::{PluginLoader, LoaderBuilder};

pub fn loader_init() -> PluginLoader {
	match LoaderBuilder::new().register_manager(NativePluginManager::new()).build() {
        Ok(loader) => loader,
        Err(e) => {
            panic!("{:?}: {}", e, e.to_string())
        }
    }
}

pub fn get_plugin_path(name: &str) -> PathBuf {
	std::env::current_dir()
        .unwrap()
        .join(format!("../../../plugins/{name}/build/plugin.npl"))
}
