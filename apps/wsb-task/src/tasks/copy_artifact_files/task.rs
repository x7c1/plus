use crate::commands::{app_names, copy_as_artifact};
use crate::core::targets::{AsBuildTarget, BuildTarget};
use crate::core::Action;
use crate::TaskResult;
use std::path::Path;

pub struct Task;

impl Task {
    pub fn start<P: AsBuildTarget>(&self, params: &P) -> TaskResult<()> {
        let commands = TaskCommands {
            target: *params.as_build_target(),
        };
        commands.run()
    }
}

struct TaskCommands {
    target: BuildTarget,
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
            .src(Path::new("dist.bundle/wsb-pilot-workspace"))
            .dst(Path::new("wsb-pilot-workspace"))
            .build();

        Action::new().spawn(&params)
    }

    fn copy_test_runner(&self) -> TaskResult<()> {
        let params = copy_as_artifact::Params::builder(self.target)
            .src(Path::new("dist.bundle/run_pilot_tests.sh.template"))
            .dst(Path::new("run_pilot_tests.sh"))
            .build();

        Action::new().spawn(&params)
    }

    fn copy_release_apps(&self) -> TaskResult<()> {
        for name in app_names().iter() {
            let params = self.to_app_params(name);
            Action::new().spawn(&params)?;
        }
        Ok(())
    }

    fn to_app_params(&self, name: &str) -> copy_as_artifact::Params {
        copy_as_artifact::Params::builder(self.target)
            .src(
                &Path::new("target")
                    .join(self.target.as_triple())
                    .join("release")
                    .join(name),
            )
            .dst(Path::new(name))
            .build()
    }
}
