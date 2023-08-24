mod config;
mod managers;
pub use managers::*;

pub use config::*;

use std::path::PathBuf;

use august_plugin_system::{function::DynamicFunction, Loader, Manager};

pub fn get_plugin_path(id: &str, version: &str, format: &str) -> PathBuf {
    std::env::current_dir()
        .unwrap()
        .join(format!("../../plugins/{id}-v{version}.{format}"))
}

#[allow(dead_code)]
pub fn loader_init<'a, M>(manager: M) -> Loader<'a, DynamicFunction>
where
    M: Manager<'a, DynamicFunction> + 'static,
{
    let mut loader = Loader::new();
    if let Err(e) = loader.context(move |mut ctx| ctx.register_manager(manager)) {
        panic!("{:?}: {}", e, e.to_string());
    }

    loader
}

#[allow(dead_code)]
pub fn benchmark<F, R>(f: F) -> (std::time::Duration, R)
where
    F: FnOnce() -> R,
{
    let timer = std::time::Instant::now();
    let data = f();
    (timer.elapsed(), data)
}
