#[derive(Debug)]
pub struct TaskOutput(String);

impl TaskOutput {
    pub fn empty() -> TaskOutput {
        TaskOutput::new("")
    }

    pub fn new<A: Into<String>>(a: A) -> Self {
        TaskOutput(a.into())
    }

    pub fn as_str(&self) -> &str {
        &self.0
    }
}
