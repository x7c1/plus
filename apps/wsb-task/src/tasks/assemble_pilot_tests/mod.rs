mod task;
pub use task::Task;

use crate::tasks::shared;
use crate::tasks::shared::build_target;
use crate::TaskResult;
use clap::{App, ArgMatches, SubCommand};
use clap_task::ClapTask;

pub fn clap() -> Box<dyn ClapTask<TaskResult<()>>> {
    Box::new(Task)
}

#[async_trait]
impl ClapTask<TaskResult<()>> for Task {
    fn name(&self) -> &str {
        "assemble-pilot-tests"
    }

    fn design(&self) -> App {
        SubCommand::with_name(self.name())
            .about("Build and copy E2E tests.")
            .arg(build_target::arg())
    }

    async fn run<'a>(&'a self, matches: &'a ArgMatches<'a>) -> TaskResult<()> {
        let params = shared::Params::from_matches(matches)?;
        self.start(&params)
    }
}
