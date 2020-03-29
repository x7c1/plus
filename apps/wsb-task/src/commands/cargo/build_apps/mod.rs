mod params;
pub use params::Params;

use crate::commands::build_apps;
use crate::commands::support::{CCFindable, Definable};
use crate::TaskResult;
use shellwork::core::command;
use shellwork::core::command::{Runner, Unprepared};

impl Definable for build_apps::Params<'_> {
    fn define(&self) -> TaskResult<Runner<Unprepared>> {
        // todo: move opt-level to params
        let runner = command::program("cargo")
            .arg("build")
            .args(&["--target", self.target.as_triple()])
            .args(&["--workspace", "--exclude=shellwork", "--exclude=wsb-task"])
            .env("RUSTFLAGS", "-C opt-level=0")
            .env_entry(self.cc());

        Ok(runner)
    }
}
