use crate::commands::build_pilot::{DefaultFormat, ShowFileName};
use crate::commands::{build_pilot, Action};
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
            let (action, params) = Action::from(target.clone(), matches, to_params);
            action.spawn(&params)?;

            let (action, params) = Action::from(target, matches, to_params_for_file_name);
            let _output = action.capture(&params);
            _output
        });
        Ok(TaskOutput::empty())
    }
}

fn params_base<T, F>(target: T, _matches: &ArgMatches, format: F) -> build_pilot::Params<T, F>
where
    T: BuildTarget,
{
    build_pilot::Params::builder(format).target(target).build()
}

fn to_params<T>(target: T, _matches: &ArgMatches) -> build_pilot::Params<T, DefaultFormat>
where
    T: BuildTarget,
{
    params_base(target, _matches, DefaultFormat)
}

fn to_params_for_file_name<T>(
    target: T,
    _matches: &ArgMatches,
) -> build_pilot::Params<T, ShowFileName>
where
    T: BuildTarget,
{
    params_base(target, _matches, ShowFileName)
}
