mod params;
use params::Params;

use crate::commands::support::Definable;
use crate::core::targets::{AsBuildTarget, BuildTarget};
use crate::core::Action;
use crate::TaskResult;
use shellwork::core::command;
use shellwork::core::command::{Runner, Unprepared};

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
        let params = Params::builder(self.target).build();
        Action::new().spawn(&params)
    }
}

impl Definable for Params<'_> {
    fn define(&self) -> TaskResult<Runner<Unprepared>> {
        let runner = command::program("tar")
            .arg("--xz")
            .args(&["--create", "--file", &self.file_path])
            .args(&["--directory", &self.directory_path])
            .arg(self.target.as_abbr());

        Ok(runner)
    }
}
