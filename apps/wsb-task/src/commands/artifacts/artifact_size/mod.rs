mod params;
pub use params::Params;

use crate::commands::support::Definable;
use crate::commands::{artifact_size, artifacts_dir};
use crate::TaskResult;
use shellwork::core::command;
use shellwork::core::command::{Runner, Unprepared};

impl Definable for artifact_size::Params {
    fn define(&self) -> TaskResult<Runner<Unprepared>> {
        let runner = command::program("tree")
            // specify max tree depth to descend
            .args(&["-L", "2"])
            // use ANSI line graphics hack when printing indentation lines
            .arg("-A")
            // sort output by change time
            .arg("-c")
            // print directory sizes
            .arg("--du")
            // print human readable file size in SI units (powers of 1000)
            .arg("--si")
            .arg(artifacts_dir());

        Ok(runner)
    }
}
