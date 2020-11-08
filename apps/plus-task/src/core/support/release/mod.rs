mod cargo_toml;
pub use cargo_toml::{CargoToml, CargoTomlContents, CargoTomlPackage};

mod terminal;
pub use terminal::ReleaseTerminal;

mod changed_files;
pub use changed_files::ChangedFiles;

mod packages;
pub use packages::PackageName;

use crate::TaskResult;

pub fn start(cargo_toml: &CargoToml) -> TaskResult<()> {
    let terminal = ReleaseTerminal::load(&cargo_toml)?;
    if cargo_toml.settings.on_crates_io {
        terminal.cargo_publish()?;
    }
    if cargo_toml.settings.has_git_tag {
        terminal.git_config()?;
        terminal.git_tag()?;
        terminal.git_push()?;
    }
    if cargo_toml.settings.has_github_release {
        terminal.gh_release_create()?;
    }
    Ok(())
}

pub fn start_dry_run(cargo_toml: &CargoToml) -> TaskResult<()> {
    let terminal = ReleaseTerminal::load(&cargo_toml)?;
    if cargo_toml.settings.on_crates_io {
        terminal.cargo_search()?;
        terminal.cargo_publish_dry_run()?;
    }
    Ok(())
}
