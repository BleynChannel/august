use copy_to_output::copy_to_output;

fn main() {
    if let Some(dir) = std::env::current_dir()
        .unwrap()
        .read_dir()
        .unwrap()
        .find_map(|entry| {
            if let Ok(entry) = entry {
                if entry.path().is_dir() {
                    if entry.file_name().to_str().unwrap().starts_with("plugin.") {
                        return Some(entry);
                    }
                }
            }
            None
        })
        .map(|entry| entry.file_name().to_str().unwrap().to_string())
    {
        println!("cargo:rerun-if-changed={dir}/*");
        copy_to_output(dir.as_str(), &std::env::var("PROFILE").unwrap()).expect("Could not copy");
    }
}
