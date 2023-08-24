use std::sync::Arc;

use crate::{
    function::{Function, Request},
    utils::RegisterManagerError,
    Loader, Manager,
};

pub struct LoaderContext<'a, 'b, T: Send + Sync> {
    loader: &'b mut Loader<'a, T>,
}

impl<'a, 'b, T: Send + Sync> LoaderContext<'a, 'b, T> {
    pub(crate) fn new(loader: &'b mut Loader<'a, T>) -> Self {
        Self { loader }
    }

    pub fn register_manager<M>(&mut self, manager: M) -> Result<(), RegisterManagerError>
    where
        M: Manager<'a, T> + 'static,
    {
        self.loader.register_manager(manager)
    }

	//TODO: Добавить параллельную версию метода
    pub fn register_managers<M>(&mut self, managers: M) -> Result<(), RegisterManagerError>
    where
        M: IntoIterator<Item = Box<dyn Manager<'a, T>>>,
    {
        self.loader.register_managers(managers)
    }

    pub fn register_request(&mut self, request: Request) {
        self.loader.requests.push(request);
    }

	//TODO: Добавить параллельную версию метода
    pub fn register_requests<I>(&mut self, requests: I)
    where
        I: IntoIterator<Item = Request>,
    {
        self.loader.requests.extend(requests);
    }

    pub fn register_function<F>(&mut self, function: F)
    where
        F: Function<Output = T> + 'static,
    {
        self.loader.registry.push(Arc::new(function));
    }

	//TODO: Добавить параллельную версию метода
    pub fn register_functions<F, I>(&mut self, functions: I)
    where
        F: Function<Output = T> + 'static,
        I: IntoIterator<Item = F>,
    {
        self.loader.registry.extend(
            functions
                .into_iter()
                .map(|f| Arc::new(f) as Arc<dyn Function<Output = T>>),
        );
    }
}
