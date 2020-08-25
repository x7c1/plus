use crate::core::env::artifacts_dir;
use crate::core::support::program_exists;
use crate::TaskResult;
use serde::Deserialize;
use shellwork::core::command;
use shellwork::core::command::{Runner, Unprepared};
use std::path::{Path, PathBuf};
use std::str::FromStr;

pub struct Task;

impl Task {
    pub fn start(&self, params: &Params) -> TaskResult<()> {
        println!("params...{:#?}", params);
        let runner = self.runner().prepare(program_exists)?;
        runner.spawn()?;
        Ok(())
    }

    fn runner(&self) -> Runner<Unprepared> {
        command::program("tree")
            // specify max tree depth to descend
            .args(&["-L", "2"])
            // use ANSI line graphics hack when printing indentation lines
            .arg("-A")
            // sort output by change time
            .arg("-c")
            // print directory sizes
            .arg("--du")
            // print human readable file size in SI units (powers of 1000)
            .arg("--si")
            // list directories before files
            .arg("--dirsfirst")
            .arg(artifacts_dir())
    }
}

#[derive(Debug)]
pub struct Params {
    pub files: ChangedFiles,
}

#[derive(Debug, Deserialize)]
pub struct ChangedFiles {
    pub paths: Vec<PathBuf>,
}

impl ChangedFiles {
    pub fn lib_cargo_tomls(&self) -> impl Iterator<Item = &Path> {
        self.paths.iter().filter_map(|path| {
            if path.starts_with("libs/") && path.ends_with("Cargo.toml") {
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
    use crate::tasks::release_libraries::task::ChangedFiles;
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
    fn filter_lib_cargo_toml() -> TaskResult<()> {
        let files = ChangedFiles {
            paths: vec![
                "Makefile".into(),
                "libs/x1/y1/Cargo.toml".into(),
                "libs/x1/y1/FooCargo.toml".into(),
                "Cargo.toml".into(),
                "libs/x1/y2/Cargo.toml".into(),
                "apps/a1/Cargo.toml".into(),
            ],
        };
        let tomls = files.lib_cargo_tomls();
        let expected: Vec<&Path> = vec![
            Path::new("libs/x1/y1/Cargo.toml"),
            Path::new("libs/x1/y2/Cargo.toml"),
        ];
        assert_eq!(Vec::from_iter(tomls), expected);
        Ok(())
    }
}
