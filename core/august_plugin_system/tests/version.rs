mod utils;

#[cfg(test)]
mod tests {
    use std::path::PathBuf;

    use crate::utils::{get_plugin_path, loader_init, VoidPluginManager};

    const FORMAT: &str = "vpl";
    const PATH: &str = "versions";

    const TOOLS: [(&str, &str); 2] = [("paint", "1.0.0"), ("photoshop", "1.0.0")];

    fn get_versions_path() -> Vec<PathBuf> {
        let id = format!("{PATH}/brush");

        vec![
            get_plugin_path(id.as_str(), "1.0.0", FORMAT),
            get_plugin_path(id.as_str(), "2.0.0", FORMAT),
            get_plugin_path(id.as_str(), "3.0.0", FORMAT),
        ]
    }

    fn get_tools_path() -> Vec<PathBuf> {
        TOOLS
            .into_iter()
            .map(|(id, version)| get_plugin_path(format!("{PATH}/{id}").as_str(), version, FORMAT))
            .collect()
    }

    #[test]
    fn load_another_version() {
        let mut loader = loader_init(VoidPluginManager::new());

        let paths = get_versions_path();

        let plugins = loader
            .load_plugins(
                paths
                    .iter()
                    .map(|path| path.to_str().unwrap())
                    .collect::<Vec<_>>(),
            )
            .unwrap();

        for bundle in plugins {
            println!("Plugin = {}", bundle);
        }
    }

    #[test]
    fn load_version_as_dependency() {
        let mut loader = loader_init(VoidPluginManager::new());

        let paths: Vec<_> = get_versions_path()
            .into_iter()
            .chain(get_tools_path().into_iter())
            .collect();

        let plugins = loader
            .load_plugins(
                paths
                    .iter()
                    .map(|path| path.to_str().unwrap())
                    .collect::<Vec<_>>(),
            )
            .unwrap();

        for bundle in plugins {
            println!("Plugin = {}", bundle);
        }
    }
}
