use std::path::Path;

pub mod copy_as_artifact;
pub mod package_artifact;

fn artifacts_dir() -> &'static Path {
    Path::new("dist")
}
