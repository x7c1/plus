mod params;
pub use params::Params;

use crate::core::targets::LinuxX86;
use crate::core::targets::{BuildTarget, LinuxArmV7, MacX86};
use crate::TaskResult;
use shellwork::core::command;
use shellwork::core::command::{Prepared, Runner, Unprepared};

pub fn spawn<A>(params: &Params<A>) -> TaskResult<()>
where
    A: BuildTarget + CanBuild,
{
    if A::unsupported() {
        return Ok(());
    }
    let _status = A::prepare(params)?.spawn()?;
    Ok(())
}

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

pub trait CanBuild: BuildTarget + Sized {
    fn runner(params: &Params<Self>) -> TaskResult<Runner<Unprepared>>;

    fn unsupported() -> bool {
        false
    }

    fn prepare(params: &Params<Self>) -> TaskResult<Runner<Prepared>> {
        let prepared = Self::runner(params)?.into_prepared();
        Ok(prepared)
    }
}

impl CanBuild for LinuxX86 {
    fn runner(params: &Params<Self>) -> TaskResult<Runner<Unprepared>> {
        let runner = base_runner(params);
        Ok(runner)
    }
}

impl CanBuild for LinuxArmV7 {
    fn runner(params: &Params<Self>) -> TaskResult<Runner<Unprepared>> {
        let runner = base_runner(params).env("CC", "arm-linux-gnueabihf-gcc");
        Ok(runner)
    }
}

impl CanBuild for MacX86 {
    fn runner(params: &Params<Self>) -> TaskResult<Runner<Unprepared>> {
        let runner = base_runner(params).env("CC", "x86_64-apple-darwin19-clang");
        Ok(runner)
    }
    fn unsupported() -> bool {
        // todo: check if sdk exists
        false
    }
}
