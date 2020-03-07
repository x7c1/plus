mod fixture;
mod outfile;
mod output;
mod status;

use crate::s3api::TEST_WORKSPACE_DIR;
use std::path::PathBuf;
use wsb_pilot::cmd::CommandRunner;

lazy_static! {
    static ref WORKSPACE: PathBuf = PathBuf::new()
        .join(&*TEST_WORKSPACE_DIR)
        .join("s3api")
        .join("get-object");
}

fn aws_s3api() -> CommandRunner {
    super::aws_s3api().current_dir(&*WORKSPACE)
}

fn wsb_s3api() -> CommandRunner {
    super::wsb_s3api().current_dir(&*WORKSPACE)
}

fn cat() -> CommandRunner {
    super::cat().current_dir(&*WORKSPACE)
}

pub struct SampleParameters {
    object_key: String,
    outfile_dst: PathBuf,
}
