use crate::commands::support::CCFindable;
use crate::TaskResult;
use shellwork::core::command;
use shellwork::core::command::{no_op, Prepared, Runner, Unprepared};

pub struct Task;

impl Task {
    pub fn start<P: CCFindable>(&self, params: &P) -> TaskResult<()> {
        // todo: ignore unsupported target like macOS
        self.prepare(params)?.spawn()?;
        Ok(())
    }

    fn prepare<P: CCFindable>(&self, params: &P) -> TaskResult<Runner<Prepared>> {
        self.runner(params).prepare(no_op)
    }

    fn runner<P: CCFindable>(&self, params: &P) -> Runner<Unprepared> {
        // todo: move opt-level to params
        command::program("cargo")
            .arg("build")
            .args(&["--target", params.as_build_target().as_triple()])
            .args(&["--workspace", "--exclude=shellwork", "--exclude=wsb-task"])
            .env("RUSTFLAGS", "-C opt-level=0")
            .env_entry(params.cc())
    }
}
