pub mod target;

use crate::commands::support::CCFindable;
use crate::core::targets::{AsBuildTarget, BuildTarget};
use crate::TaskResult;
use clap::ArgMatches;
use clap_extractor::Matcher;

#[derive(Debug)]
pub struct SharedParams {
    pub target: BuildTarget,
}

impl SharedParams {
    pub fn from_matches<'a>(matches: &'a ArgMatches<'a>) -> TaskResult<Self> {
        let params = SharedParams::builder()
            .target(matches.single("target").as_required()?)
            .build();

        Ok(params)
    }
    pub fn builder() -> ParamsBuilder {
        ParamsBuilder { target: None }
    }
}

impl AsBuildTarget for SharedParams {
    fn as_build_target(&self) -> &BuildTarget {
        &self.target
    }
}

impl CCFindable for SharedParams {}

pub struct ParamsBuilder {
    target: Option<BuildTarget>,
}

impl<'a> ParamsBuilder {
    pub fn target(mut self, target: BuildTarget) -> Self {
        self.target = Some(target);
        self
    }

    pub fn build(self) -> SharedParams {
        SharedParams {
            target: self.target.expect("target is required"),
        }
    }
}
