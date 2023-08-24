use august_plugin_system::{Info, utils::bundle::Bundle};
use libloading::Library;

use crate::NativeConfig;

pub struct Plugin {
	pub(crate) bundle: Bundle,
	#[allow(dead_code)]
    pub(crate) info: Info,
	#[allow(dead_code)]
    pub(crate) config: NativeConfig,
    pub(crate) library: Option<Library>,
}

impl Plugin {
    pub fn new(bundle: Bundle, info: Info, config: NativeConfig) -> Self {
        Self {
			bundle,
            info,
            config,
            library: None,
        }
    }
}
