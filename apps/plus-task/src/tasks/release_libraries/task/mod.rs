mod changed_files;
pub use changed_files::ChangedFiles;

pub mod crates_io;

use crates_io::CargoToml;

use crate::core::support::program_exists;
use crate::error::Error::PackageAlreadyPublished;
use crate::TaskResult;
use shellwork::core::command;
use shellwork::core::command::{no_op, Runner, Unprepared};
use std::path::Path;

pub struct Task;

impl Task {
    pub fn release(&self, params: &Params) -> TaskResult<()> {
        println!("[start] #release params...{:#?}", params);

        let mut tomls = params.files.lib_cargo_tomls();
        tomls.try_for_each(|toml| self.start(toml))?;
        Ok(())
    }

    pub fn dry_run(&self, params: &Params) -> TaskResult<()> {
        println!("[start] #dry_run params...{:#?}", params);

        let mut tomls = params.files.lib_cargo_tomls();
        tomls.try_for_each(|toml| self.start_dry_run(toml))?;
        Ok(())
    }

    fn start(&self, toml: &Path) -> TaskResult<()> {
        let runner = self.runner_to_publish(toml);
        runner.prepare(program_exists)?.spawn()?;

        /*
        github-actions[bot] <41898282+github-actions[bot]@users.noreply.github.com>
        git config user.email "abcde@example.com"
        git config user.name "abcde"
         */

        // rf. https://github.community/t/github-actions-bot-email-address/17204/4
        let r1 = command::program("git").arg("config").args(&[
            "user.email",
            "41898282+github-actions[bot]@users.noreply.github.com",
        ]);

        let output = r1.prepare(no_op::<crate::Error>)?.capture()?;
        println!("git config...{:#?}", output);

        let r2 = command::program("git")
            .arg("config")
            .args(&["user.name", "github-actions[bot]"]);

        let output = r2.prepare(no_op::<crate::Error>)?.capture()?;
        println!("git config...{:#?}", output);

        let cargo_toml = CargoToml::load(toml)?;
        let tag = format!(
            "{}-v{}",
            cargo_toml.package.name, cargo_toml.package.version
        );
        let runner1 = command::program("git")
            .arg("tag")
            .args(&["-a", &tag])
            .args(&["-m", &format!("add tag: {}", tag)]);

        let output = runner1.prepare(no_op::<crate::Error>)?.capture()?;
        println!("git tag...{:#?}", output);

        let runner2 = command::program("git").args(&["push", "origin", &tag]);
        let output2 = runner2.prepare(no_op::<crate::Error>)?.capture()?;
        println!("git push...{:#?}", output2);

        Ok(())
    }

    fn start_dry_run(&self, toml: &Path) -> TaskResult<()> {
        let cargo_toml = CargoToml::load(toml)?;
        if crates_io::already_has(&cargo_toml.package)? {
            return Err(PackageAlreadyPublished(cargo_toml.package));
        }
        let runner = self.runner_to_publish(toml).arg("--dry-run");
        runner.prepare(program_exists)?.spawn()?;
        Ok(())
    }

    fn runner_to_publish(&self, toml: &Path) -> Runner<Unprepared> {
        command::program("cargo").args(&[
            "publish",
            "--manifest-path",
            toml.to_str().expect("path to Cargo.toml required"),
        ])
    }
}

#[derive(Debug)]
pub struct Params {
    pub files: ChangedFiles,
}
