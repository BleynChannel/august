use std::env::consts::OS;

use crate::{config::NativeConfig, Plugin};
use august_plugin_system::{
    context::LoadPluginContext,
    function::Function,
    utils::{bundle::Bundle, ManagerResult, Ptr},
    Depend, Info, Loader, Manager, Plugin as StdPlugin, RegisterPluginContext,
};
use libloading::Library;

pub struct NativePluginManager {
    plugins: Vec<Plugin>,
}

impl NativePluginManager {
    pub fn new() -> Self {
        Self { plugins: vec![] }
    }

    fn remove_plugin(&mut self, bundle: &Bundle) {
        self.plugins.retain(|plugin| plugin.bundle == *bundle);
    }
}

impl<'a, F: Function> Manager<'a, F> for NativePluginManager {
    fn format(&self) -> &str {
        "npl"
    }

    fn register_manager(&mut self, _: Ptr<'a, Loader<'a, F>>) -> ManagerResult<()> {
        Ok(())
    }
    fn unregister_manager(&mut self) -> ManagerResult<()> {
        Ok(())
    }

    fn register_plugin(&mut self, context: RegisterPluginContext) -> ManagerResult<Info> {
        let config = NativeConfig::load(context.path)?;
        let info = Info {
            depends: config.depends.clone().map_or(vec![], |depends| {
                depends
                    .into_iter()
                    .map(|(id, version)| Depend::new(id, version))
                    .collect()
            }),
            optional_depends: config.optional_depends.clone().map_or(vec![], |depends| {
                depends
                    .into_iter()
                    .map(|(id, version)| Depend::new(id, version))
                    .collect()
            }),
        };

        self.plugins
            .push(Plugin::new(context.bundle.clone(), info.clone(), config));
        Ok(info)
    }
    fn unregister_plugin(&mut self, plugin: Ptr<'a, StdPlugin<'a, F>>) -> ManagerResult<()> {
        self.remove_plugin(&plugin.as_ref().info().bundle);
        Ok(())
    }

    fn load_plugin(&mut self, context: LoadPluginContext<'a, F>) -> ManagerResult<()> {
        let plugin = context.plugin();

        // Загрузка библиотеки
        #[cfg(target_os = "windows")]
        let script = "main.dll";
        #[cfg(target_os = "linux")]
        let script = "libmain.so";
        //TODO: Сделать для MacOS

        let library;
        unsafe {
            library = Library::new(
                plugin
                    .info()
                    .path
                    .join(OS.to_string() + "/" + script)
                    .as_os_str(),
            )?;
        }

        let bundle = &plugin.info().bundle;
        self.plugins
            .iter_mut()
            .find(|p| p.bundle == *bundle)
            .unwrap()
            .library = Some(library);

        Ok(())
    }

    fn unload_plugin(&mut self, plugin: Ptr<'a, StdPlugin<'a, F>>) -> ManagerResult<()> {
        let bundle = &plugin.as_ref().info().bundle;
        self.plugins
            .iter_mut()
            .find(|p| p.bundle == *bundle)
            .unwrap()
            .library
            .take();

        Ok(())
    }
}
