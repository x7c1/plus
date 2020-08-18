#[macro_use]
extern crate async_trait;

#[macro_use]
extern crate clap;

#[macro_use]
extern crate failure;

#[macro_use]
extern crate serde_derive;

#[macro_use]
extern crate plus_s3_macros;

mod commands;

mod error;
pub use error::Result as S3ApiResult;

mod output;
pub use output::{CommandOutput, CommandResult};

mod serialize;

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
            eprintln!("s3api failed: {:#?}", e);
            exit(1);
        }
    }
}

async fn run() -> CommandResult {
    let tasks = commands::define_all();
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
