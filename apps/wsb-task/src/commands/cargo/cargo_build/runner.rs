use crate::commands::cargo::cargo_build;
use crate::commands::cargo::cargo_build::Params;
use crate::commands::CommandProvider;
use crate::core::targets::{BuildTarget, LinuxArmV7, LinuxX86, MacX86};
use crate::TaskResult;
use shellwork::core::command;
use shellwork::core::command::{Runner, Unprepared};

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

pub trait ShouldBuild: CommandProvider<Params = Params<Self>> {
    fn spawn(&self, params: &Self::Params) -> TaskResult<()> {
        let runner = self.prepare(params)?;
        let _status = runner.spawn()?;
        Ok(())
    }
}

impl CommandProvider for LinuxX86 {
    type Params = cargo_build::Params<Self>;

    fn define(&self, params: &Self::Params) -> TaskResult<Runner<Unprepared>> {
        let runner = base_runner(params);
        Ok(runner)
    }
}
impl ShouldBuild for LinuxX86 {}

impl CommandProvider for LinuxArmV7 {
    type Params = cargo_build::Params<Self>;

    fn define(&self, params: &Self::Params) -> TaskResult<Runner<Unprepared>> {
        let runner = base_runner(params).env("CC", "arm-linux-gnueabihf-gcc");
        Ok(runner)
    }
}
impl ShouldBuild for LinuxArmV7 {}

#[derive(Debug)]
pub struct UnsupportedReason;

pub trait MayBuild: CommandProvider<Params = Params<Self>> {
    fn spawn(&self, params: &Self::Params) -> TaskResult<()> {
        if let Some(reason) = self.unsupported() {
            eprintln!(
                "unsupported target: {} > {:#?}",
                params.target.as_triple(),
                reason
            );
            return Ok(());
        }
        let runner = self.prepare(params)?;
        let _status = runner.spawn()?;
        Ok(())
    }
    fn unsupported(&self) -> Option<UnsupportedReason>;
}

impl CommandProvider for MacX86 {
    type Params = cargo_build::Params<Self>;
    fn define(&self, params: &Params<Self>) -> TaskResult<Runner<Unprepared>> {
        let runner = base_runner(params).env("CC", "x86_64-apple-darwin19-clang");
        Ok(runner)
    }
}
impl MayBuild for MacX86 {
    fn unsupported(&self) -> Option<UnsupportedReason> {
        // todo: check if sdk exists
        None
    }
}
