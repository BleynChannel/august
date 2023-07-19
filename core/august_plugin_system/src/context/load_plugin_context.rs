use std::{cell::Ref, any::Any};

use crate::{function::Function, utils::RegisterRequestError, Link, Requests, Plugin, Registry};

#[derive(Clone)]
pub struct LoadPluginContext {
    pub(crate) plugin: Link<Plugin>,
    pub(crate) registry: Link<Registry>,
    pub(crate) requests: Link<Requests>,
}

impl LoadPluginContext {
    pub(crate) fn new(
        plugin: Link<Plugin>,
        registry: Link<Registry>,
        requests: Link<Requests>,
    ) -> Self {
        Self {
            plugin,
            registry,
            requests,
        }
    }

    pub fn plugin(&self) -> Ref<'_, Plugin> {
        self.plugin.borrow()
    }

    pub fn registry(&self) -> Ref<'_, Registry> {
        self.registry.borrow()
    }

    pub fn requests(&self) -> Ref<'_, Requests> {
        self.requests.borrow()
    }

    pub fn register_request(&mut self, request: Function, externals: Vec<Box<dyn Any>>) -> Result<(), RegisterRequestError> {		
		{
            if let Some(ord) = self
                .requests
                .borrow()
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

        self.plugin.borrow_mut().requests.push((externals, request));

		Ok(())
    }
}
