use crate::core::support::release::{CargoToml, PackageName};
use crate::Error::UnknownPackageName;
use crate::TaskResult;
use serde::Deserialize;
use std::path::{Path, PathBuf};
use std::str::FromStr;

#[derive(Debug, Deserialize)]
pub struct ChangedFiles {
    pub paths: Vec<PathBuf>,
}

impl ChangedFiles {
    pub fn filter_cargo_tomls<'a>(
        &'a self,
        names: &'a [PackageName],
    ) -> impl Iterator<Item = TaskResult<CargoToml<'a>>> {
        self.cargo_toml_paths()
            .map(CargoToml::load)
            .filter_map(move |toml| match toml {
                Ok(toml) if toml.in_package(&names) => Some(Ok(toml)),
                Ok(_) => None,
                Err(UnknownPackageName(_)) => None,
                Err(e) => Some(Err(e)),
            })
    }

    pub fn cargo_toml_paths(&self) -> impl Iterator<Item = &Path> {
        self.paths.iter().filter_map(|path| {
            if path.ends_with("Cargo.toml") {
                Some(path.as_path())
            } else {
                None
            }
        })
    }
}

impl FromStr for ChangedFiles {
    type Err = crate::Error;

    fn from_str(x: &str) -> Result<Self, Self::Err> {
        let files = ChangedFiles {
            paths: serde_json::from_str(x)?,
        };
        Ok(files)
    }
}

#[cfg(test)]
mod tests {
    use super::ChangedFiles;
    use crate::TaskResult;
    use std::iter::FromIterator;
    use std::path::{Path, PathBuf};
    use std::str::FromStr;

    #[test]
    fn create_from_str() -> TaskResult<()> {
        let files = ChangedFiles::from_str(r#"["x1","x2","x3"]"#)?;
        assert_eq!(
            files.paths,
            [
                PathBuf::from("x1"),
                PathBuf::from("x2"),
                PathBuf::from("x3"),
            ]
        );
        Ok(())
    }

    #[test]
    fn cargo_toml_paths() -> TaskResult<()> {
        let files = ChangedFiles {
            paths: vec![
                "Makefile".into(),
                "libs/x1/y1/Cargo.toml".into(),
                "libs/x1/y1/FooCargo.toml".into(),
                "Cargo.toml".into(),
                "libs/x1/y2/z3/Cargo.toml".into(),
                "apps/a1/Cargo.toml".into(),
            ],
        };
        let tomls = files.cargo_toml_paths();
        let expected: Vec<&Path> = vec![
            Path::new("libs/x1/y1/Cargo.toml"),
            Path::new("Cargo.toml"),
            Path::new("libs/x1/y2/z3/Cargo.toml"),
            Path::new("apps/a1/Cargo.toml"),
        ];
        assert_eq!(Vec::from_iter(tomls), expected);
        Ok(())
    }
}
