mod utils;

#[cfg(test)]
mod tests {
    use crate::utils::{loader_init, VoidPluginManager};

    #[test]
    fn load_plugin() {
        let mut loader = loader_init(VoidPluginManager::new());

        if let Err(e) = loader.stop() {
            panic!("{:?}: {}", e, e.to_string());
        }
    }
}
