use crate::core::env::artifacts_dir;
use crate::core::targets::{AsBuildTarget, BuildTarget};
use std::path::{Path, PathBuf};

#[derive(Debug)]
pub struct Params {
    pub target: BuildTarget,
    pub src: PathBuf,
    pub dst: PathBuf,
}

impl Params {
    pub fn builder(target: BuildTarget) -> ParamsBuilder {
        ParamsBuilder {
            target,
            src: None,
            dst: None,
        }
    }
}

impl AsBuildTarget for Params {
    fn as_build_target(&self) -> &BuildTarget {
        &self.target
    }
}

pub struct ParamsBuilder {
    target: BuildTarget,
    src: Option<PathBuf>,
    dst: Option<PathBuf>,
}

impl ParamsBuilder {
    pub fn src(mut self, path: &Path) -> Self {
        self.src = Some(path.to_path_buf());
        self
    }

    pub fn dst(mut self, path: &Path) -> Self {
        let dst = artifacts_dir().join(self.target.as_abbr()).join(path);
        self.dst = Some(dst);
        self
    }

    pub fn build(self) -> Params {
        Params {
            target: self.target,
            src: self.src.expect("src is required"),
            dst: self.dst.expect("dst is required"),
        }
    }
}
