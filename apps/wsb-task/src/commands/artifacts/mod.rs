use std::path::Path;

pub mod compress_artifact;
pub mod copy_as_artifact;

fn artifacts_dir() -> &'static Path {
    Path::new("dist")
}
