use crate::core::support::release::{CargoToml, ChangedFiles, PackageName, ReleaseTerminal};
use crate::TaskResult;

pub struct Task;

impl Task {
    pub fn release(&self, params: &Params) -> TaskResult<()> {
        println!("[start] #release params...{:#?}", params);
        params
            .files
            .filter_cargo_tomls(&params.target_packages)
            .try_for_each(|toml| self.start(&toml?))
    }

    pub fn release_dry_run(&self, params: &Params) -> TaskResult<()> {
        println!("[start] #dry_run params...{:#?}", params);
        params
            .files
            .filter_cargo_tomls(&params.target_packages)
            .try_for_each(|toml| self.start_dry_run(&toml?))
    }

    fn start(&self, cargo_toml: &CargoToml) -> TaskResult<()> {
        let terminal = ReleaseTerminal::load(&cargo_toml)?;
        if cargo_toml.settings.on_crates_io {
            terminal.cargo_publish()?;
        }
        if cargo_toml.settings.has_git_tag {
            terminal.git_config()?;
            terminal.git_tag()?;
            terminal.git_push()?;
        }
        Ok(())
    }

    fn start_dry_run(&self, cargo_toml: &CargoToml) -> TaskResult<()> {
        let terminal = ReleaseTerminal::load(&cargo_toml)?;
        if cargo_toml.settings.on_crates_io {
            terminal.cargo_search()?;
            terminal.cargo_publish_dry_run()?;
        }
        Ok(())
    }
}

#[derive(Debug)]
pub struct Params {
    pub files: ChangedFiles,
    pub target_packages: Vec<PackageName>,
}
