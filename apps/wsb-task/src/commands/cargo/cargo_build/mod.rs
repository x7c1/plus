mod params;
pub use params::Params;

use crate::core::targets::LinuxX86;
use crate::core::targets::{BuildTarget, LinuxArmV7, MacX86};
use crate::error::Error::CommandFailed;
use crate::TaskResult;
use shellwork::core::command;
use shellwork::core::command::Runner;

pub fn spawn<T: BuildTarget + CanBuild>(params: &Params<T>) -> TaskResult<()> {
    T::spawn(params)
}

fn base_runner<T>(params: &Params<T>) -> command::Runner
where
    T: BuildTarget,
{
    // todo: move opt-level to params
    command::Runner::program("cargo")
        .arg("build")
        .arg("--verbose")
        .args(&["--target", &params.target.as_triple()])
        .env("RUSTFLAGS", "-C opt-level=0")
}

pub trait CanBuild: Sized {
    fn runner(params: &Params<Self>) -> TaskResult<Runner>
    where
        Self: BuildTarget;

    fn spawn(params: &Params<Self>) -> TaskResult<()>
    where
        Self: BuildTarget,
    {
        let runner = Self::runner(params)?;
        let child = runner.spawn()?;
        if child.success() {
            Ok(())
        } else {
            Err(CommandFailed {
                code: child.status_code(),
                summary: runner.create_summary(),
            })
        }
    }
}

impl CanBuild for LinuxX86 {
    fn runner(params: &Params<Self>) -> TaskResult<Runner> {
        let runner = base_runner(params);
        Ok(runner)
    }
}

impl CanBuild for LinuxArmV7 {
    fn runner(params: &Params<Self>) -> TaskResult<Runner> {
        let runner = base_runner(params).env("CC", "arm-linux-gnueabihf-gcc");
        Ok(runner)
    }
}

impl CanBuild for MacX86 {
    fn runner(params: &Params<Self>) -> TaskResult<Runner> {
        // todo: check if sdk exists
        let runner = base_runner(params).env("CC", "x86_64-apple-darwin19-clang");
        Ok(runner)
    }
}
