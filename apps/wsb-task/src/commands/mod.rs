mod artifacts;
pub use artifacts::copy_as_artifact;
pub use artifacts::package_artifact;
pub use artifacts::{app_names, artifact_size, artifacts_dir, executable_names};

pub mod strip;
pub mod support;
