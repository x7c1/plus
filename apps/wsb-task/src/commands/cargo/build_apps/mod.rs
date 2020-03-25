mod params;
pub use params::Params;

use crate::commands::build_apps;
use crate::commands::support::{Definable2, InsertCC};
use crate::TaskResult;
use shellwork::core::command;
use shellwork::core::command::{Runner, Unprepared};

impl Definable2 for build_apps::Params<'_> {
    fn define(&self) -> TaskResult<Runner<Unprepared>> {
        // todo: move opt-level to params
        let runner = command::program("cargo")
            .arg("build")
            .args(&["--target", self.target.as_triple()])
            .args(&["--workspace", "--exclude=shellwork", "--exclude=wsb-task"])
            .env("RUSTFLAGS", "-C opt-level=0")
            .insert_cc(&self.target);

        Ok(runner)
    }
}
