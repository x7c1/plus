mod body;
mod fixtures;
mod output;
mod status;

use crate::s3api::Workspace;
use std::path::PathBuf;

lazy_static! {
    static ref WORKSPACE: Workspace = Workspace::new(&["s3api", "put-object"]);
}

fn workspace<'a>() -> &'a Workspace {
    &*WORKSPACE
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
