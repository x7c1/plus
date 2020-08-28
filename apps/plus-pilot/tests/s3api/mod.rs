use env_extractor::{EnvVar, RequiredResult};

mod parameters_pair;
pub use parameters_pair::ParametersPair;

mod workspace;
use std::str::FromStr;
pub use workspace::Workspace;

mod get_object;
mod put_object;

lazy_static! {
    static ref TEST_BUCKET: String = load_test_bucket().unwrap();
    static ref TEST_APPS_DIR: String = load_apps_dir().unwrap();
    static ref TEST_WORKSPACE_DIR: String = load_workspace_dir().unwrap();
}

type EnvResult = RequiredResult<String>;

fn load_test_bucket() -> EnvResult {
    EnvVar::new("PLUS_TEST_BUCKET").as_required()
}

fn load_apps_dir() -> EnvResult {
    EnvVar::new("PLUS_APPS_DIR").as_required()
}

fn load_workspace_dir() -> EnvResult {
    EnvVar::new("PLUS_WORKSPACE_DIR").as_required()
}
