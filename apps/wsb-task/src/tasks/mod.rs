use crate::{TaskOutput, TaskResult};
use clap_task::ClapTask;

#[macro_use]
pub mod build_apps;

#[macro_use]
pub mod assemble_pilot_tests;

pub fn define_all() -> Vec<Box<dyn ClapTask<TaskResult<TaskOutput>>>> {
    vec![build_apps::define(), assemble_pilot_tests::define()]
}
