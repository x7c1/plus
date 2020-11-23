use crate::core::build_mode::BuildMode;
use crate::core::support::release::{ChangedFiles, PackageName};
use crate::core::targets::BuildTarget;

#[derive(Debug)]
pub struct Params {
    pub files: ChangedFiles,
    pub target_packages: Vec<PackageName>,
    pub target: BuildTarget,
    pub build_mode: BuildMode,
}
