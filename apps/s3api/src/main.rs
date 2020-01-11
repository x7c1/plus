#[macro_use]
extern crate clap;
#[macro_use]
extern crate failure;

mod commands;
mod error;
use crate::commands::{Definition, Task};
use crate::error::Error::SubCommandMissing;
use clap::{App, ArgMatches};
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
    let definitions: Vec<Definition> = vec![commands::put_object::create()];
    let finder = TaskFinder::new(definitions)?;
    let task = finder.require_task()?;
    task.run()
}

struct TaskFinder<'a, 'b> {
    definitions: Vec<Definition<'a, 'b>>,
    matches: ArgMatches<'a>,
}

impl<'a, 'b> TaskFinder<'a, 'b> {
    fn new(definitions: Vec<Definition<'a, 'b>>) -> S3ApiResult<TaskFinder<'a, 'b>> {
        let app = definitions
            .iter()
            .map(|task| task.define)
            .fold(Self::init(), |acc, define| acc.subcommand(define()));

        Ok(TaskFinder {
            definitions,
            matches: app.get_matches_safe()?,
        })
    }

    fn init() -> App<'a, 'b> {
        App::new(crate_name!())
            .version(crate_version!())
            .author(crate_authors!())
    }

    fn require_task(&'a self) -> S3ApiResult<Task<'a, 'b>> {
        let to_task = |definition: &'a Definition| {
            self.matches
                .subcommand_matches(definition.name.to_string())
                .map(|matches| Task {
                    definition,
                    matches,
                })
        };
        self.definitions
            .iter()
            .find_map(to_task)
            .ok_or(SubCommandMissing)
    }
}
