use std::sync::Arc;

use crate::{
    function::{Function, Request},
    utils::BuilderError,
    Requests, PluginLoader, PluginManager, Registry,
};

pub struct LoaderBuilder {
    managers: Vec<Box<dyn PluginManager>>,
    registry: Registry,
    requests: Requests,
}

impl LoaderBuilder {
    pub fn new() -> Self {
        Self {
            managers: Vec::new(),
            registry: Registry::new(),
            requests: Requests::new(),
        }
    }

    pub fn build(self) -> Result<PluginLoader, BuilderError> {
        Ok(PluginLoader::new(
            self.managers,
            self.registry,
            self.requests,
        )?)
    }

    pub fn register_manager(mut self, manager: Box<dyn PluginManager>) -> Self {
        self.managers.push(manager);
        self
    }

    pub fn register_managers(mut self, managers: Vec<Box<dyn PluginManager>>) -> Self {
        self.managers = managers;
        self
    }

    pub fn register_request(mut self, request: Request) -> Self {
        self.requests.push(request);
        self
    }

    pub fn register_requests(mut self, requests: Vec<Request>) -> Self {
        self.requests = requests;
        self
    }

    pub fn register_function(mut self, function: Function) -> Self {
        self.registry.push(Arc::new(function));
        self
    }

    pub fn register_functions(mut self, functions: Vec<Function>) -> Self {
        self.registry
            .extend(functions.into_iter().map(|f| Arc::new(f)));
        self
    }
}
