mod fixtures;
mod outfile;
mod output;
mod status;

use crate::s3api::Workspace;
use std::path::PathBuf;

lazy_static! {
    static ref WORKSPACE: Workspace = Workspace::new(&["s3api", "get-object"]);
}

fn workspace<'a>() -> &'a Workspace {
    &*WORKSPACE
}

pub struct SampleParameters {
    object_key: String,
    outfile_dst: PathBuf,
}
