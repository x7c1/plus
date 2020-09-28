use crate::core::build_mode::{AsBuildMode, BuildMode};
use crate::core::support::{CCRequired, HasBuildMode};
use crate::core::targets::{AsBuildTarget, BuildTarget};
use crate::tasks::shared::{build_mode, build_target};
use crate::TaskResult;
use clap::ArgMatches;

#[derive(Debug)]
pub struct Params {
    target: BuildTarget,
    build_mode: BuildMode,
}

impl Params {
    pub fn from_matches<'a>(matches: &'a ArgMatches<'a>) -> TaskResult<Self> {
        let params = Params::builder()
            .target(build_target::from(matches)?)
            .build_mode(build_mode::from(matches)?)
            .build();

        Ok(params)
    }
    pub fn builder() -> ParamsBuilder {
        ParamsBuilder {
            target: None,
            build_mode: None,
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
    build_mode: Option<BuildMode>,
}

impl<'a> ParamsBuilder {
    pub fn target(mut self, target: BuildTarget) -> Self {
        self.target = Some(target);
        self
    }

    pub fn build_mode(mut self, build_mode: BuildMode) -> Self {
        self.build_mode = Some(build_mode);
        self
    }

    pub fn build(self) -> Params {
        Params {
            target: self.target.expect("target is required"),
            build_mode: self.build_mode.expect("build-mode is required"),
        }
    }
}
