mod managers;
mod native_config;
pub use managers::*;

pub use native_config::*;

use std::path::PathBuf;

use august_plugin_system::{function::StdFunction, Loader, Manager};

pub fn get_plugin_path(name: &str, format: &str) -> PathBuf {
    std::env::current_dir()
        .unwrap()
        .join(format!("C:/Projects/Project/AUGUST/august/plugins/{name}/plugin.{format}"))
}

#[allow(dead_code)]
pub fn loader_init<'a, M>(manager: M) -> Loader<'a, StdFunction>
where
    M: Manager<'a, StdFunction> + 'static,
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
