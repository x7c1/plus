mod params;
pub use params::Params;

use crate::core::targets::LinuxX86;
use crate::core::targets::{BuildTarget, LinuxArmV7, MacX86};
use crate::error::Error::CommandFailed;
use crate::TaskResult;
use shellwork::core::command;
use shellwork::core::command::Sender;

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
    fn sender(params: &Params<Self>) -> TaskResult<Sender>
    where
        Self: BuildTarget;

    fn spawn(params: &Params<Self>) -> TaskResult<()>
    where
        Self: BuildTarget,
    {
        let sender = Self::sender(params)?;
        let child = sender.spawn()?;
        if child.success() {
            Ok(())
        } else {
            Err(CommandFailed {
                code: child.status_code(),
                summary: sender.create_summary(),
            })
        }
    }
}

impl CanBuild for LinuxX86 {
    fn sender(params: &Params<Self>) -> TaskResult<Sender> {
        let sender = create_sender(params);
        Ok(sender)
    }
}

impl CanBuild for LinuxArmV7 {
    fn sender(params: &Params<Self>) -> TaskResult<Sender> {
        let sender = create_sender(params).env("CC", "arm-linux-gnueabihf-gcc");
        Ok(sender)
    }
}

impl CanBuild for MacX86 {
    fn sender(params: &Params<Self>) -> TaskResult<Sender> {
        // todo: check if sdk exists
        let sender = create_sender(params).env("CC", "x86_64-apple-darwin19-clang");
        Ok(sender)
    }
}
