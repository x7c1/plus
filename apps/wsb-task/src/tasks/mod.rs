use crate::{TaskOutput, TaskResult};
use clap_task::ClapTask;

#[macro_use]
pub mod build_apps;

pub fn define_all() -> Vec<Box<dyn ClapTask<TaskResult<TaskOutput>>>> {
    vec![build_apps::define()]
}
