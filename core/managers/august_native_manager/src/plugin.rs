use august_plugin_system::PluginInfo;
use libloading::Library;

use crate::NativeConfig;

pub struct Plugin {
    pub(crate) info: PluginInfo,
	#[allow(dead_code)]
    pub(crate) config: NativeConfig,
    pub(crate) library: Option<Library>,
}

impl Plugin {
    pub fn new(info: PluginInfo, config: NativeConfig) -> Self {
        Self {
            info,
            config,
            library: None,
        }
    }
}
