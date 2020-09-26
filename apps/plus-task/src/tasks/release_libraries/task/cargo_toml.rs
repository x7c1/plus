use crate::error::Error::InvalidCargoToml;
use crate::TaskResult;
use serde_derive::Deserialize;
use std::fs;
use std::path::Path;

#[derive(Debug)]
pub struct CargoToml<'a> {
    pub path: &'a Path,
    pub contents: CargoTomlContents,
}

impl CargoToml<'_> {
    pub fn load(path: &Path) -> TaskResult<CargoToml> {
        let text = fs::read_to_string(path)?;
        let contents: CargoTomlContents = toml::from_str(&text).map_err(InvalidCargoToml)?;
        let cargo_toml = CargoToml { path, contents };
        Ok(cargo_toml)
    }

    pub fn is_private_version(&self) -> bool {
        self.contents.package.version == "0.0.0"
    }

    pub fn package_summary(&self) -> String {
        let package = &self.contents.package;
        format!("name: {}, version: {}", package.name, package.version)
    }
}

#[derive(Debug, Deserialize)]
pub struct CargoTomlContents {
    pub package: CargoTomlPackage,
}

#[derive(Clone, Debug, Deserialize)]
pub struct CargoTomlPackage {
    pub name: String,
    pub version: String,
}
