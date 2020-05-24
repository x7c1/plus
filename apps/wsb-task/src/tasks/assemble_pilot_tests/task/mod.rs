mod build_pilot;
use build_pilot::OutputKind;

use crate::commands::copy_as_artifact;
use crate::core::targets::{AsBuildTarget, BuildTarget};
use crate::core::ActionOutput;
use crate::TaskResult;
use shellwork::core::command::no_op;
use std::path::Path;

pub struct Task;

impl Task {
    pub fn start<P: AsBuildTarget>(&self, params: &P) -> TaskResult<()> {
        let commands = TaskCommands {
            target: *params.as_build_target(),
        };
        // todo: ignore unsupported target like macOS
        commands.run()
    }
}

struct TaskCommands {
    target: BuildTarget,
}

impl TaskCommands {
    fn run(&self) -> TaskResult<()> {
        self.build_pilot()?;
        let output = self.get_pilot_file_name()?;
        self.copy_pilot_file(output)
    }

    fn build_pilot(&self) -> TaskResult<()> {
        let params = self.params_to_build_pilot(OutputKind::Default);
        let runner = build_pilot::create_runner(&params);
        runner.prepare(no_op::<crate::Error>)?.spawn()?;
        Ok(())
    }

    fn get_pilot_file_name(&self) -> TaskResult<ActionOutput<build_pilot::Params>> {
        let params = self.params_to_build_pilot(OutputKind::FileName);
        let runner = build_pilot::create_runner(&params);
        let output = runner.prepare(no_op::<crate::Error>)?.capture()?;
        Ok(ActionOutput::new(output))
    }

    fn copy_pilot_file(&self, output: ActionOutput<build_pilot::Params>) -> TaskResult<()> {
        let params = self.params_to_copy_pilot(output);
        let runner = copy_as_artifact::create_runner(&params);
        runner.prepare(no_op::<crate::Error>)?.spawn()?;
        Ok(())
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
