mod fixtures;
mod outfile;
mod output;
mod status;

use crate::s3api::{Workspace, TEST_WORKSPACE_DIR};
use std::path::PathBuf;

lazy_static! {
    static ref WORKSPACE: PathBuf = PathBuf::new()
        .join(&*TEST_WORKSPACE_DIR)
        .join("s3api")
        .join("get-object");
}

fn workspace() -> Workspace {
    Workspace::new(&*WORKSPACE)
}

pub struct SampleParameters {
    object_key: String,
    outfile_dst: PathBuf,
}
