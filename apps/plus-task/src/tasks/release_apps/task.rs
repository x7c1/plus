use crate::core::support::release;
use crate::core::support::release::{ChangedFiles, PackageName, ReleaseTerminal};
use crate::TaskResult;

pub struct Task;

impl Task {
    pub fn release(&self, params: &Params) -> TaskResult<()> {
        println!("[start] #release params...{:#?}", params);
        params
            .files
            .filter_cargo_tomls(&params.target_packages)
            .try_for_each(|toml| release::start(&toml?))
    }

    pub fn release_dry_run(&self, params: &Params) -> TaskResult<()> {
        println!("[start] #dry_run params...{:#?}", params);
        params
            .files
            .filter_cargo_tomls(&params.target_packages)
            .try_for_each(|toml| release::start_dry_run(&toml?))
    }
}

#[derive(Debug)]
pub struct Params {
    pub files: ChangedFiles,
    pub target_packages: Vec<PackageName>,
}
