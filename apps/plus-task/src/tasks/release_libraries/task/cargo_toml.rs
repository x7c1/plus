use crate::error::Error::InvalidCargoToml;
use crate::TaskResult;
use serde_derive::Deserialize;
use std::fs;
use std::path::Path;

#[derive(Debug, Deserialize)]
pub struct CargoToml {
    pub package: CargoTomlPackage,
}

impl CargoToml {
    pub fn load(path: &Path) -> TaskResult<CargoToml> {
        let content = fs::read_to_string(path)?;
        let cargo_toml = toml::from_str(&content).map_err(InvalidCargoToml)?;
        Ok(cargo_toml)
    }
}

#[derive(Clone, Debug, Deserialize)]
pub struct CargoTomlPackage {
    pub name: String,
    pub version: String,
}
