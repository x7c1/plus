mod body;
mod fixtures;
mod output;
mod status;

use crate::s3api::{Workspace, TEST_WORKSPACE_DIR};
use std::path::PathBuf;

lazy_static! {
    static ref WORKSPACE: PathBuf = PathBuf::new()
        .join(&*TEST_WORKSPACE_DIR)
        .join("s3api")
        .join("put-object");
}

fn workspace() -> Workspace {
    Workspace::new(&*WORKSPACE)
}

pub struct SampleParameters {
    object_key: String,
    upload_src: PathBuf,
    download_dst: PathBuf,
}

impl SampleParameters {
    pub fn format_to_delete(&self) -> String {
        format!("{{ Key={} }}", self.object_key)
    }
}
