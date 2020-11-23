mod cargo_toml;

pub use cargo_toml::{CargoToml, CargoTomlContents, CargoTomlPackage};

mod terminal;

pub use terminal::ReleaseTerminal;

mod changed_files;

pub use changed_files::ChangedFiles;

mod packages;

pub use packages::PackageName;

use crate::tasks::shared::git_arg::HasGitConfig;
use crate::TaskResult;

pub fn start<A>(git_config: A, cargo_toml: &CargoToml) -> TaskResult<()>
where
    A: HasGitConfig,
{
    let terminal = ReleaseTerminal::load(&git_config, &cargo_toml)?;
    if cargo_toml.settings.on_crates_io {
        terminal.cargo_publish()?;
    }
    if cargo_toml.settings.has_git_tag {
        terminal.git_tag()?;
        terminal.git_push()?;
    }
    if cargo_toml.settings.has_github_release {
        terminal.gh_release_create()?;
    }
    if cargo_toml.settings.has_github_assets {
        terminal.upload_assets()?;
    }
    Ok(())
}

pub fn start_dry_run<A>(git_config: A, cargo_toml: &CargoToml) -> TaskResult<()>
where
    A: HasGitConfig,
{
    let terminal = ReleaseTerminal::load(&git_config, &cargo_toml)?;
    if cargo_toml.settings.on_crates_io {
        terminal.cargo_search()?;
        terminal.cargo_publish_dry_run()?;
    }
    if cargo_toml.settings.has_github_assets {
        terminal.all_assets_exist()?;
    }
    Ok(())
}
