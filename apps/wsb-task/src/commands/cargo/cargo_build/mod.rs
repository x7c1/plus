mod params;
pub use params::Params;

use crate::core::targets::LinuxX86;
use crate::core::targets::{BuildTarget, LinuxArmV7, MacX86};
use crate::error::Error::CommandFailed;
use crate::TaskResult;
use std::process::{Child, Command, Stdio};

pub fn spawn<T: BuildTarget + CanBuild>(params: &Params<T>) -> TaskResult<()> {
    T::spawn(params)
}

pub trait CanBuild: Sized {
    fn spawn(params: &Params<Self>) -> TaskResult<()>
    where
        Self: BuildTarget;
}

impl CanBuild for LinuxX86 {
    fn spawn(params: &Params<Self>) -> TaskResult<()> {
        // rf. [rust - How would you stream output from a Process?](https://stackoverflow.com/questions/31992237/how-would-you-stream-output-from-a-process)
        let child = Command::new("cargo")
            .arg("build")
            .arg("--verbose")
            .args(&["--target", &params.target.as_triple()])
            .env("RUSTFLAGS", "-C opt-level=0")
            .stderr(Stdio::inherit())
            .stdout(Stdio::inherit())
            .spawn()?;

        output(child, params)
    }
}

impl CanBuild for LinuxArmV7 {
    fn spawn(params: &Params<Self>) -> TaskResult<()> {
        // todo: remove duplicates
        let child = Command::new("cargo")
            .arg("build")
            .arg("--verbose")
            .args(&["--target", &params.target.as_triple()])
            .env("RUSTFLAGS", "-C opt-level=0")
            .env("CC", "arm-linux-gnueabihf-gcc")
            .stderr(Stdio::inherit())
            .stdout(Stdio::inherit())
            .spawn()?;

        output(child, params)
    }
}

impl CanBuild for MacX86 {
    fn spawn(params: &Params<Self>) -> TaskResult<()> {
        // todo: check if sdk exists
        // todo: remove duplicates
        let child = Command::new("cargo")
            .arg("build")
            .arg("--verbose")
            .args(&["--target", &params.target.as_triple()])
            .env("RUSTFLAGS", "-C opt-level=0")
            .env("CC", "x86_64-apple-darwin19-clang")
            .stderr(Stdio::inherit())
            .stdout(Stdio::inherit())
            .spawn()?;

        output(child, params)
    }
}

fn output<T>(mut child: Child, params: &Params<T>) -> TaskResult<()>
where
    T: BuildTarget,
{
    let status = child.wait()?;
    if status.success() {
        Ok(())
    } else {
        Err(CommandFailed {
            code: status.code(),
            // todo: remove duplicates
            command: format!("cargo build --target {}", &params.target.as_triple()),
        })
    }
}
