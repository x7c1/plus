mod params;
pub use params::Params;

use crate::core::targets::LinuxX86;
use crate::core::targets::{BuildTarget, LinuxArmV7, MacX86};
use crate::error::Error::CommandFailed;
use crate::TaskResult;
use shellwork::core::{command, ExitedProcess};

pub fn spawn<T: BuildTarget + CanBuild>(params: &Params<T>) -> TaskResult<()> {
    T::spawn(params)
}

fn create_sender<T>(params: &Params<T>) -> command::Sender
where
    T: BuildTarget,
{
    // todo: move opt-level to params
    command::Sender::program("cargo")
        .arg("build")
        .arg("--verbose")
        .args(&["--target", &params.target.as_triple()])
        .env("RUSTFLAGS", "-C opt-level=0")
}

pub trait CanBuild: Sized {
    fn spawn(params: &Params<Self>) -> TaskResult<()>
    where
        Self: BuildTarget;
}

impl CanBuild for LinuxX86 {
    fn spawn(params: &Params<Self>) -> TaskResult<()> {
        let sender = create_sender(params);
        let child = sender.spawn()?;
        output(child, params)
    }
}

impl CanBuild for LinuxArmV7 {
    fn spawn(params: &Params<Self>) -> TaskResult<()> {
        let sender = create_sender(params).env("CC", "arm-linux-gnueabihf-gcc");
        let child = sender.spawn()?;
        output(child, params)
    }
}

impl CanBuild for MacX86 {
    fn spawn(params: &Params<Self>) -> TaskResult<()> {
        // todo: check if sdk exists
        let sender = create_sender(params).env("CC", "x86_64-apple-darwin19-clang");
        let child = sender.spawn()?;
        output(child, params)
    }
}

fn output<T>(child: ExitedProcess, params: &Params<T>) -> TaskResult<()>
where
    T: BuildTarget,
{
    if child.success() {
        Ok(())
    } else {
        Err(CommandFailed {
            code: child.status_code(),
            // todo: remove duplicates
            command: format!("cargo build --target {}", &params.target.as_triple()),
        })
    }
}
