use crate::core::support::{confirm_cc, CCRequired, HasBuildMode};
use crate::TaskResult;
use shellwork::core::command;
use shellwork::core::command::{Prepared, Runner, Unprepared};

pub struct Task;

impl Task {
    pub fn start<P>(&self, params: &P) -> TaskResult<()>
    where
        P: CCRequired,
        P: HasBuildMode,
    {
        self.prepare(params)?.spawn()?;
        Ok(())
    }

    fn prepare<P>(&self, params: &P) -> TaskResult<Runner<Prepared>>
    where
        P: CCRequired,
        P: HasBuildMode,
    {
        self.runner(params).prepare(|_| confirm_cc(params))
    }

    fn runner<P>(&self, params: &P) -> Runner<Unprepared>
    where
        P: CCRequired,
        P: HasBuildMode,
    {
        command::program("cargo")
            .arg("build")
            .push_arg(params.build_mode())
            .args(&["--target", params.as_build_target().as_triple()])
            .args(&["--workspace", "--exclude=shellwork", "--exclude=plus-task"])
            .env("RUSTFLAGS", params.opt_level())
            .env_entry(params.cc())
    }
}
