mod output;
mod status;

use crate::s3api::TEST_WORKSPACE_DIR;
use std::path::{Path, PathBuf};
use std::{fs, io};
use wsb_pilot::cmd::CommandRunner;

lazy_static! {
    static ref WORKSPACE: PathBuf = PathBuf::new()
        .join(&*TEST_WORKSPACE_DIR)
        .join("s3api")
        .join("put-object");
}

fn aws_s3api() -> CommandRunner {
    super::aws_s3api().current_dir(&*WORKSPACE)
}

fn wsb_s3api() -> CommandRunner {
    super::wsb_s3api().current_dir(&*WORKSPACE)
}

fn cat(path: &Path) -> io::Result<String> {
    let full_path = WORKSPACE.join(path);
    fs::read_to_string(full_path)
}

struct Sample {
    object_key: String,
    upload_src: PathBuf,
    download_dst: PathBuf,
}
