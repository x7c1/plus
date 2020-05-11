use crate::commands::support::CCFindable;
use crate::core::targets::{AsBuildTarget, BuildTarget};

#[derive(Debug)]
pub struct Params {
    pub target: BuildTarget,
}

impl Params {
    pub fn builder() -> ParamsBuilder {
        ParamsBuilder { target: None }
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
}

impl<'a> ParamsBuilder {
    pub fn target(mut self, target: BuildTarget) -> Self {
        self.target = Some(target);
        self
    }

    pub fn build(self) -> Params {
        Params {
            target: self.target.expect("target is required"),
        }
    }
}
