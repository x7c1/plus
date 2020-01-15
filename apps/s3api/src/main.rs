#[macro_use]
extern crate clap;

#[macro_use]
extern crate failure;

mod commands;

mod error;
pub use error::Result as S3ApiResult;

mod summary;
pub use summary::{CommandResult, ResponseSummary};

use clap::App;
use clap_task::TaskFinder;
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
    let definitions = commands::define_all();
    let finder = TaskFinder::new(init(), definitions)?;
    let task = finder.require_task()?;
    task.run()
}

fn init<'a, 'b>() -> App<'a, 'b> {
    App::new(crate_name!())
        .version(crate_version!())
        .author(crate_authors!())
}
