use crate::{
    function::Function,
    utils::{Ptr, RegisterRequestError},
    Plugin, Registry, Requests,
};

pub struct LoadPluginContext<'a, T: Send + Sync> {
    pub(crate) plugin: Ptr<'a, Plugin<'a, T>>,
    pub(crate) registry: Ptr<'a, Registry<T>>,
    pub(crate) requests: Ptr<'a, Requests>,
}

impl<'a, T: Send + Sync> LoadPluginContext<'a, T> {
    pub(crate) const fn new(
        plugin: Ptr<'a, Plugin<'a, T>>,
        registry: Ptr<'a, Registry<T>>,
        requests: Ptr<'a, Requests>,
    ) -> Self {
        Self {
            plugin,
            registry,
            requests,
        }
    }

    pub fn plugin(&self) -> &Plugin<'a, T> {
        self.plugin.as_ref()
    }

    pub fn registry(&self) -> &Registry<T> {
        self.registry.as_ref()
    }

    pub fn requests(&self) -> &Requests {
        self.requests.as_ref()
    }

    pub fn register_request<F>(&mut self, request: F) -> Result<(), RegisterRequestError>
    where
        F: Function<Output = T> + 'static,
    {
        {
            if let Some(ord) = self
                .requests
                .as_ref()
                .iter()
                .find(|ord| *ord.name == request.name())
            {
                for input in ord.inputs.iter() {
                    request
                        .inputs()
                        .iter()
                        .find(|arg| *input == arg.ty)
                        .ok_or(RegisterRequestError::ArgumentsIncorrectly)?;
                }

                if ord.output != request.output().map(|arg| arg.ty) {
                    return Err(RegisterRequestError::ArgumentsIncorrectly);
                }
            } else {
                return Err(RegisterRequestError::NotFound);
            }
        }

        self.plugin.as_mut().requests.push(Box::new(request));

        Ok(())
    }
}
