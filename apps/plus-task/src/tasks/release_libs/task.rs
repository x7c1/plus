use crate::core::support::release;
use crate::core::support::release::{ChangedFiles, PackageName};
use crate::tasks::shared::git_arg::{GitConfig, HasGitConfig};
use crate::TaskResult;

pub struct Task;

impl Task {
    pub fn release(&self, params: &Params) -> TaskResult<()> {
        println!("[start] #release params...{:#?}", params);
        params
            .files
            .filter_cargo_tomls(&params.target_packages)
            .try_for_each(|toml| release::start(params, &toml?))
    }

    pub fn release_dry_run(&self, params: &Params) -> TaskResult<()> {
        println!("[start] #dry_run params...{:#?}", params);
        params
            .files
            .filter_cargo_tomls(&params.target_packages)
            .try_for_each(|toml| release::start_dry_run(params, &toml?))
    }
}

#[derive(Debug)]
pub struct Params {
    pub files: ChangedFiles,
    pub target_packages: Vec<PackageName>,
    pub git_config: GitConfig,
}

impl HasGitConfig for &Params {
    fn get_git_config(&self) -> &GitConfig {
        &self.git_config
    }
}
