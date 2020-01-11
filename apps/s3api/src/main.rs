#[macro_use]
extern crate clap;
#[macro_use]
extern crate failure;

mod commands;
mod error;
use crate::commands::Task;
use crate::error::Error::SubCommandMissing;
use clap::App;
pub use error::Result as S3ApiResult;
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

fn run() -> S3ApiResult<()> {
    let tasks: Vec<Task> = vec![commands::put_object::create_task()];
    let mut app = create_app();
    for task in &tasks {
        app = app.subcommand((task.create)());
    }
    let arg_matches = app.get_matches_safe()?;
    let (sub_matches, run) = tasks
        .iter()
        .find_map(|task| {
            arg_matches
                .subcommand_matches(task.name.to_string())
                .map(|matches| (matches, task.run))
        })
        .ok_or(SubCommandMissing)?;

    run(sub_matches)
}

fn create_app<'a, 'b>() -> App<'a, 'b> {
    App::new(crate_name!())
        .version(crate_version!())
        .author(crate_authors!())
}
