#[macro_use]
extern crate async_trait;

#[macro_use]
extern crate clap;

#[macro_use]
extern crate failure;

pub mod commands;
pub mod core;

mod error;
pub use error::Error;
pub use error::Result as TaskResult;

mod output;
pub use output::TaskOutput;
mod tasks;

use clap::App;
use clap_task::{ClapTasks, TaskRunner};
use std::process::exit;

#[tokio::main]
async fn main() {
    match run().await {
        Ok(output) => {
            println!("{}", output.as_str());
        }
        Err(e) => {
            eprintln!("wsb-task failed: {:#?}", e);
            exit(1);
        }
    }
}

async fn run() -> TaskResult<TaskOutput> {
    let tasks = tasks::define_all();
    init()
        .subcommands(tasks.to_apps())
        .get_matches()
        .run_matched_from(&tasks)
        .await?
}

fn init<'a, 'b>() -> App<'a, 'b> {
    App::new(crate_name!())
        .version(crate_version!())
        .author(crate_authors!())
}
