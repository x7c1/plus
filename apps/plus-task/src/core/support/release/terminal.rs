use crate::core::build_mode::BuildMode::Release;
use crate::core::support::get_tar_path;
use crate::core::support::release::{CargoToml, CargoTomlPackage};
use crate::core::targets::BuildTarget;
use crate::error::Error::{AssetNotFound, CrateVersionNotFound, PackageAlreadyPublished};
use crate::tasks::shared::git_arg::{GitConfig, HasGitConfig};
use crate::TaskResult;
use shellwork::core::command;
use shellwork::core::command::{no_op, Runner, Unprepared};
use std::iter::FromIterator;
use std::path::{Path, PathBuf};
use toml::Value;

pub struct ReleaseTerminal<'a> {
    git_config: &'a GitConfig,
    cargo_toml_path: &'a Path,
    next_tag: String,
    package: &'a CargoTomlPackage,
}

impl ReleaseTerminal<'_> {
    pub fn load<'a, A: HasGitConfig>(
        git_config: &'a A,
        cargo_toml: &'a CargoToml,
    ) -> TaskResult<ReleaseTerminal<'a>> {
        let terminal = ReleaseTerminal {
            git_config: git_config.get_git_config(),
            cargo_toml_path: cargo_toml.path,
            next_tag: cargo_toml.contents.package.create_next_tag(),
            package: &cargo_toml.contents.package,
        };
        Ok(terminal)
    }

    pub fn cargo_publish(&self) -> TaskResult<()> {
        runner_to_publish(self.cargo_toml_path)
            .prepare(no_op::<crate::Error>)?
            .spawn()?;

        Ok(())
    }

    pub fn cargo_publish_dry_run(&self) -> TaskResult<()> {
        runner_to_publish(self.cargo_toml_path)
            .arg("--dry-run")
            .prepare(no_op::<crate::Error>)?
            .spawn()?;

        Ok(())
    }

    /// return Err if the package version already exists.
    pub fn cargo_search(&self) -> TaskResult<()> {
        let output = command::program("cargo")
            .args(&["search", &self.package.name])
            .prepare(no_op::<crate::Error>)?
            .capture()?;

        let stdout = output.stdout();
        let exists = if let Some(version) = extract_version(stdout.as_ref(), &self.package.name) {
            version == self.package.version
        } else {
            return Err(CrateVersionNotFound(self.package.clone()));
        };
        if exists {
            return Err(PackageAlreadyPublished(self.package.clone()));
        }
        Ok(())
    }

    /// caution: this method change .git/config file.
    pub fn git_config(&self) -> TaskResult<()> {
        command::program("git")
            .arg("config")
            .args(&["user.email", &self.git_config.user_email])
            .prepare(no_op::<crate::Error>)?
            .spawn()?;

        command::program("git")
            .arg("config")
            .args(&["user.name", &self.git_config.user_name])
            .prepare(no_op::<crate::Error>)?
            .spawn()?;

        Ok(())
    }

    pub fn git_tag(&self) -> TaskResult<()> {
        command::program("git")
            .args(self.git_config.as_cli_format())
            .arg("tag")
            .args(&["-a", &self.next_tag])
            .args(&["-m", ""])
            .prepare(no_op::<crate::Error>)?
            .spawn()?;

        Ok(())
    }

    pub fn git_push(&self) -> TaskResult<()> {
        command::program("git")
            .args(self.git_config.as_cli_format())
            .args(&["push", "origin", &self.next_tag])
            .prepare(no_op::<crate::Error>)?
            .spawn()?;

        Ok(())
    }

    pub fn gh_release_create(&self) -> TaskResult<()> {
        command::program("gh")
            .args(&["release", "create", &self.next_tag])
            .args(&["--notes", ""])
            .prepare(no_op::<crate::Error>)?
            .spawn()?;

        Ok(())
    }

    pub fn upload_assets(&self) -> TaskResult<()> {
        let upload = |path: &PathBuf| -> TaskResult<()> {
            command::program("gh")
                .args(&["release", "upload", &self.next_tag, &path.to_string_lossy()])
                .prepare(no_op::<crate::Error>)?
                .spawn()?;

            Ok(())
        };
        self.asset_paths().iter().try_for_each(upload)
    }

    pub fn all_assets_exist(&self) -> TaskResult<()> {
        for path in self.asset_paths() {
            if !path.exists() {
                return Err(AssetNotFound(path));
            }
        }
        Ok(())
    }

    fn asset_paths(&self) -> Vec<PathBuf> {
        let targets = BuildTarget::all();
        let iter = targets
            .iter()
            .map(|target| get_tar_path(*target, Release, self.package));

        Vec::from_iter(iter)
    }
}

fn runner_to_publish(toml: &Path) -> Runner<Unprepared> {
    command::program("cargo").args(&[
        "publish",
        "--manifest-path",
        toml.to_str().expect("path to Cargo.toml required"),
    ])
}

fn extract_version(toml_line: &str, package_name: &str) -> Option<String> {
    let value = toml_line.parse::<Value>().unwrap();
    value[package_name].as_str().map(|x| x.to_string())
}

#[cfg(test)]
mod tests {
    use crate::core::support::release::terminal::extract_version;
    use crate::core::support::release::CargoToml;
    use crate::TaskResult;
    use std::path::PathBuf;

    #[test]
    fn load_toml() -> TaskResult<()> {
        let path = PathBuf::from("../../libs/env-extractor/Cargo.toml");
        let toml = CargoToml::load(&path)?;
        assert_eq!(toml.contents.package.name, "env-extractor");
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
