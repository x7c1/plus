use crate::commands::build_pilot;
use crate::core::targets::BuildTarget;
use crate::{TaskOutput, TaskResult};
use clap::{App, ArgMatches, SubCommand};
use clap_task::ClapTask;
use shellwork::core::command::{MayRun, ShouldRun};

pub fn define() -> Box<dyn ClapTask<TaskResult<TaskOutput>>> {
    Box::new(Task)
}

struct Task;

#[async_trait]
impl ClapTask<TaskResult<TaskOutput>> for Task {
    fn name(&self) -> &str {
        "build-pilot-tests"
    }

    fn design(&self) -> App {
        SubCommand::with_name(self.name()).about("Build E2E tests.")
    }

    async fn run<'a>(&'a self, matches: &'a ArgMatches<'a>) -> TaskResult<TaskOutput> {
        try_foreach_targets!(|target| {
            let params = to_params(target, matches);
            params.spawn(&params)
        });
        Ok(TaskOutput::empty())
    }
}

fn to_params<T>(target: T, _matches: &ArgMatches) -> build_pilot::Params<T>
where
    T: BuildTarget,
{
    build_pilot::Params::builder().target(target).build()
}
