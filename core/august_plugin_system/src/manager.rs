use std::path::PathBuf;

use crate::{
    context::LoadPluginContext,
    utils::{ManagerResult, Ptr},
    Loader, Plugin, PluginInfo,
};

pub trait Manager<'a, T: Send + Sync>: Send + Sync {
    fn format(&self) -> &str;

    fn register_manager(&mut self, _loader: Ptr<'a, Loader<'a, T>>) -> ManagerResult<()> {
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
    fn unregister_plugin(&mut self, _plugin: Ptr<'a, Plugin<'a, T>>) -> ManagerResult<()> {
        Ok(())
    }

    fn register_plugin_error(&mut self, _info: PluginInfo) {}

    fn load_plugin(&mut self, _context: LoadPluginContext<'a, T>) -> ManagerResult<()> {
        Ok(())
    }

    fn unload_plugin(&mut self, _plugin: Ptr<'a, Plugin<'a, T>>) -> ManagerResult<()> {
        Ok(())
    }
}

impl<'a, T: Send + Sync> PartialEq for dyn Manager<'a, T> {
    fn eq(&self, other: &Self) -> bool {
        self.format() == other.format()
    }

    fn ne(&self, other: &Self) -> bool {
        self.format() != other.format()
    }
}
