use crate::error::Error::CommandFailed;
use crate::{TaskOutput, TaskResult};
use std::process::{Command, Stdio};

pub fn spawn(params: Params) -> TaskResult<TaskOutput> {
    // rf. [rust - How would you stream output from a Process?](https://stackoverflow.com/questions/31992237/how-would-you-stream-output-from-a-process)
    let mut child = Command::new("cargo")
        .arg("build")
        .args(&["--target", &params.target])
        .stderr(Stdio::inherit())
        .stdout(Stdio::inherit())
        .spawn()?;

    let status = child.wait()?;
    if status.success() {
        Ok(TaskOutput::empty())
    } else {
        Err(CommandFailed {
            code: status.code(),
            // todo: remove duplicates
            command: format!("cargo build --target {}", &params.target),
        })
    }
}

pub struct Params {
    pub target: String,
}
