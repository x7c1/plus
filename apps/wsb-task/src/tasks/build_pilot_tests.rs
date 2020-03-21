use crate::commands::build_pilot::OutputKind;
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
            println!("output: {:?}", _output);
            _output
        });
        Ok(TaskOutput::empty())
    }
}

fn new_params<T>(target: T, _matches: &ArgMatches, format: OutputKind) -> build_pilot::Params<T>
where
    T: BuildTarget,
{
    build_pilot::Params::builder(format).target(target).build()
}

fn to_params<T>(target: T, matches: &ArgMatches) -> build_pilot::Params<T>
where
    T: BuildTarget,
{
    new_params(target, matches, OutputKind::Default)
}

fn to_params_for_file_name<T>(target: T, matches: &ArgMatches) -> build_pilot::Params<T>
where
    T: BuildTarget,
{
    new_params(target, matches, OutputKind::FileName)
}
