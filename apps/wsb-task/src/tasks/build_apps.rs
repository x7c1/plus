use crate::commands::cargo_build;
use crate::commands::cargo_build::CanBuild;
use crate::core::targets::{BuildTarget, LinuxArmV7, LinuxX86, MacX86};
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
        SubCommand::with_name(self.name()).about("build wasabi applications.")
    }

    async fn run<'a>(&'a self, matches: &'a ArgMatches<'a>) -> TaskResult<TaskOutput> {
        build(LinuxX86, matches)?;
        build(LinuxArmV7, matches)?;
        build(MacX86, matches)?;
        Ok(TaskOutput::empty())
    }
}

fn build<T: BuildTarget + CanBuild>(target: T, _matches: &ArgMatches) -> TaskResult<()> {
    let params = cargo_build::Params::builder().target(target).build();
    cargo_build::spawn(&params)
}
