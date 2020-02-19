use crate::S3ApiResult;

pub type CommandResult = S3ApiResult<CommandOutput>;

#[derive(Debug)]
pub struct CommandOutput {}

impl CommandOutput {
    pub fn empty() -> CommandOutput {
        CommandOutput {}
    }
}
