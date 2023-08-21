use std::path::PathBuf;

use august_native_manager::NativePluginManager;
use august_plugin_system::{function::DynamicFunction, Loader};

pub fn loader_init<'a>() -> Loader<'a, DynamicFunction> {
    let mut loader = Loader::new();
    if let Err(e) = loader.context(move |mut ctx| ctx.register_manager(NativePluginManager::new()))
    {
        panic!("{:?}: {}", e, e.to_string())
    };
    loader
}

pub fn get_plugin_path(name: &str) -> PathBuf {
    std::env::current_dir()
        .unwrap()
        .join(format!("../../../plugins/{name}/build/plugin.npl"))
}
