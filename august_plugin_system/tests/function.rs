mod utils;

#[cfg(test)]
mod tests {
    use crate::utils::{loader_init, managers::VoidPluginManager};

    #[test]
    fn load_plugin() {
        let mut loader = loader_init(VoidPluginManager::new());

		// loader.
    }
}
