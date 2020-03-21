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
            let to_params = to_params_for(OutputKind::Default);
            let (action, params) = Action::from(target.clone(), matches, to_params);
            action.spawn(&params)?;

            let to_params = to_params_for(OutputKind::FileName);
            let (action, params) = Action::from(target, matches, to_params);
            let _output = action.capture(&params);
            println!("output: {:?}", _output);
            _output
        });
        Ok(TaskOutput::empty())
    }
}

fn to_params_for<T>(kind: OutputKind) -> impl FnOnce(T, &ArgMatches) -> build_pilot::Params<T>
where
    T: BuildTarget,
{
    |target, matches| create_params(target, matches, kind)
}

fn create_params<T>(target: T, _matches: &ArgMatches, kind: OutputKind) -> build_pilot::Params<T>
where
    T: BuildTarget,
{
    build_pilot::Params::builder(kind).target(target).build()
}
