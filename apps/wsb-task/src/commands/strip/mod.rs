mod params;
pub use params::Params;

use crate::commands::strip;
use crate::commands::support::Definable;
use crate::core::targets::BuildTarget;
use crate::TaskResult;
use shellwork::core::command;
use shellwork::core::command::{Runner, Unprepared};

impl Definable for strip::Params<'_> {
    fn define(&self) -> TaskResult<Runner<Unprepared>> {
        let program = match self.target {
            BuildTarget::LinuxX86 => "strip",
            BuildTarget::LinuxArmV7 => "arm-linux-gnueabihf-strip",
            BuildTarget::MacX86 => "x86_64-apple-darwin19-strip",
        };
        let runner = command::program(program).arg(&self.file_path);
        Ok(runner)
    }
}
