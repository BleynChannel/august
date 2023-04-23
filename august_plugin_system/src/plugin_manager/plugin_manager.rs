use crate::{plugin::Plugin};

pub trait PluginManager {
	fn register_manager(&mut self);
	fn unregister_manager(&mut self);
}