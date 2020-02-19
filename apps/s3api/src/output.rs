use crate::S3ApiResult;

pub type CommandResult = S3ApiResult<CommandOutput>;

#[derive(Debug)]
pub struct CommandOutput {
    raw: String,
}

impl CommandOutput {
    pub fn empty() -> CommandOutput {
        CommandOutput {
            raw: "".to_string(),
        }
    }
    pub fn new<A: Into<String>>(a: A) -> CommandOutput {
        CommandOutput { raw: a.into() }
    }
    pub fn as_str(&self) -> &str {
        &self.raw
    }
}
