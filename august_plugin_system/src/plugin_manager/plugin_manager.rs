use crate::plugin::Plugin;

pub trait PluginManager {
    fn register_manager(&mut self);
    fn unregister_manager(&mut self);

    fn register_plugin(&mut self, plugin: &Plugin) -> anyhow::Result<()>;
    fn unregister_plugin(&mut self, plugin: &Plugin) -> anyhow::Result<()>;
}
