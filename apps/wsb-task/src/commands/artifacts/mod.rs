use std::path::Path;

pub mod copy_as_artifact;
pub mod package_artifact;

fn artifacts_dir() -> &'static Path {
    Path::new("dist")
}

pub fn app_names() -> Vec<&'static str> {
    vec!["s3api"]
}

pub fn artifact_names() -> Vec<&'static str> {
    let mut others = vec!["wsb_pilot_tests"];
    let mut names = app_names();
    names.append(&mut others);
    names
}
