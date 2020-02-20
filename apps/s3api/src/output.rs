use crate::serialize::AwsJsonSerialize;
use crate::S3ApiResult;

pub type CommandResult = S3ApiResult<CommandOutput>;

#[derive(Debug)]
pub struct CommandOutput(String);

impl CommandOutput {
    pub fn empty() -> CommandOutput {
        CommandOutput::new("")
    }

    pub fn new<A: Into<String>>(a: A) -> Self {
        CommandOutput(a.into())
    }

    pub fn json<A: AwsJsonSerialize>(a: A) -> S3ApiResult<Self> {
        let json = a.to_aws_json()?;
        Ok(Self::new(json))
    }

    pub fn as_str(&self) -> &str {
        &self.0
    }
}
