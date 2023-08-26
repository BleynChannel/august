mod utils;

#[cfg(test)]
mod main {
    use crate::utils::{get_plugin_path, loader_init};

    #[test]
    fn load_manager() {
        loader_init();
    }

    #[test]
    fn load_plugin() {
        let mut loader = loader_init();

        loader
            .load_plugin_now(get_plugin_path("native_plugin", "1.0.0").to_str().unwrap())
            .unwrap();
    }
}
