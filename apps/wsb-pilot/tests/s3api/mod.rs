use env_extractor::{FromStrResult, SingleValue};
use wsb_pilot::cmd::CommandRunner;

mod get_object;
mod put_object;

lazy_static! {
    static ref TEST_BUCKET: String = load_test_bucket().unwrap();
    static ref TEST_APPS_DIR: String = load_apps_dir().unwrap();
    static ref TEST_WORKSPACE_DIR: String = load_workspace_dir().unwrap();
}

fn aws_s3api() -> CommandRunner {
    CommandRunner::new("aws").arg("s3api")
}

fn wsb_s3api() -> CommandRunner {
    CommandRunner::new(format!("{}/s3api", *TEST_APPS_DIR))
}

fn load_test_bucket() -> FromStrResult<String> {
    SingleValue::new("WSB_TEST_BUCKET").as_required()
}

fn load_apps_dir() -> FromStrResult<String> {
    SingleValue::new("WSB_APPS_DIR").as_required()
}

fn load_workspace_dir() -> FromStrResult<String> {
    SingleValue::new("WSB_WORKSPACE_DIR").as_required()
}
