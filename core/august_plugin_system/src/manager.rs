use crate::{
    context::LoadPluginContext,
    utils::{ManagerResult, Ptr},
    Info, Loader, Plugin, RegisterPluginContext,
};

pub trait Manager<'a, T: Send + Sync>: Send + Sync {
    fn format(&self) -> &str;

    fn register_manager(&mut self, _loader: Ptr<'a, Loader<'a, T>>) -> ManagerResult<()> {
        Ok(())
    }

    fn unregister_manager(&mut self) -> ManagerResult<()> {
        Ok(())
    }

    fn register_plugin(&mut self, _context: RegisterPluginContext) -> ManagerResult<Info> {
        Ok(Info::new())
    }

    fn unregister_plugin(&mut self, _plugin: &Plugin<'a, T>) -> ManagerResult<()> {
        Ok(())
    }

    fn load_plugin(&mut self, _context: LoadPluginContext<'a, '_, T>) -> ManagerResult<()> {
        Ok(())
    }

    fn unload_plugin(&mut self, _plugin: &Plugin<'a, T>) -> ManagerResult<()> {
        Ok(())
    }
}

impl<'a, T: Send + Sync> PartialEq for dyn Manager<'a, T> {
    fn eq(&self, other: &Self) -> bool {
        self.format() == other.format()
    }
}

impl<'a, T, TT> PartialEq<Box<dyn Manager<'a, T>>> for dyn Manager<'a, TT>
where
    T: Send + Sync,
    TT: Send + Sync,
{
    fn eq(&self, other: &Box<dyn Manager<'a, T>>) -> bool {
        self.format() == other.format()
    }
}

impl<'a, T, TT> PartialEq<dyn Manager<'a, TT>> for Box<dyn Manager<'a, T>>
where
    T: Send + Sync,
    TT: Send + Sync,
{
    fn eq(&self, other: &dyn Manager<'a, TT>) -> bool {
        self.format() == other.format()
    }
}