use august_plugin_system::PluginInfo;
use libloading::Library;

use crate::NativeConfig;

pub struct Plugin {
    pub(crate) info: PluginInfo,
    pub(crate) config: NativeConfig,
    pub(crate) library: Library,
}

impl Plugin {
    pub fn new(info: PluginInfo, config: NativeConfig, library: Library) -> Self {
        Self {
            info,
            config,
            library,
        }
    }
}
