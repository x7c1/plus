use crate::commands::package_artifact;
use crate::core::targets::{AsBuildTarget, BuildTarget};
use crate::core::Action;
use crate::TaskResult;

pub struct Task;

impl Task {
    pub fn start<P: AsBuildTarget>(&self, params: &P) -> TaskResult<()> {
        let commands = TaskCommands {
            target: &params.as_build_target(),
        };
        commands.run()
    }
}

struct TaskCommands<'a> {
    target: &'a BuildTarget,
}

impl TaskCommands<'_> {
    fn run(&self) -> TaskResult<()> {
        let params = package_artifact::Params::builder(self.target).build();
        Action::new().spawn(&params)
    }
}
