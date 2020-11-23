use crate::core::support::release::packages::PackageSettings;
use crate::core::support::release::PackageName;
use crate::error::Error::InvalidCargoToml;
use crate::TaskResult;
use serde_derive::Deserialize;
use std::fs;
use std::path::Path;
use std::str::FromStr;

#[derive(Debug)]
pub struct CargoToml<'a> {
    pub path: &'a Path,
    pub contents: CargoTomlContents,
    pub settings: PackageSettings,
}

impl CargoToml<'_> {
    pub fn load(path: &Path) -> TaskResult<CargoToml> {
        let text = fs::read_to_string(path)?;
        let contents: CargoTomlContents = toml::from_str(&text).map_err(InvalidCargoToml)?;
        let settings = PackageSettings::get(PackageName::from_str(&contents.package.name)?);
        let cargo_toml = CargoToml {
            path,
            contents,
            settings,
        };
        Ok(cargo_toml)
    }

    pub fn in_package(&self, names: &[PackageName]) -> bool {
        names.contains(&self.settings.package_name)
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

impl CargoTomlPackage {
    pub fn create_next_tag(&self) -> String {
        format!(
            "{prefix}-v{version}",
            prefix = self.name,
            version = self.version
        )
    }
}
