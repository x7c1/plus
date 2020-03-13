use crate::commands::cargo::cargo_build::Params;
use crate::core::targets::{BuildTarget, LinuxArmV7, LinuxX86, MacX86};
use crate::TaskResult;
use shellwork::core::command;
use shellwork::core::command::{Prepared, Runner, Unprepared};

fn base_runner<T>(params: &Params<T>) -> Runner<Unprepared>
where
    T: BuildTarget,
{
    // todo: move opt-level to params
    command::program("cargo")
        .arg("build")
        .arg("--verbose")
        .args(&["--target", &params.target.as_triple()])
        .env("RUSTFLAGS", "-C opt-level=0")
}

#[derive(Debug)]
pub struct UnsupportedReason;

pub trait BuildRunner: BuildTarget + Sized {
    fn define(params: &Params<Self>) -> TaskResult<Runner<Unprepared>>;

    fn unsupported() -> Option<UnsupportedReason> {
        None
    }

    fn prepare(params: &Params<Self>) -> TaskResult<Runner<Prepared>> {
        let prepared = Self::define(params)?.into_prepared();
        Ok(prepared)
    }
}

impl BuildRunner for LinuxX86 {
    fn define(params: &Params<Self>) -> TaskResult<Runner<Unprepared>> {
        let runner = base_runner(params);
        Ok(runner)
    }
}

impl BuildRunner for LinuxArmV7 {
    fn define(params: &Params<Self>) -> TaskResult<Runner<Unprepared>> {
        let runner = base_runner(params).env("CC", "arm-linux-gnueabihf-gcc");
        Ok(runner)
    }
}

impl BuildRunner for MacX86 {
    fn define(params: &Params<Self>) -> TaskResult<Runner<Unprepared>> {
        let runner = base_runner(params).env("CC", "x86_64-apple-darwin19-clang");
        Ok(runner)
    }
    fn unsupported() -> Option<UnsupportedReason> {
        // todo: check if sdk exists
        None
    }
}
