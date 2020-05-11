use crate::commands::support::CCFindable;
use crate::tasks::build_apps::Params;
use crate::TaskResult;
use shellwork::core::command;
use shellwork::core::command::{no_op, Prepared, Runner, Unprepared};

pub struct Task;

impl Task {
    pub fn run(&self, params: &Params) -> TaskResult<()> {
        self.prepare(params)?.spawn()?;
        Ok(())
    }

    fn prepare(&self, params: &Params) -> TaskResult<Runner<Prepared>> {
        self.runner(params).prepare(no_op)
    }

    fn runner(&self, params: &Params) -> Runner<Unprepared> {
        // todo: move opt-level to params
        command::program("cargo")
            .arg("build")
            .args(&["--target", params.target.as_triple()])
            .args(&["--workspace", "--exclude=shellwork", "--exclude=wsb-task"])
            .env("RUSTFLAGS", "-C opt-level=0")
            .env_entry(params.cc())
    }
}
