mod params;
pub use params::Params;

use crate::commands::compress_artifact;
use crate::commands::support::Definable;
use crate::TaskResult;
use shellwork::core::command;
use shellwork::core::command::{Runner, Unprepared};

impl Definable for compress_artifact::Params<'_> {
    fn define(&self) -> TaskResult<Runner<Unprepared>> {
        let runner = command::program("tar")
            .arg("--xz")
            .args(&["--create", "--file", &self.file_path])
            .args(&["--directory", &self.directory_path])
            .arg(self.target.as_abbr());

        Ok(runner)
    }
}
