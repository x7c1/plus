mod task;
use task::Task;

use crate::TaskResult;
use clap::{App, ArgMatches, SubCommand};
use clap_task::ClapTask;

pub fn clap() -> Box<dyn ClapTask<TaskResult<()>>> {
    Box::new(Task)
}

#[async_trait]
impl ClapTask<TaskResult<()>> for Task {
    fn name(&self) -> &str {
        "show-sizes"
    }

    fn design(&self) -> App {
        SubCommand::with_name(self.name()).about("Show artifact sizes.")
    }

    async fn run<'a>(&'a self, _matches: &'a ArgMatches<'a>) -> TaskResult<()> {
        self.start()
    }
}
