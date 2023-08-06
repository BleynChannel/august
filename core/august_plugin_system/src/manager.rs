use std::path::PathBuf;

use crate::{
    context::LoadPluginContext,
    function::Function,
    utils::{ManagerResult, Ptr},
    Loader, Plugin, PluginInfo,
};

pub trait Manager<'a, F: Function>: Send + Sync {
    fn format(&self) -> &str;

    fn register_manager(&mut self, _loader: Ptr<'a, Loader<'a, F>>) -> ManagerResult<()> {
        Ok(())
    }

    fn unregister_manager(&mut self) -> ManagerResult<()> {
        Ok(())
    }

    fn register_plugin(&mut self, path: &PathBuf) -> ManagerResult<PluginInfo> {
        Ok(PluginInfo::new(
            path.file_name().unwrap().to_str().unwrap().to_string(),
        ))
    }
    fn unregister_plugin(&mut self, _plugin: Ptr<'a, Plugin<'a, F>>) -> ManagerResult<()> {
        Ok(())
    }

    fn register_plugin_error(&mut self, _info: PluginInfo) {}

    fn load_plugin(&mut self, _context: LoadPluginContext<'a, F>) -> ManagerResult<()> {
        Ok(())
    }

    fn unload_plugin(&mut self, _plugin: Ptr<'a, Plugin<'a, F>>) -> ManagerResult<()> {
        Ok(())
    }
}

impl<'a, F: Function> PartialEq for dyn Manager<'a, F> {
    fn eq(&self, other: &Self) -> bool {
        self.format() == other.format()
    }

    fn ne(&self, other: &Self) -> bool {
        self.format() != other.format()
    }
}
