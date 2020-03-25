use crate::commands::build_pilot::OutputKind;
use crate::commands::{build_pilot, copy_as_artifact, Action};
use crate::core::targets::BuildTarget;
use crate::{TaskOutput, TaskResult};
use clap::{App, ArgMatches, SubCommand};
use clap_task::ClapTask;
use shellwork::core::command::{Runnable, RunnerOutput};
use std::path::Path;

pub fn define() -> Box<dyn ClapTask<TaskResult<TaskOutput>>> {
    Box::new(Task)
}

struct Task;

#[async_trait]
impl ClapTask<TaskResult<TaskOutput>> for Task {
    fn name(&self) -> &str {
        "assemble-pilot-tests"
    }

    fn design(&self) -> App {
        SubCommand::with_name(self.name()).about("Build E2E tests.")
    }

    async fn run<'a>(&'a self, matches: &'a ArgMatches<'a>) -> TaskResult<TaskOutput> {
        try_foreach_targets!(|target| {
            // build and show compile error if exists.
            let params = params_to_build_pilot(target.clone(), matches, OutputKind::Default);
            let action = Action::create(&target, &params);
            action.spawn(&params)?;

            // build and get filename of pilot-test.
            let params = params_to_build_pilot(target.clone(), matches, OutputKind::FileName);
            let action = Action::create(&target, &params);
            let output = action.capture(&params)?;

            // copy pilot-test file to artifact directory.
            let params = params_to_copy_pilot(target.clone(), output);
            let action = Action::create(&target, &params);
            action.spawn(&params)?;

            TaskResult::Ok(())
        });
        Ok(TaskOutput::empty())
    }
}

fn params_to_build_pilot<T>(
    target: T,
    _matches: &ArgMatches,
    kind: OutputKind,
) -> build_pilot::Params<T>
where
    T: BuildTarget,
{
    build_pilot::Params::builder(kind).target(target).build()
}

fn params_to_copy_pilot<T>(target: T, output: Option<RunnerOutput>) -> copy_as_artifact::Params<T>
where
    T: BuildTarget,
{
    let output = output.expect("output required");
    let src = output.stdout();

    copy_as_artifact::Params::builder(target)
        .src(Path::new(src.as_ref()))
        .dst(Path::new("wsb_pilot_tests"))
        .build()
}
