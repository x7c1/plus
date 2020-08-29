use crate::error::Error::{CrateVersionNotFound, PackageAlreadyPublished};
use crate::tasks::release_libraries::crates_io::extract_version;
use crate::tasks::release_libraries::task::crates_io::{CargoToml, CargoTomlPackage};
use crate::TaskResult;
use shellwork::core::command;
use shellwork::core::command::{no_op, Runner, Unprepared};
use std::path::Path;

pub struct ReleaseTerminal<'a> {
    cargo_toml_path: &'a Path,
    next_tag: String,
    package: CargoTomlPackage,
}

impl ReleaseTerminal<'_> {
    pub fn load(cargo_toml_path: &Path) -> TaskResult<ReleaseTerminal> {
        let cargo_toml = CargoToml::load(cargo_toml_path)?;
        let terminal = ReleaseTerminal {
            cargo_toml_path,
            next_tag: create_next_tag(&cargo_toml.package),
            package: cargo_toml.package,
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
            .args(&["-m", &format!("add tag: {}", self.next_tag)])
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
