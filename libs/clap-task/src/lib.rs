#[macro_use]
extern crate async_trait;

#[macro_use]
extern crate failure;

mod error;
pub use error::Error;
pub use error::Result as ClapTaskResult;

use crate::error::Error::SubCommandMissing;
use clap::{App, ArgMatches};
use std::iter::FromIterator;

#[async_trait]
pub trait ClapTask<T>: Send + Sync {
    fn name(&self) -> &str;

    fn design(&self) -> App;

    async fn run<'a>(&'a self, matches: &'a ArgMatches<'a>) -> T;
}

pub trait ClapTasks<T> {
    fn to_apps(&self) -> Vec<App>;

    fn sub_matches<'a>(
        &'a self,
        matches: &'a ArgMatches,
    ) -> ClapTaskResult<(&'a Box<dyn ClapTask<T>>, &'a ArgMatches<'a>)>;
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

#[async_trait]
pub trait TaskRunner<T> {
    async fn run_matched_from<U>(&self, tasks: &U) -> ClapTaskResult<T>
    where
        U: ClapTasks<T>,
        U: Sync;
}

#[async_trait]
impl<'a, T: 'a> TaskRunner<T> for ArgMatches<'a> {
    async fn run_matched_from<U>(&self, tasks: &U) -> ClapTaskResult<T>
    where
        U: ClapTasks<T>,
        U: Sync,
    {
        let (task, sub_matches) = tasks.sub_matches(self)?;
        let item = task.run(sub_matches).await;
        Ok(item)
    }
}
