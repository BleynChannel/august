use std::{
    cmp::Ordering,
    ffi::OsStr,
    fmt::Display,
    fs::File,
    io::{Read, Write},
    path::Path,
};

use semver::Version;
use serde::{Deserialize, Serialize};
use zip::{write::FileOptions, ZipArchive, ZipWriter};

use crate::{
    utils::{BundleFromError, BundleUnzipError},
    Depend, Info, Plugin,
};

use super::BundleZipError;

#[derive(Debug, Clone, PartialEq, Eq, PartialOrd, Ord, Serialize, Deserialize, Hash)]
pub struct Bundle {
    pub id: String,
    pub version: Version,
    pub format: String,
}

impl Bundle {
    pub fn from_filename<S>(filename: &S) -> Result<Self, BundleFromError>
    where
        S: AsRef<OsStr> + ?Sized,
    {
        let mut path = filename
            .as_ref()
            .to_str()
            .ok_or(BundleFromError::OsStrToStrFailed)?
            .to_string();

        let format = path
            .drain(path.rfind('.').ok_or(BundleFromError::FormatFailed)? + 1..)
            .collect::<String>();
        let version = path
            .drain(path.rfind("-v").ok_or(BundleFromError::VersionFailed)? + 2..path.len() - 1)
            .collect::<String>();
        let id = path
            .drain(..path.rfind("-v").ok_or(BundleFromError::IDFailed)?)
            .collect::<String>();

        if format.is_empty() {
            return Err(BundleFromError::FormatFailed);
        }
        if version.is_empty() {
            return Err(BundleFromError::VersionFailed);
        }
        if id.is_empty() {
            return Err(BundleFromError::IDFailed);
        }

        Ok(Self {
            id,
            version: Version::parse(version.as_str())?,
            format,
        })
    }
}

impl<ID: AsRef<str>> PartialEq<(ID, &Version)> for Bundle {
    fn eq(&self, (id, version): &(ID, &Version)) -> bool {
        self.id == *id.as_ref() && self.version == **version
    }
}

impl<O: Send + Sync, I: Info> PartialEq<Plugin<'_, O, I>> for Bundle {
    fn eq(&self, other: &Plugin<'_, O, I>) -> bool {
        self.id == other.info.bundle.id && self.version == other.info.bundle.version
    }
}

impl PartialEq<Depend> for Bundle {
    fn eq(&self, Depend { id: name, version }: &Depend) -> bool {
        self.id == *name && self.version == *version
    }
}

impl<ID: AsRef<str>> PartialOrd<(ID, &Version)> for Bundle {
    fn partial_cmp(&self, (id, version): &(ID, &Version)) -> Option<Ordering> {
        match self.id == *id.as_ref() {
            true => self.version.partial_cmp(*version),
            false => None,
        }
    }
}

impl<O: Send + Sync, I: Info> PartialOrd<Plugin<'_, O, I>> for Bundle {
    fn partial_cmp(&self, other: &Plugin<'_, O, I>) -> Option<Ordering> {
        match self.id == other.info.bundle.id {
            true => self.version.partial_cmp(&other.info.bundle.version),
            false => None,
        }
    }
}

impl PartialOrd<Depend> for Bundle {
    fn partial_cmp(&self, Depend { id: name, version }: &Depend) -> Option<Ordering> {
        match self.id == *name {
            true => self.version.partial_cmp(version),
            false => None,
        }
    }
}

impl Display for Bundle {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{}-v{}.{}", self.id, self.version, self.format)
    }
}

pub fn zip<S, F>(
    path: &S,
    target_path: &str,
    compression_method: zip::CompressionMethod,
    mut on_zip_file: F,
) -> Result<(), BundleZipError>
where
    S: AsRef<OsStr> + ?Sized,
    F: FnMut(&Path),
{
    let path = Path::new(path);
    let target_path =
        Path::new(target_path).join(path.file_name().ok_or(BundleZipError::NoNameFailed)?);

    if !path.is_dir() {
        return Err(BundleZipError::MissingBundleFailed);
    }

    match target_path.exists() {
        true if target_path.is_file() => Ok(()),
        true => Err(BundleZipError::ContainSameDirFailed),
        false => Ok({
            let file =
                File::create(target_path).map_err(|e| BundleZipError::CreateBundleFailed(e))?;
            let mut archive = ZipWriter::new(file);
            let options = FileOptions::default()
                .compression_method(compression_method)
                .unix_permissions(0o755);

            let mut buffer = Vec::new();
            for entry in walkdir::WalkDir::new(path)
                .into_iter()
                .filter_map(|e| e.ok())
            {
                let entry_path = entry.path();
                let name = entry_path.strip_prefix(path).unwrap();

                if entry_path.is_file() {
                    #[allow(deprecated)]
                    archive.start_file_from_path(name, options)?;
                    let mut f = File::open(entry_path)?;

                    f.read_to_end(&mut buffer)?;
                    archive.write_all(&buffer)?;
                    buffer.clear();
                } else if !name.as_os_str().is_empty() {
                    #[allow(deprecated)]
                    archive.add_directory_from_path(name, options)?;
                }

                on_zip_file(name);
            }
        }),
    }
}

pub fn unzip<S>(path: &S, target_path: &str) -> Result<Bundle, BundleUnzipError>
where
    S: AsRef<OsStr> + ?Sized,
{
    let path = Path::new(path);
    let target_path =
        Path::new(target_path).join(path.file_name().ok_or(BundleUnzipError::NoNameFailed)?);

    if !path.is_file() {
        return Err(BundleUnzipError::MissingBundleFailed);
    }

    match target_path.exists() {
        true if target_path.is_dir() => Ok(()),
        true => Err(BundleUnzipError::ContainSameFileFailed),
        false => Ok({
            let file = File::open(path)?;
            let mut archive = ZipArchive::new(file)?;
            archive.extract(&target_path)?;
        }),
    }?;

    Ok(Bundle::from_filename(target_path.file_name().unwrap())?)
}

#[test]
fn test_zip() {
    let temp_path = "../../bundles/temp";
    if !Path::new(temp_path).exists() {
        std::fs::create_dir_all(temp_path).unwrap();
    }

    let name = "plugin_a-v1.0.0.vpl";
    let path = format!("../../bundles/{name}");

    let target_path = temp_path;
    zip(&path, target_path, zip::CompressionMethod::Stored, |name| {
        println!("{}", name.display())
    })
    .unwrap();

    std::fs::remove_file(format!("{target_path}/{name}")).unwrap();
}

#[test]
fn test_unzip() {
    let temp_path = "../../bundles/temp";
    if !Path::new(temp_path).exists() {
        std::fs::create_dir_all(temp_path).unwrap();
    }

    let name = "plugin_b-v1.0.0.vpl";
    let path = format!("../../bundles/{name}");

    let target_path = temp_path;
    let bundle = unzip(&path, target_path).unwrap();
    println!("{bundle}");

    std::fs::remove_dir_all(format!("{target_path}/{name}")).unwrap();
}
