use std::path::PathBuf;

use crate::{Plugin, PluginInfo, error::FunctionResult};

pub trait PluginManager {
    fn format(&self) -> &str;

    fn register_manager(&mut self) -> FunctionResult<()> {
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
    fn unregister_plugin(&mut self, plugin: &Plugin) -> FunctionResult<()> {
        Ok(())
    }

    fn register_plugin_error(&mut self, info: &PluginInfo) {}

    fn load_plugin(&mut self, plugin: &Plugin) -> FunctionResult<()> {
        Ok(())
    }

    fn unload_plugin(&mut self, plugin: &Plugin) -> FunctionResult<()> {
        Ok(())
    }
}

impl PartialEq for dyn PluginManager {
    fn eq(&self, other: &Self) -> bool {
        self.format() == other.format()
    }
}
