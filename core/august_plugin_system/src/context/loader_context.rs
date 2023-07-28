use std::sync::Arc;

use crate::{
    function::{Function, Request},
    utils::RegisterManagerError,
    Loader, Manager,
};

pub struct LoaderContext<'a, 'b> {
    loader: &'b mut Loader<'a>,
}

impl<'a, 'b> LoaderContext<'a, 'b> {
    pub(crate) fn new(loader: &'b mut Loader<'a>) -> Self {
        Self { loader }
    }

    pub fn register_manager<M>(&mut self, manager: M) -> Result<(), RegisterManagerError>
    where
        M: Manager<'a> + 'static,
    {
        self.loader.register_manager(manager)
    }

    pub fn register_managers<M>(&mut self, managers: M) -> Result<(), RegisterManagerError>
    where
        M: IntoIterator<Item = Box<dyn Manager<'a>>>,
    {
        self.loader.register_managers(managers)
    }

	//TODO: Добавить регистрацию плагинов

    pub fn register_request(&mut self, request: Request) {
        self.loader.requests.push(request);
    }

    pub fn register_requests<R>(&mut self, requests: R)
    where
        R: IntoIterator<Item = Request>,
    {
        self.loader.requests.extend(requests);
    }

    pub fn register_function(&mut self, function: Function) {
        self.loader.registry.push(Arc::new(function));
    }

    pub fn register_functions<F>(&mut self, functions: F)
    where
        F: IntoIterator<Item = Function>,
    {
        self.loader
            .registry
            .extend(functions.into_iter().map(|f| Arc::new(f)));
    }
}
