mod task;
use task::Task;

use crate::{TaskOutput, TaskResult};
use clap::{App, ArgMatches, SubCommand};
use clap_task::ClapTask;

pub fn clap() -> Box<dyn ClapTask<TaskResult<TaskOutput>>> {
    Box::new(Task)
}

#[async_trait]
impl ClapTask<TaskResult<TaskOutput>> for Task {
    fn name(&self) -> &str {
        "show-sizes"
    }

    fn design(&self) -> App {
        SubCommand::with_name(self.name()).about("Show artifact sizes.")
    }

    async fn run<'a>(&'a self, _matches: &'a ArgMatches<'a>) -> TaskResult<TaskOutput> {
        self.start()?;
        Ok(TaskOutput::empty())
    }
}
