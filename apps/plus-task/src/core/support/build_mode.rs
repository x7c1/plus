use crate::core::build_mode::{AsBuildMode, BuildMode};

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
