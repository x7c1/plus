use env_extractor::{FromStrResult, SingleValue};

mod parameters_pair;
pub use parameters_pair::ParametersPair;

mod workspace;
pub use workspace::Workspace;

mod get_object;
mod put_object;

lazy_static! {
    static ref TEST_BUCKET: String = load_test_bucket().unwrap();
    static ref TEST_APPS_DIR: String = load_apps_dir().unwrap();
    static ref TEST_WORKSPACE_DIR: String = load_workspace_dir().unwrap();
}

fn load_test_bucket() -> FromStrResult<String> {
    SingleValue::new("WSB_TEST_BUCKET").as_required()
}

fn load_apps_dir() -> FromStrResult<String> {
    SingleValue::new("WSB_APPS_DIR").as_required()
}

fn load_workspace_dir() -> FromStrResult<String> {
    SingleValue::new("PLUS_WORKSPACE_DIR").as_required()
}
