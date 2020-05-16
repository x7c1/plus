mod task;
pub use task::Task;

use crate::tasks::SharedParams;
use crate::{TaskOutput, TaskResult};
use clap::{App, Arg, ArgMatches, SubCommand};
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
            .arg(
                Arg::with_name("target")
                    .long("target")
                    .required(true)
                    .takes_value(true)
                    .help("Build for the target label."),
            )
    }

    async fn run<'a>(&'a self, matches: &'a ArgMatches<'a>) -> TaskResult<TaskOutput> {
        let params = SharedParams::from_matches(matches)?;
        self.run(&params)?;
        Ok(TaskOutput::empty())
    }
}
