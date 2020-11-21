use crate::core::build_mode::{AsBuildMode, BuildMode};
use crate::core::env::artifacts_dir;
use crate::core::support::release::CargoTomlPackage;
use crate::core::targets::BuildTarget;
use std::path::PathBuf;

pub trait HasBuildMode: AsBuildMode {
    fn opt_level(&self) -> String {
        // rf. [What do the optimization levels `-Os` and `-Oz` do in rustc? - Stack Overflow](https://stackoverflow.com/questions/45608392/what-do-the-optimization-levels-os-and-oz-do-in-rustc)
        let level = match self.as_build_mode() {
            BuildMode::Debug => "0",
            BuildMode::Release => "z",
        };
        format!("-C opt-level={}", level)
    }
    fn build_mode(&self) -> Option<&str> {
        match self.as_build_mode() {
            BuildMode::Debug => None,
            BuildMode::Release => Some("--release"),
        }
    }
}

pub fn get_artifacts_dir(target: BuildTarget, mode: BuildMode) -> PathBuf {
    artifacts_dir().join(mode.as_str()).join(target.as_triple())
}

pub fn get_tar_path(target: BuildTarget, mode: BuildMode, package: &CargoTomlPackage) -> PathBuf {
    let artifacts_dir = get_artifacts_dir(target, mode);
    artifacts_dir.join(format!(
        "{}-{}.tar.xz",
        package.create_next_tag(),
        target.as_triple()
    ))
}
