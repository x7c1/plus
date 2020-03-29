use std::borrow::Cow;
use std::process::Output;

#[derive(Debug)]
pub struct RunnerOutput {
    output: Option<Output>,
}

impl RunnerOutput {
    pub fn new(output: Output) -> Self {
        RunnerOutput {
            output: Some(output),
        }
    }
    pub fn empty() -> Self {
        RunnerOutput { output: None }
    }
    pub fn stdout(&self) -> Cow<'_, str> {
        if let Some(output) = &self.output {
            String::from_utf8_lossy(&output.stdout)
        } else {
            Cow::Borrowed("")
        }
    }
}
