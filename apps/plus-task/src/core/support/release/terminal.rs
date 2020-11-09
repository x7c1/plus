use crate::core::support::release::{CargoToml, CargoTomlPackage};
use crate::core::targets::BuildTarget;
use crate::error::Error::{CrateVersionNotFound, PackageAlreadyPublished};
use crate::TaskResult;
use shellwork::core::command;
use shellwork::core::command::{no_op, Runner, Unprepared};
use std::path::{Path, PathBuf};
use toml::Value;

pub struct ReleaseTerminal<'a> {
    cargo_toml_path: &'a Path,
    next_tag: String,
    package: &'a CargoTomlPackage,
}

impl ReleaseTerminal<'_> {
    pub fn load<'a>(cargo_toml: &'a CargoToml) -> TaskResult<ReleaseTerminal<'a>> {
        let terminal = ReleaseTerminal {
            cargo_toml_path: cargo_toml.path,
            next_tag: create_next_tag(&cargo_toml.contents.package),
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

    pub fn git_config(&self) -> TaskResult<()> {
        // rf. https://github.community/t/github-actions-bot-email-address/17204/4
        command::program("git")
            .arg("config")
            .args(&[
                "user.email",
                "41898282+github-actions[bot]@users.noreply.github.com",
            ])
            .prepare(no_op::<crate::Error>)?
            .spawn()?;

        command::program("git")
            .arg("config")
            .args(&["user.name", "github-actions[bot]"])
            .prepare(no_op::<crate::Error>)?
            .spawn()?;

        Ok(())
    }

    pub fn git_tag(&self) -> TaskResult<()> {
        command::program("git")
            .arg("tag")
            .args(&["-a", &self.next_tag])
            .args(&["-m", ""])
            .prepare(no_op::<crate::Error>)?
            .spawn()?;

        Ok(())
    }

    pub fn git_push(&self) -> TaskResult<()> {
        command::program("git")
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
        let targets = BuildTarget::all();
        let mut target_asset_paths = targets.iter().map(|target| {
            PathBuf::from(".")
                .join("dist")
                .join("release")
                .join(target.as_triple())
                .join(format!("{}-{}.tar.xz", &self.next_tag, target.as_triple()))
        });
        let upload = |path: PathBuf| -> TaskResult<()> {
            command::program("gh")
                .args(&["release", "upload", &self.next_tag, &path.to_string_lossy()])
                .prepare(no_op::<crate::Error>)?
                .spawn()?;

            Ok(())
        };
        target_asset_paths.try_for_each(upload)
    }
}

fn runner_to_publish(toml: &Path) -> Runner<Unprepared> {
    command::program("cargo").args(&[
        "publish",
        "--manifest-path",
        toml.to_str().expect("path to Cargo.toml required"),
    ])
}

fn create_next_tag(package: &CargoTomlPackage) -> String {
    format!(
        "{prefix}-v{version}",
        prefix = package.name,
        version = package.version
    )
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
