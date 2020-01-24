#[macro_use]
extern crate clap;

#[macro_use]
extern crate failure;

mod commands;

mod errors;
pub use errors::Result as S3ApiResult;

mod summary;
use clap_task::ClapTasks;
pub use summary::{CommandResult, ResponseSummary};

use clap::App;
use std::process::exit;

fn main() {
    println!("Hello, world!");

    match run() {
        Ok(response) => {
            println!("succeeded: {:#?}", response);
        }
        Err(e) => {
            println!("failed: {:#?}", e);
            exit(1);
        }
    }
}

fn run() -> CommandResult {
    let tasks = commands::define_all();
    let app = init().subcommands(tasks.to_apps());
    let matches = app.get_matches();
    tasks.run_matched(&matches)?
}

fn init<'a, 'b>() -> App<'a, 'b> {
    App::new(crate_name!())
        .version(crate_version!())
        .author(crate_authors!())
}
