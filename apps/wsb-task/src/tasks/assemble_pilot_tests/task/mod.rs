mod build_pilot;
use build_pilot::OutputKind;

use crate::core::targets::{AsBuildTarget, BuildTarget};
use crate::core::ActionOutput;
use crate::tasks::shared::commands::copy_as_artifact;
use crate::TaskResult;
use shellwork::core::command::{no_op, Runner, Unprepared};
use std::path::Path;

pub struct Task;

impl Task {
    pub fn start<P: AsBuildTarget>(&self, params: &P) -> TaskResult<()> {
        let commands = TaskCommands::new(params);
        // todo: ignore unsupported target like macOS
        commands.run()
    }
}

struct TaskCommands {
    runners: CommandRunners,
}

impl TaskCommands {
    pub fn new<P: AsBuildTarget>(params: &P) -> TaskCommands {
        TaskCommands {
            runners: CommandRunners {
                target: *params.as_build_target(),
            },
        }
    }

    pub fn run(&self) -> TaskResult<()> {
        self.build_pilot()?;
        let output = self.get_pilot_file_name()?;
        self.copy_pilot_file(output)
    }

    fn build_pilot(&self) -> TaskResult<()> {
        self.runners
            .to_build_pilot()
            .prepare(no_op::<crate::Error>)?
            .spawn()?;

        Ok(())
    }

    fn get_pilot_file_name(&self) -> TaskResult<ActionOutput<build_pilot::Params>> {
        let output = self
            .runners
            .to_get_pilot_file_name()
            .prepare(no_op::<crate::Error>)?
            .capture()?;

        Ok(ActionOutput::new(output))
    }

    fn copy_pilot_file(&self, output: ActionOutput<build_pilot::Params>) -> TaskResult<()> {
        self.runners
            .to_copy_pilot_file(output)
            .prepare(no_op::<crate::Error>)?
            .spawn()?;

        Ok(())
    }
}

struct CommandRunners {
    target: BuildTarget,
}

impl CommandRunners {
    pub fn to_build_pilot(&self) -> Runner<Unprepared> {
        let params = self.params_to_build_pilot(OutputKind::Default);
        build_pilot::create_runner(&params)
    }

    pub fn to_get_pilot_file_name(&self) -> Runner<Unprepared> {
        let params = self.params_to_build_pilot(OutputKind::FileName);
        build_pilot::create_runner(&params)
    }

    pub fn to_copy_pilot_file(
        &self,
        output: ActionOutput<build_pilot::Params>,
    ) -> Runner<Unprepared> {
        let params = self.params_to_copy_pilot(output);
        copy_as_artifact::create_runner(&params)
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
