use crate::S3ApiResult;

pub type CommandResult = S3ApiResult<ResponseSummary>;

#[derive(Debug)]
pub struct ResponseSummary {}

impl ResponseSummary {
    pub fn empty() -> ResponseSummary {
        ResponseSummary {}
    }
}
