mod params;
use params::Params;

use crate::core::build_mode::AsBuildMode;
use crate::core::env::{app_names, executable_names};
use crate::core::support::program_exists;
use crate::core::targets::AsBuildTarget;
use crate::TaskResult;
use shellwork::core::command;
use shellwork::core::command::{Prepared, Runner, Unprepared};

pub struct Task;

impl Task {
    pub fn start<P>(&self, params: &P) -> TaskResult<()>
    where
        P: AsBuildTarget,
        P: AsBuildMode,
    {
        // app_names().iter().map(|name| {})
        // self.prepare(params)?.spawn()?;
        // todo:
        unimplemented!();
        Ok(())
    }
    /*
        fn prepare<P: AsBuildTarget>(&self, params: &P) -> TaskResult<Runner<Prepared>> {
            let params = Params::builder(params.as_build_target()).build();
            self.runner(&params).prepare(program_exists)
        }

        fn runner(&self, params: &Params) -> Runner<Unprepared> {
            command::program("tar")
                .arg("--xz")
                .args(&["--create", "--file", &params.dst_path])
                // .args(&["--directory", &params.directory_path])
                .arg(params.target.as_abbr())
        }
    */
}
