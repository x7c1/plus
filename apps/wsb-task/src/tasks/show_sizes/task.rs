use crate::commands::artifacts_dir;
use crate::TaskResult;
use shellwork::core::command;
use shellwork::core::command::{no_op, Prepared, Runner, Unprepared};

pub struct Task;

impl Task {
    pub fn start(&self) -> TaskResult<()> {
        self.prepare()?.spawn()?;
        Ok(())
    }

    fn prepare(&self) -> TaskResult<Runner<Prepared>> {
        self.runner().prepare(no_op)
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
            .arg(artifacts_dir())
    }
}
