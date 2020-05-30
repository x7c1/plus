use std::path::Path;

pub mod copy_as_artifact;

pub fn artifacts_dir() -> &'static Path {
    Path::new("dist")
}

pub fn app_names() -> Vec<&'static str> {
    vec!["s3api"]
}

pub fn executable_names() -> Vec<&'static str> {
    let mut others = vec!["wsb_pilot_tests"];
    let mut names = app_names();
    names.append(&mut others);
    names
}
