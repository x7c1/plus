use crate::core::build_mode::BuildMode;
use crate::core::env::app_names;
use crate::core::support::HasBuildMode;
use crate::core::targets::{AsBuildTarget, BuildTarget};
use crate::tasks::shared::commands::copy_as_artifact;
use crate::TaskResult;
use shellwork::core::command::no_op;
use std::path::Path;

pub struct Task;

impl Task {
    pub fn start<P>(&self, params: &P) -> TaskResult<()>
    where
        P: AsBuildTarget,
        P: HasBuildMode,
    {
        let commands = TaskCommands {
            target: *params.as_build_target(),
            build_mode: *params.as_build_mode(),
        };
        commands.run()
    }
}

struct TaskCommands {
    target: BuildTarget,
    build_mode: BuildMode,
}

impl TaskCommands {
    fn run(&self) -> TaskResult<()> {
        self.copy_workspace()?;
        self.copy_test_runner()?;
        self.copy_release_apps()?;
        Ok(())
    }

    fn copy_workspace(&self) -> TaskResult<()> {
        let params = copy_as_artifact::Params::builder(self.target)
            .src(Path::new("dist.bundle/plus_pilot_workspace"))
            .dst(Path::new("plus_pilot_workspace"))
            .build();

        self.spawn(&params)
    }

    fn copy_test_runner(&self) -> TaskResult<()> {
        let params = copy_as_artifact::Params::builder(self.target)
            .src(Path::new("dist.bundle/run_pilot_tests.sh.template"))
            .dst(Path::new("run_pilot_tests.sh"))
            .build();

        self.spawn(&params)
    }

    fn copy_release_apps(&self) -> TaskResult<()> {
        for name in app_names().iter() {
            let params = self.to_app_params(name);
            self.spawn(&params)?;
        }
        Ok(())
    }

    fn to_app_params(&self, name: &str) -> copy_as_artifact::Params {
        copy_as_artifact::Params::builder(self.target)
            .src(
                &Path::new("target")
                    .join(self.target.as_triple())
                    .join(self.build_mode.as_str())
                    .join(name),
            )
            .dst(Path::new(name))
            .build()
    }

    fn spawn(&self, params: &copy_as_artifact::Params) -> TaskResult<()> {
        let runner = copy_as_artifact::create_runner(params);
        runner.prepare(no_op::<crate::Error>)?.spawn()?;
        Ok(())
    }
}
