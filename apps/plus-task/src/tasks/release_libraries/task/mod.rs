mod changed_files;
pub use changed_files::ChangedFiles;

mod terminal;
pub use terminal::ReleaseTerminal;

mod cargo_toml;
pub use cargo_toml::{CargoToml, CargoTomlPackage};

use crate::TaskResult;
use std::path::Path;

pub struct Task;

impl Task {
    pub fn release(&self, params: &Params) -> TaskResult<()> {
        println!("[start] #release params...{:#?}", params);
        let mut tomls = params.files.lib_cargo_tomls();
        tomls.try_for_each(|toml| self.start(toml))?;
        Ok(())
    }

    pub fn release_dry_run(&self, params: &Params) -> TaskResult<()> {
        println!("[start] #dry_run params...{:#?}", params);
        let mut tomls = params.files.lib_cargo_tomls();
        tomls.try_for_each(|toml| self.start_dry_run(toml))?;
        Ok(())
    }

    fn start(&self, toml: &Path) -> TaskResult<()> {
        let terminal = ReleaseTerminal::load(toml)?;
        terminal.cargo_publish()?;
        terminal.git_config()?;
        terminal.git_tag()?;
        terminal.git_push()?;
        Ok(())
    }

    fn start_dry_run(&self, toml: &Path) -> TaskResult<()> {
        let terminal = ReleaseTerminal::load(toml)?;
        terminal.cargo_search()?;
        terminal.cargo_publish_dry_run()?;
        Ok(())
    }
}

#[derive(Debug)]
pub struct Params {
    pub files: ChangedFiles,
}
