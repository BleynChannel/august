use crate::PluginLoader;

pub struct WrapperLoader {
    loader: *mut PluginLoader,
}

impl WrapperLoader {
    pub fn new(loader: &mut PluginLoader) -> Self {
        Self {
            loader: loader as *mut PluginLoader,
        }
    }

    pub fn unwrap(&self) -> &mut PluginLoader {
        unsafe { &mut *self.loader }
    }
}

impl PluginLoader {
    pub fn wrap(&mut self) -> WrapperLoader {
        WrapperLoader {
            loader: self as *mut PluginLoader,
        }
    }
}