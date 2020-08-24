use crate::core::env::artifacts_dir;
use crate::core::support::program_exists;
use crate::TaskResult;
use serde::Deserialize;
use shellwork::core::command;
use shellwork::core::command::{Runner, Unprepared};
use std::str::FromStr;

pub struct Task;

impl Task {
    pub fn start(&self, params: &Params) -> TaskResult<()> {
        println!("params...{:#?}", params);
        let runner = self.runner().prepare(program_exists)?;
        runner.spawn()?;
        Ok(())
    }

    fn runner(&self) -> Runner<Unprepared> {
        command::program("tree")
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
            // list directories before files
            .arg("--dirsfirst")
            .arg(artifacts_dir())
    }
}

#[derive(Debug)]
pub struct Params {
    pub files: ChangedFiles,
}

#[derive(Debug, Deserialize)]
pub struct ChangedFiles {
    pub paths: Vec<String>,
}

impl FromStr for ChangedFiles {
    type Err = crate::Error;

    fn from_str(x: &str) -> Result<Self, Self::Err> {
        println!("changed files - passed args: {}", x);
        let files = ChangedFiles { paths: serde_json::from_str(x)? };
        Ok(files)
    }
}
