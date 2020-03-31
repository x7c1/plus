use crate::{TaskOutput, TaskResult};
use clap_task::ClapTask;

#[macro_use]
pub mod assemble_pilot_tests;

#[macro_use]
pub mod build_apps;

#[macro_use]
pub mod copy_artifact_files;

pub fn define_all() -> Vec<Box<dyn ClapTask<TaskResult<TaskOutput>>>> {
    vec![
        assemble_pilot_tests::define(),
        build_apps::define(),
        copy_artifact_files::define(),
    ]
}
