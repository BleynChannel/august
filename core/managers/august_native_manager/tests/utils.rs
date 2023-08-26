use std::path::PathBuf;

use august_native_manager::NativePluginManager;
use august_plugin_system::{function::FunctionOutput, Loader, StdInfo};

pub fn loader_init<'a>() -> Loader<'a, FunctionOutput, StdInfo> {
    let mut loader = Loader::new();
    loader
        .context(move |mut ctx| ctx.register_manager(NativePluginManager::new()))
        .unwrap();
    loader
}

pub fn get_plugin_path(id: &str, version: &str) -> PathBuf {
    std::env::current_dir()
        .unwrap()
        .join(format!("../../../plugins/{id}/build/{id}-v{version}.npl"))
}
