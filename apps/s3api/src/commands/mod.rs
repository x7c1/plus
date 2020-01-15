// see also:
// https://docs.aws.amazon.com/cli/latest/reference/s3api/

use crate::CommandResult;
use clap_task::Definition;

pub mod get_object;
pub mod put_object;

pub fn define_all<'a, 'b>() -> Vec<Definition<'a, 'b, CommandResult>> {
    vec![get_object::define(), put_object::define()]
}
