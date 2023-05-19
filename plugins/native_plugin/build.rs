use std::path::Path;

use anyhow::Result;
use fs_extra::dir::copy;
use fs_extra::dir::create_all;
use fs_extra::dir::CopyOptions;

fn main() {
    let current_dir = std::env::current_dir().unwrap();

    if let Some(dir) = current_dir
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

        copy_to_output(dir.as_str(), None).expect("Could not copy");
    }
}

fn copy_to_output(path: &str, to_dir: Option<&str>) -> Result<()> {
    let mut options = CopyOptions::new();
    options.overwrite = true;

    let from_path = Path::new(path);
    let out_path = Path::new(to_dir.unwrap_or("target"));

    println!("{}", out_path.display());

    create_all(out_path, true)?;
    copy(from_path, out_path, &options)?;

    Ok(())
}
