use crate::commands::{build_apps, Action2};
use crate::core::targets::TargetArch;
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
        "build-apps"
    }

    fn design(&self) -> App {
        SubCommand::with_name(self.name()).about("Build wasabi applications.")
    }

    async fn run<'a>(&'a self, matches: &'a ArgMatches<'a>) -> TaskResult<TaskOutput> {
        TargetArch::all().iter().try_for_each(|target| {
            let params = to_params(target, matches)?;
            build_apps(&params)
        })?;
        Ok(TaskOutput::empty())
    }
}

fn to_params<'a>(
    target: &'a TargetArch,
    _matches: &ArgMatches,
) -> TaskResult<build_apps::Params<'a>> {
    let params = build_apps::Params::builder().target(target).build();
    Ok(params)
}

fn build_apps(params: &build_apps::Params) -> TaskResult<()> {
    Action2::new().spawn(params)
}
