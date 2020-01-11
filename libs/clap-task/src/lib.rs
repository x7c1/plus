#[macro_use]
extern crate failure;

mod error;
pub use error::Error;
pub use error::Result as ClapTaskResult;

use crate::error::Error::SubCommandMissing;
use clap::{App, ArgMatches};

pub struct Definition<'a, 'b, T> {
    pub name: String,
    pub define: fn() -> App<'a, 'b>,
    pub run: fn(matches: &ArgMatches) -> T,
}

pub struct Task<'a, 'b, T> {
    pub definition: &'a Definition<'a, 'b, T>,
    pub matches: &'a ArgMatches<'a>,
}

impl<T> Task<'_, '_, T> {
    pub fn run(&self) -> T {
        let run = self.definition.run;
        run(self.matches)
    }
}

pub struct TaskFinder<'a, 'b, T> {
    definitions: Vec<Definition<'a, 'b, T>>,
    matches: ArgMatches<'a>,
}

impl<'a, 'b, T> TaskFinder<'a, 'b, T> {
    pub fn new(
        app: App<'a, 'b>,
        definitions: Vec<Definition<'a, 'b, T>>,
    ) -> ClapTaskResult<TaskFinder<'a, 'b, T>> {
        let app = definitions
            .iter()
            .map(|task| task.define)
            .fold(app, |acc, define| acc.subcommand(define()));

        Ok(TaskFinder {
            definitions,
            matches: app.get_matches_safe()?,
        })
    }

    pub fn require_task(&'a self) -> ClapTaskResult<Task<'a, 'b, T>> {
        let to_task = |definition: &'a Definition<'a, 'b, T>| {
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
