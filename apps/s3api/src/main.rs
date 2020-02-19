#[macro_use]
extern crate clap;

#[macro_use]
extern crate failure;

mod commands;

mod error;
pub use error::Result as S3ApiResult;

mod summary;
use clap_task::{ClapTasks, TaskRunner};
pub use summary::{CommandOutput, CommandResult};

use clap::App;
use std::process::exit;

fn main() {
    match run() {
        Ok(output) => {
            println!("{:?}", output);
        }
        Err(e) => {
            eprintln!("failed: {:#?}", e);
            exit(1);
        }
    }
}

fn run() -> CommandResult {
    let tasks = commands::define_all();
    init()
        .subcommands(tasks.to_apps())
        .get_matches()
        .run_matched_from(&tasks)?
}

fn init<'a, 'b>() -> App<'a, 'b> {
    App::new(crate_name!())
        .version(crate_version!())
        .author(crate_authors!())
}
