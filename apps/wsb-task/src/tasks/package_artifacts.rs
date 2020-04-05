use crate::commands::package_artifact;
use crate::core::targets::BuildTarget;
use crate::core::Action;
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
        "package-artifacts"
    }

    fn design(&self) -> App {
        SubCommand::with_name(self.name()).about("Compress and archive artifacts.")
    }

    async fn run<'a>(&'a self, matches: &'a ArgMatches<'a>) -> TaskResult<TaskOutput> {
        BuildTarget::all()
            .iter()
            .map(|target| TaskCommands { target, matches })
            .try_for_each(|commands| commands.run())?;

        Ok(TaskOutput::empty())
    }
}

struct TaskCommands<'a> {
    target: &'a BuildTarget,
    #[allow(dead_code)]
    matches: &'a ArgMatches<'a>,
}

impl TaskCommands<'_> {
    fn run(&self) -> TaskResult<()> {
        let params = package_artifact::Params::builder(self.target).build();
        Action::new().spawn(&params)
    }
}
