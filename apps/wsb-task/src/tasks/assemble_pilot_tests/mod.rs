mod task;
pub use task::Task;

use crate::tasks::shared::build_target;
use crate::tasks::SharedParams;
use crate::{TaskOutput, TaskResult};
use clap::{App, ArgMatches, SubCommand};
use clap_task::ClapTask;

pub fn clap() -> Box<dyn ClapTask<TaskResult<TaskOutput>>> {
    Box::new(Task)
}

#[async_trait]
impl ClapTask<TaskResult<TaskOutput>> for Task {
    fn name(&self) -> &str {
        "assemble-pilot-tests"
    }

    fn design(&self) -> App {
        SubCommand::with_name(self.name())
            .about("Build and copy E2E tests.")
            .arg(build_target::arg())
    }

    async fn run<'a>(&'a self, matches: &'a ArgMatches<'a>) -> TaskResult<TaskOutput> {
        let params = SharedParams::from_matches(matches)?;
        self.run(&params)?;
        Ok(TaskOutput::empty())
    }
}
