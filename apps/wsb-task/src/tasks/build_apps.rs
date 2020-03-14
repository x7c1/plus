use crate::commands::{cargo_build, Action};
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
        "build-apps"
    }

    fn design(&self) -> App {
        SubCommand::with_name(self.name()).about("Build wasabi applications.")
    }

    async fn run<'a>(&'a self, matches: &'a ArgMatches<'a>) -> TaskResult<TaskOutput> {
        try_foreach_targets!(|target| {
            let action = to_params(target, matches);
            action.spawn(&action.0)
        });
        Ok(TaskOutput::empty())
    }
}

fn to_params<T>(target: T, _matches: &ArgMatches) -> Action<cargo_build::Params<T>>
where
    T: BuildTarget,
{
    let params = cargo_build::Params::builder().target(target).build();
    Action(params)
}
