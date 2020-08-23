use crate::TaskResult;
use clap_task::ClapTask;

#[macro_use]
pub mod assemble_pilot_tests;

#[macro_use]
pub mod build_apps;

#[macro_use]
pub mod copy_artifact_files;

#[macro_use]
pub mod package_artifacts;

#[macro_use]
pub mod setup_artifacts_directory;

#[macro_use]
pub mod show_sizes;

#[macro_use]
pub mod strip_executables;

pub mod shared;

pub fn define_all() -> Vec<Box<dyn ClapTask<TaskResult<()>>>> {
    vec![
        assemble_pilot_tests::clap(),
        build_apps::clap(),
        copy_artifact_files::clap(),
        package_artifacts::clap(),
        setup_artifacts_directory::clap(),
        show_sizes::clap(),
        strip_executables::clap(),
    ]
}
