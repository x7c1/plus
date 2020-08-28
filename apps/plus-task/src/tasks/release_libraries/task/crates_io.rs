use crate::error::Error::{CrateVersionNotFound, InvalidCargoToml};
use crate::TaskResult;
use serde_derive::Deserialize;
use shellwork::core::command;
use shellwork::core::command::no_op;
use std::fs;
use std::path::Path;
use toml::Value;

#[derive(Debug, Deserialize)]
pub struct CargoToml {
    pub package: CargoTomlPackage,
}

impl CargoToml {
    pub fn load(path: &Path) -> TaskResult<CargoToml> {
        let content = fs::read_to_string(path)?;
        let cargo_toml = toml::from_str(&content).map_err(|e| InvalidCargoToml(e))?;
        Ok(cargo_toml)
    }
}

#[derive(Clone, Debug, Deserialize)]
pub struct CargoTomlPackage {
    pub name: String,
    pub version: String,
}

pub fn already_has(package: &CargoTomlPackage) -> TaskResult<bool> {
    let runner = command::program("cargo").args(&["search", &package.name]);
    let output = runner.prepare(no_op::<crate::Error>)?.capture()?;
    let stdout = output.stdout();
    let exists = if let Some(version) = extract_version(stdout.as_ref(), &package.name) {
        version == package.version
    } else {
        return Err(CrateVersionNotFound(package.clone()));
    };
    Ok(exists)
}

fn extract_version(toml_line: &str, package_name: &str) -> Option<String> {
    let value = toml_line.parse::<Value>().unwrap();
    value[package_name].as_str().map(|x| x.to_string())
}

#[cfg(test)]
mod tests {
    use crate::tasks::release_libraries::task::crates_io::{extract_version, CargoToml};
    use crate::TaskResult;
    use std::path::PathBuf;

    #[test]
    fn load_toml() -> TaskResult<()> {
        let path = PathBuf::from("Cargo.toml");
        let toml = CargoToml::load(&path)?;
        assert_eq!(toml.package.name, "plus-task");
        Ok(())
    }

    #[test]
    fn extract_package_version() -> TaskResult<()> {
        let line = r#"env-extractor = "0.1.2"    # Modules to extract environment variables."#;
        let version = extract_version(line, "env-extractor");
        assert_eq!(version, Some("0.1.2".to_string()));
        Ok(())
    }
}
