use crate::commands::artifact_size;
use crate::commands::support::Definable;
use crate::{TaskOutput, TaskResult};
use clap::{App, ArgMatches, SubCommand};
use clap_task::ClapTask;

pub fn define() -> Box<dyn ClapTask<TaskResult<TaskOutput>>> {
    Box::new(Task)
}

struct Task;

#[async_trait]
impl ClapTask<TaskResult<TaskOutput>> for Task {
    fn name(&self) -> &str {
        "show-sizes"
    }

    fn design(&self) -> App {
        SubCommand::with_name(self.name()).about("Show artifact sizes.")
    }

    async fn run<'a>(&'a self, _matches: &'a ArgMatches<'a>) -> TaskResult<TaskOutput> {
        let params = artifact_size::Params {};
        params.define()?.prepare(|_| TaskResult::Ok(()))?.spawn()?;
        Ok(TaskOutput::empty())
    }
}
