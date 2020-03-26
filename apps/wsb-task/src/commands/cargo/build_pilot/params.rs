use crate::core::targets::{AsBuildTarget, BuildTarget};

#[derive(Debug)]
pub struct Params<'a> {
    pub target: &'a BuildTarget,
    pub output_kind: OutputKind,
}

#[derive(Debug, PartialEq)]
pub enum OutputKind {
    Default,
    FileName,
}

impl<'a> Params<'a> {
    pub fn builder(kind: OutputKind) -> ParamsBuilder<'a> {
        ParamsBuilder {
            target: None,
            output_kind: kind,
        }
    }
}

impl AsBuildTarget for Params<'_> {
    fn as_build_target(&self) -> &BuildTarget {
        self.target
    }
}

pub struct ParamsBuilder<'a> {
    target: Option<&'a BuildTarget>,
    output_kind: OutputKind,
}

impl<'a> ParamsBuilder<'a> {
    pub fn target(mut self, target: &'a BuildTarget) -> Self {
        self.target = Some(target);
        self
    }

    pub fn build(self) -> Params<'a> {
        Params {
            target: self.target.expect("target is required"),
            output_kind: self.output_kind,
        }
    }
}
