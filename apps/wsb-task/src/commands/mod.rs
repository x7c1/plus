mod artifacts;
pub use artifacts::copy_as_artifact;
pub use artifacts::package_artifact;
pub use artifacts::{app_names, artifact_names};

mod cargo;
pub use cargo::build_apps;
pub use cargo::build_pilot;

pub mod strip;
pub mod support;
