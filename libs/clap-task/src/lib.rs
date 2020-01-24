#[macro_use]
extern crate failure;

mod error;
pub use error::Error;
pub use error::Result as ClapTaskResult;

use crate::error::Error::SubCommandMissing;
use clap::{App, ArgMatches};
use std::iter::FromIterator;

pub trait ClapTask<T> {
    fn design(&self) -> App;

    fn name(&self) -> &str;

    fn run(&self, matches: &ArgMatches) -> T;
}

pub trait ClapTasks<T> {
    fn to_apps(&self) -> Vec<App>;

    fn sub_matches<'a>(
        &'a self,
        matches: &'a ArgMatches,
    ) -> ClapTaskResult<(&'a Box<dyn ClapTask<T>>, &'a ArgMatches<'a>)>;

    fn run_matched(&self, matches: &ArgMatches) -> ClapTaskResult<T> {
        let (task, sub_matches) = self.sub_matches(matches)?;
        Ok(task.run(sub_matches))
    }
}

impl<T> ClapTasks<T> for Vec<Box<dyn ClapTask<T>>> {
    fn to_apps(&self) -> Vec<App> {
        let apps = self.iter().map(|task| task.design());
        Vec::from_iter(apps)
    }

    fn sub_matches<'a>(
        &'a self,
        matches: &'a ArgMatches,
    ) -> ClapTaskResult<(&'a Box<dyn ClapTask<T>>, &'a ArgMatches<'a>)> {
        self.iter()
            .find_map(|x| matches.subcommand_matches(x.name()).map(|m| (x, m)))
            .ok_or_else(|| SubCommandMissing)
    }
}
