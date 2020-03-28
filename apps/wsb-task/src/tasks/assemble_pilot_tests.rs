use crate::commands::build_pilot::OutputKind;
use crate::commands::{build_pilot, copy_as_artifact};
use crate::core::targets::BuildTarget;
use crate::core::{Action, ActionOutput};
use crate::{TaskOutput, TaskResult};
use clap::{App, ArgMatches, SubCommand};
use clap_task::ClapTask;
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

fn build_pilot(target: &BuildTarget, matches: &ArgMatches) -> TaskResult<()> {
    let params = params_to_build_pilot(target, matches, OutputKind::Default);
    Action::new().spawn(&params)
}

fn get_pilot_file_name<'a>(
    target: &'a BuildTarget,
    matches: &'a ArgMatches,
) -> TaskResult<ActionOutput<build_pilot::Params<'a>>> {
    let params = params_to_build_pilot(target, matches, OutputKind::FileName);
    let output = Action::new().capture(&params)?;
    Ok(output)
}

fn params_to_build_pilot<'a>(
    target: &'a BuildTarget,
    _matches: &ArgMatches,
    kind: OutputKind,
) -> build_pilot::Params<'a> {
    build_pilot::Params::builder(kind).target(target).build()
}

fn copy_pilot_file<'a>(
    target: &'a BuildTarget,
    _matches: &'a ArgMatches,
    output: ActionOutput<build_pilot::Params<'a>>,
) -> TaskResult<()> {
    let params = params_to_copy_pilot(target, output);
    Action::new().spawn(&params)
}

fn params_to_copy_pilot<'a>(
    target: &'a BuildTarget,
    output: ActionOutput<build_pilot::Params<'a>>,
) -> copy_as_artifact::Params<'a> {
    copy_as_artifact::Params::builder(target)
        .src(&output.pilot_file_path())
        .dst(Path::new("wsb_pilot_tests"))
        .build()
}
