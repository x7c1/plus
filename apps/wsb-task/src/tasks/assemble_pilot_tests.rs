use crate::commands::build_pilot::OutputKind;
use crate::commands::{build_pilot, copy_as_artifact, Action};
use crate::core::targets::BuildTarget;
use crate::{TaskOutput, TaskResult};
use clap::{App, ArgMatches, SubCommand};
use clap_task::ClapTask;
use shellwork::core::command::RunnerOutput;
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
        BuildTarget::all().iter().try_for_each(|target| {
            build_pilot(target, matches)?;
            let output = get_pilot_file_name(target, matches)?;
            copy_pilot_file(target, matches, output)?;
            TaskResult::Ok(())
        })?;
        Ok(TaskOutput::empty())
    }
}

fn build_pilot(target: &BuildTarget, _matches: &ArgMatches) -> TaskResult<()> {
    let params = params_to_build_pilot(target, _matches, OutputKind::Default);
    Action::new().spawn(&params)
}

fn get_pilot_file_name(target: &BuildTarget, _matches: &ArgMatches) -> TaskResult<RunnerOutput> {
    let params = params_to_build_pilot(target, _matches, OutputKind::FileName);
    let maybe = Action::new().capture(&params)?;
    // todo: avoid unwrap
    Ok(maybe.unwrap())
}

fn params_to_build_pilot<'a>(
    target: &'a BuildTarget,
    _matches: &ArgMatches,
    kind: OutputKind,
) -> build_pilot::Params<'a> {
    build_pilot::Params::builder(kind).target(target).build()
}

fn copy_pilot_file(
    target: &BuildTarget,
    _matches: &ArgMatches,
    output: RunnerOutput,
) -> TaskResult<()> {
    let params = params_to_copy_pilot(target, output);
    Action::new().spawn(&params)
}

fn params_to_copy_pilot(target: &BuildTarget, output: RunnerOutput) -> copy_as_artifact::Params {
    let src = output.stdout();
    copy_as_artifact::Params::builder(target)
        .src(Path::new(src.as_ref()))
        .dst(Path::new("wsb_pilot_tests"))
        .build()
}
