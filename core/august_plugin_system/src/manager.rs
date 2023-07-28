use std::path::PathBuf;

use crate::{
    context::LoadPluginContext,
    utils::{FunctionResult, Ptr},
    Loader, Plugin, PluginInfo,
};

pub trait Manager<'a>: Send + Sync {
    fn format(&self) -> &str;

    fn register_manager(&mut self, _loader: Ptr<'a, Loader<'a>>) -> FunctionResult<()> {
        Ok(())
    }

    fn unregister_manager(&mut self) -> FunctionResult<()> {
        Ok(())
    }

    fn register_plugin(&mut self, path: &PathBuf) -> FunctionResult<PluginInfo> {
        Ok(PluginInfo::new(
            path.file_name().unwrap().to_str().unwrap().to_string(),
        ))
    }
    fn unregister_plugin(&mut self, _plugin: Ptr<'a, Plugin>) -> FunctionResult<()> {
        Ok(())
    }

    fn register_plugin_error(&mut self, _info: PluginInfo) {}

    fn load_plugin(&mut self, _context: LoadPluginContext) -> FunctionResult<()> {
        Ok(())
    }

    fn unload_plugin(&mut self, _plugin: Ptr<'a, Plugin>) -> FunctionResult<()> {
        Ok(())
    }
}

impl<'a> PartialEq for dyn Manager<'a> {
    fn eq(&self, other: &Self) -> bool {
        self.format() == other.format()
    }

    fn ne(&self, other: &Self) -> bool {
        self.format() != other.format()
    }
}
