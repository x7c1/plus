use crate::commands::support::CCFindable;
use crate::core::targets::{AsBuildTarget, BuildTarget};
use crate::core::ActionOutput;
use std::path::PathBuf;

#[derive(Debug)]
pub struct Params {
    pub target: BuildTarget,
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
        }
    }
}

impl AsBuildTarget for Params {
    fn as_build_target(&self) -> &BuildTarget {
        &self.target
    }
}

impl CCFindable for Params {}

pub struct ParamsBuilder {
    target: Option<BuildTarget>,
    output_kind: OutputKind,
}

impl ParamsBuilder {
    pub fn target(mut self, target: BuildTarget) -> Self {
        self.target = Some(target);
        self
    }

    pub fn build(self) -> Params {
        Params {
            target: self.target.expect("target is required"),
            output_kind: self.output_kind,
        }
    }
}

impl ActionOutput<Params> {
    pub fn pilot_file_path(&self) -> PathBuf {
        PathBuf::new().join(self.stdout().as_ref())
    }
}
