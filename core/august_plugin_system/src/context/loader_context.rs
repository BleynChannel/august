use std::sync::Arc;

use crate::{
    function::{Function, Request},
    utils::RegisterManagerError,
    Loader, Manager,
};

pub struct LoaderContext<'a, 'b, F: Function> {
    loader: &'b mut Loader<'a, F>,
}

impl<'a, 'b, F: Function> LoaderContext<'a, 'b, F> {
    pub(crate) fn new(loader: &'b mut Loader<'a, F>) -> Self {
        Self { loader }
    }

    pub fn register_manager<M>(&mut self, manager: M) -> Result<(), RegisterManagerError>
    where
        M: Manager<'a, F> + 'static,
    {
        self.loader.register_manager(manager)
    }

    pub fn register_managers<M>(&mut self, managers: M) -> Result<(), RegisterManagerError>
    where
        M: IntoIterator<Item = Box<dyn Manager<'a, F>>>,
    {
        self.loader.register_managers(managers)
    }

    pub fn register_request(&mut self, request: Request) {
        self.loader.requests.push(request);
    }

    pub fn register_requests<I>(&mut self, requests: I)
    where
        I: IntoIterator<Item = Request>,
    {
        self.loader.requests.extend(requests);
    }

    pub fn register_function(&mut self, function: F) {
        self.loader.registry.push(Arc::new(function));
    }

    pub fn register_functions<I>(&mut self, functions: I)
    where
        I: IntoIterator<Item = F>,
    {
        self.loader
            .registry
            .extend(functions.into_iter().map(|f| Arc::new(f)));
    }
}
