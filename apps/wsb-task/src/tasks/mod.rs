use crate::{TaskOutput, TaskResult};
use clap_task::ClapTask;

pub mod build_apps;

pub fn define_all() -> Vec<Box<dyn ClapTask<TaskResult<TaskOutput>>>> {
    vec![build_apps::define()]
}
