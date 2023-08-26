use crate::{function::Function, utils::RegisterRequestError, Plugin, Registry, Requests};

pub struct LoadPluginContext<'a, 'b, T: Send + Sync> {
    pub(crate) plugin: &'b mut Plugin<'a, T>,
    pub(crate) requests: &'b Requests,
    pub(crate) registry: &'b Registry<T>,
}

impl<'a, 'b, T: Send + Sync> LoadPluginContext<'a, 'b, T> {
    pub(crate) fn new(
        plugin: &'b mut Plugin<'a, T>,
        requests: &'b Requests,
        registry: &'b Registry<T>,
    ) -> Self {
        Self {
            plugin,
            requests,
            registry,
        }
    }

    pub const fn plugin(&self) -> &Plugin<'a, T> {
        self.plugin
    }

    pub const fn requests(&self) -> &Requests {
        self.requests
    }

    pub const fn registry(&self) -> &Registry<T> {
        self.registry
    }

    pub fn register_request<F>(&mut self, request: F) -> Result<(), RegisterRequestError>
    where
        F: Function<Output = T> + 'static,
    {
        {
            if let Some(req) = self.requests.iter().find(|req| *req.name == request.name()) {
                for input in req.inputs.iter() {
                    request
                        .inputs()
                        .iter()
                        .find(|arg| *input == arg.ty)
                        .ok_or(RegisterRequestError::ArgumentsIncorrectly)?;
                }

                if req.output != request.output().map(|arg| arg.ty) {
                    return Err(RegisterRequestError::ArgumentsIncorrectly);
                }
            } else {
                return Err(RegisterRequestError::NotFound);
            }
        }

        self.plugin.requests.push(Box::new(request));

        Ok(())
    }
}
