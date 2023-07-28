use std::any::Any;

use crate::{
    function::Function,
    utils::{Ptr, RegisterRequestError},
    Plugin, Registry, Requests,
};

pub struct LoadPluginContext<'a> {
    pub(crate) plugin: Ptr<'a, Plugin<'a>>,
    pub(crate) registry: Ptr<'a, Registry>,
    pub(crate) requests: Ptr<'a, Requests>,
}

impl<'a> LoadPluginContext<'a> {
    pub(crate) const fn new(
        plugin: Ptr<'a, Plugin<'a>>,
        registry: Ptr<'a, Registry>,
        requests: Ptr<'a, Requests>,
    ) -> Self {
        Self {
            plugin,
            registry,
            requests,
        }
    }

    pub fn plugin(&self) -> &Plugin<'a> {
        self.plugin.as_ref()
    }

    pub fn registry(&self) -> &Registry {
        self.registry.as_ref()
    }

    pub fn requests(&self) -> &Requests {
        self.requests.as_ref()
    }

    pub fn register_request(
        &mut self,
        request: Function,
        externals: Vec<Box<dyn Any>>,
    ) -> Result<(), RegisterRequestError> {
        {
            if let Some(ord) = self
                .requests
                .as_ref()
                .iter()
                .find(|ord| ord.name == request.name)
            {
                for input in ord.inputs.iter() {
                    request
                        .inputs
                        .iter()
                        .find(|arg| *input == arg.ty)
                        .ok_or(RegisterRequestError::ArgumentsIncorrectly)?;
                }

                if ord.output != request.output.as_ref().map(|arg| arg.ty) {
                    return Err(RegisterRequestError::ArgumentsIncorrectly);
                }
            } else {
                return Err(RegisterRequestError::NotFound);
            }
        }

        self.plugin.as_mut().requests.push((externals, request));

        Ok(())
    }
}
