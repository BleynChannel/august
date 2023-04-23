use crate::{config::Config, PluginManager};

pub struct Plugin {
    name: String,
    path: String,
    config: Config,
    manager: Box<dyn PluginManager>,
	isLoad: bool,
}

impl Plugin {}
