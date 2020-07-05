mod params;
use params::Params;

use crate::core::support::confirm_program;
use crate::core::targets::AsBuildTarget;
use crate::TaskResult;
use shellwork::core::command;
use shellwork::core::command::{Prepared, Runner, Unprepared};

pub struct Task;

impl Task {
    pub fn start<P: AsBuildTarget>(&self, params: &P) -> TaskResult<()> {
        self.prepare(params)?.spawn()?;
        Ok(())
    }

    fn prepare<P: AsBuildTarget>(&self, params: &P) -> TaskResult<Runner<Prepared>> {
        let params = Params::builder(params.as_build_target()).build();
        self.runner(&params).prepare(confirm_program)
    }

    fn runner(&self, params: &Params) -> Runner<Unprepared> {
        command::program("tar")
            .arg("--xz")
            .args(&["--create", "--file", &params.file_path])
            .args(&["--directory", &params.directory_path])
            .arg(params.target.as_abbr())
    }
}
