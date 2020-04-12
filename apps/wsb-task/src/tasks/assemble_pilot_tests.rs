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
        SubCommand::with_name(self.name()).about("Build and copy E2E tests.")
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
        self.build_pilot()?;
        let output = self.get_pilot_file_name()?;
        self.copy_pilot_file(output)
    }

    fn build_pilot(&self) -> TaskResult<()> {
        let params = self.params_to_build_pilot(OutputKind::Default);
        Action::new().spawn(&params)
    }

    fn get_pilot_file_name(&self) -> TaskResult<ActionOutput<build_pilot::Params>> {
        let params = self.params_to_build_pilot(OutputKind::FileName);
        let output = Action::new().capture(&params)?;
        Ok(output)
    }

    fn copy_pilot_file(&self, output: ActionOutput<build_pilot::Params>) -> TaskResult<()> {
        let params = self.params_to_copy_pilot(output);
        Action::new().spawn(&params)
    }

    fn params_to_build_pilot(&self, kind: OutputKind) -> build_pilot::Params {
        build_pilot::Params::builder(kind)
            .target(self.target)
            .build()
    }

    fn params_to_copy_pilot(
        &self,
        output: ActionOutput<build_pilot::Params>,
    ) -> copy_as_artifact::Params {
        copy_as_artifact::Params::builder(self.target)
            .src(&output.pilot_file_path())
            .dst(Path::new("wsb_pilot_tests"))
            .build()
    }
}
