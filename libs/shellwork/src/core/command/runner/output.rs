use std::borrow::Cow;
use std::process::Output;

#[derive(Debug)]
pub struct RunnerOutput {
    output: Output,
}

impl RunnerOutput {
    pub fn new(output: Output) -> Self {
        RunnerOutput { output }
    }
    pub fn stdout(&self) -> Cow<'_, str> {
        String::from_utf8_lossy(&self.output.stdout)
    }
}
