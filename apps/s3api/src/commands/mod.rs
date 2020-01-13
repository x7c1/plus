// see also:
// https://docs.aws.amazon.com/cli/latest/reference/s3api/

use crate::S3ApiResult;
use clap_task::Definition;

pub mod get_object;
pub mod put_object;

pub type CommandResult = S3ApiResult<()>;

pub fn create_all<'a, 'b>() -> Vec<Definition<'a, 'b, CommandResult>> {
    vec![get_object::create(), put_object::create()]
}
