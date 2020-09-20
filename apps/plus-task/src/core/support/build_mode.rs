use crate::core::build_mode::{AsBuildMode, BuildMode};
use crate::core::env::artifacts_dir;
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
    let target = {
        let suffix = match mode {
            BuildMode::Debug => "-debug",
            BuildMode::Release => "",
        };
        format!("{}{}", target.as_abbr(), suffix)
    };
    artifacts_dir().join(target)
}
