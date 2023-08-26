use crate::{
    context::LoadPluginContext,
    utils::{ManagerResult, Ptr},
    Info, Loader, Plugin, RegisterPluginContext,
};

pub trait Manager<'a, O: Send + Sync, I: Info>: Send + Sync {
    fn format(&self) -> &str;

    fn register_manager(&mut self, _loader: Ptr<'a, Loader<'a, O, I>>) -> ManagerResult<()> {
        Ok(())
    }

    fn unregister_manager(&mut self) -> ManagerResult<()> {
        Ok(())
    }

    fn register_plugin(&mut self, _context: RegisterPluginContext) -> ManagerResult<I>;

    fn unregister_plugin(&mut self, _plugin: &Plugin<'a, O, I>) -> ManagerResult<()> {
        Ok(())
    }

    fn load_plugin(&mut self, _context: LoadPluginContext<'a, '_, O, I>) -> ManagerResult<()> {
        Ok(())
    }

    fn unload_plugin(&mut self, _plugin: &Plugin<'a, O, I>) -> ManagerResult<()> {
        Ok(())
    }
}

impl<'a, O: Send + Sync, I: Info> PartialEq for dyn Manager<'a, O, I> {
    fn eq(&self, other: &Self) -> bool {
        self.format() == other.format()
    }
}

impl<'a, O, OO, I, II> PartialEq<Box<dyn Manager<'a, O, I>>> for dyn Manager<'a, OO, II>
where
    O: Send + Sync,
    OO: Send + Sync,
    I: Info,
    II: Info,
{
    fn eq(&self, other: &Box<dyn Manager<'a, O, I>>) -> bool {
        self.format() == other.format()
    }
}

impl<'a, O, OO, I, II> PartialEq<dyn Manager<'a, OO, II>> for Box<dyn Manager<'a, O, I>>
where
    O: Send + Sync,
    OO: Send + Sync,
    I: Info,
    II: Info,
{
    fn eq(&self, other: &dyn Manager<'a, OO, II>) -> bool {
        self.format() == other.format()
    }
}
