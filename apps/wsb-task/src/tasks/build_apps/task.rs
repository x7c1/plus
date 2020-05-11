use crate::commands::support::CCFindable;
use crate::tasks::build_apps::Params;
use crate::TaskResult;
use shellwork::core::command;
use shellwork::core::command::{no_op, Prepared, Runner, Unprepared};

pub struct Task;

impl Task {
    pub fn define(params: &Params) -> TaskResult<Runner<Unprepared>> {
        // todo: move opt-level to params
        let runner = command::program("cargo")
            .arg("build")
            .args(&["--target", params.target.as_triple()])
            .args(&["--workspace", "--exclude=shellwork", "--exclude=wsb-task"])
            .env("RUSTFLAGS", "-C opt-level=0")
            .env_entry(params.cc());

        Ok(runner)
    }

    pub fn prepare(params: &Params) -> TaskResult<Runner<Prepared>> {
        Self::define(params)?.prepare(no_op)
    }
}
