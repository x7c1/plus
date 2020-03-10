use crate::commands::cargo_build;
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

    async fn run<'a>(&'a self, _matches: &'a ArgMatches<'a>) -> TaskResult<TaskOutput> {
        cargo_build::spawn(cargo_build::Params {
            target: "x86_64-unknown-linux-musl".to_string(),
        })
    }
}
