use crate::core::build_mode::{AsBuildMode, BuildMode};
use crate::core::support::{CCRequired, HasBuildMode};
use crate::core::targets::{AsBuildTarget, BuildTarget};
use crate::core::ActionOutput;
use std::path::PathBuf;

#[derive(Debug)]
pub struct Params {
    pub target: BuildTarget,
    build_mode: BuildMode,
    pub output_kind: OutputKind,
}

#[derive(Debug, PartialEq)]
pub enum OutputKind {
    Default,
    FileName,
}

impl Params {
    pub fn builder(kind: OutputKind) -> ParamsBuilder {
        ParamsBuilder {
            target: None,
            output_kind: kind,
            build_mode: BuildMode::Debug,
        }
    }
}

impl AsBuildTarget for Params {
    fn as_build_target(&self) -> &BuildTarget {
        &self.target
    }
}

impl CCRequired for Params {}

impl AsBuildMode for Params {
    fn as_build_mode(&self) -> &BuildMode {
        &self.build_mode
    }
}
impl HasBuildMode for Params {}

pub struct ParamsBuilder {
    target: Option<BuildTarget>,
    output_kind: OutputKind,
    build_mode: BuildMode,
}

impl ParamsBuilder {
    pub fn target(mut self, target: BuildTarget) -> Self {
        self.target = Some(target);
        self
    }
    pub fn build_mode(mut self, build_mode: BuildMode) -> Self {
        self.build_mode = build_mode;
        self
    }
    pub fn build(self) -> Params {
        Params {
            target: self.target.expect("target is required"),
            build_mode: self.build_mode,
            output_kind: self.output_kind,
        }
    }
}

impl ActionOutput<Params> {
    pub fn pilot_file_path(&self) -> PathBuf {
        PathBuf::new().join(self.stdout().as_ref())
    }
}
