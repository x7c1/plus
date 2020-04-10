use crate::commands::{artifacts_dir, executable_names, strip};
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
        "strip-executables"
    }

    fn design(&self) -> App {
        SubCommand::with_name(self.name()).about("Strip executable artifacts.")
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
        for name in executable_names().iter() {
            let path = artifacts_dir().join(self.target.as_abbr()).join(name);
            let params = strip::Params::builder(self.target).file_path(path).build();
            Action::new().spawn(&params)?;
        }
        Ok(())
    }
}
