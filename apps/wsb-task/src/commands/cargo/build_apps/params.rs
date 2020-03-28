use crate::commands::support::CCFindable;
use crate::core::targets::{AsBuildTarget, BuildTarget};

#[derive(Debug)]
pub struct Params<'a> {
    pub target: &'a BuildTarget,
}

impl<'a> Params<'a> {
    pub fn builder() -> ParamsBuilder<'a> {
        ParamsBuilder { target: None }
    }
}

impl AsBuildTarget for Params<'_> {
    fn as_build_target(&self) -> &BuildTarget {
        self.target
    }
}

impl CCFindable for Params<'_> {}

pub struct ParamsBuilder<'a> {
    target: Option<&'a BuildTarget>,
}

impl<'a> ParamsBuilder<'a> {
    pub fn target(mut self, target: &'a BuildTarget) -> Self {
        self.target = Some(target);
        self
    }

    pub fn build(self) -> Params<'a> {
        Params {
            target: self.target.expect("target is required"),
        }
    }
}
