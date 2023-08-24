mod utils;

#[cfg(test)]
mod main {
    use crate::utils::{get_plugin_path, loader_init};

    #[test]
    fn load_manager() {
        let mut loader = loader_init();

        if let Err(e) = loader.stop() {
            panic!("{:?}: {}", e, e.to_string());
        }
    }

    #[test]
    fn load_plugin() {
        let mut loader = loader_init();

        loader
            .load_plugin_now(get_plugin_path("native_plugin", "1.0.0").to_str().unwrap())
            .unwrap();

        if let Err(e) = loader.stop() {
            panic!("{:?}: {}", e, e.to_string());
        }
    }
}
