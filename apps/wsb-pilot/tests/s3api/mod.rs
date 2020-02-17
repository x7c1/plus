use env_extractor::{FromSingle, InfallibleResult};
use std::process::{Command, Output};

mod put_object;

lazy_static! {
    static ref TEST_BUCKET: String = load_test_bucket().unwrap();
    static ref TEST_APPS_DIR: String = load_apps_dir().unwrap();
    static ref TEST_WORKSPACE_DIR: String = load_workspace_dir().unwrap();
}

fn s3api() -> Command {
    Command::new(format!("{}/s3api", *TEST_APPS_DIR))
}

fn load_test_bucket() -> InfallibleResult<String> {
    FromSingle::new("WSB_TEST_BUCKET").as_required()
}

fn load_apps_dir() -> InfallibleResult<String> {
    FromSingle::new("WSB_APPS_DIR").as_required()
}

fn load_workspace_dir() -> InfallibleResult<String> {
    FromSingle::new("WSB_WORKSPACE_DIR").as_required()
}

fn dump_if_failed(output: &Output) {
    if !output.status.success() {
        dump(output);
    }
}

fn dump(output: &Output) {
    let to_string = |vec| String::from_utf8_lossy(vec).to_string();
    println!("{}", to_string(&output.stdout));

    let e = to_string(&output.stderr);
    if e.len() > 0 {
        println!("stderr: {}", e);
    }
}
