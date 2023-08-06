use crate::{
    function::Function,
    utils::{Ptr, RegisterRequestError},
    Plugin, Registry, Requests,
};

pub struct LoadPluginContext<'a, F: Function> {
    pub(crate) plugin: Ptr<'a, Plugin<'a, F>>,
    pub(crate) registry: Ptr<'a, Registry<F>>,
    pub(crate) requests: Ptr<'a, Requests>,
}

impl<'a, F: Function> LoadPluginContext<'a, F> {
    pub(crate) const fn new(
        plugin: Ptr<'a, Plugin<'a, F>>,
        registry: Ptr<'a, Registry<F>>,
        requests: Ptr<'a, Requests>,
    ) -> Self {
        Self {
            plugin,
            registry,
            requests,
        }
    }

    pub fn plugin(&self) -> &Plugin<'a, F> {
        self.plugin.as_ref()
    }

    pub fn registry(&self) -> &Registry<F> {
        self.registry.as_ref()
    }

    pub fn requests(&self) -> &Requests {
        self.requests.as_ref()
    }

    pub fn register_request(&mut self, request: F) -> Result<(), RegisterRequestError> {
        {
            if let Some(ord) = self
                .requests
                .as_ref()
                .iter()
                .find(|ord| ord.name() == request.name())
            {
                for input in ord.inputs().iter() {
                    request
                        .inputs()
                        .iter()
                        .find(|arg| *input == arg.ty())
                        .ok_or(RegisterRequestError::ArgumentsIncorrectly)?;
                }

                if *ord.output() != request.output().as_ref().map(|arg| arg.ty()) {
                    return Err(RegisterRequestError::ArgumentsIncorrectly);
                }
            } else {
                return Err(RegisterRequestError::NotFound);
            }
        }

        self.plugin.as_mut().requests.push(request);

        Ok(())
    }
}
