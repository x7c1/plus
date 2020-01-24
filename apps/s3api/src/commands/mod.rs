// see also:
// https://docs.aws.amazon.com/cli/latest/reference/s3api/

use crate::CommandResult;
use clap_task::ClapTask;

pub mod get_object;
pub mod put_object;

pub fn define_all() -> Vec<Box<dyn ClapTask<CommandResult>>> {
    vec![get_object::define(), put_object::define()]
}
