use crate::{TaskOutput, TaskResult};
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
pub mod show_sizes;

#[macro_use]
pub mod strip_executables;

mod params;
pub use params::SharedParams;

pub fn define_all() -> Vec<Box<dyn ClapTask<TaskResult<TaskOutput>>>> {
    vec![
        assemble_pilot_tests::clap(),
        build_apps::clap(),
        copy_artifact_files::define(),
        package_artifacts::define(),
        show_sizes::define(),
        strip_executables::define(),
    ]
}
