use crate::core::build_mode::BuildMode;
use crate::core::support::get_artifacts_dir;
use crate::core::targets::{AsBuildTarget, BuildTarget};
use std::path::{Path, PathBuf};

#[derive(Debug)]
pub struct Params {
    pub target: BuildTarget,
    pub mode: BuildMode,
    pub src: PathBuf,
    pub dst: PathBuf,
}

impl Params {
    pub fn builder(target: BuildTarget, mode: BuildMode) -> ParamsBuilder {
        ParamsBuilder {
            target,
            mode,
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
    mode: BuildMode,
    src: Option<PathBuf>,
    dst: Option<PathBuf>,
}

impl ParamsBuilder {
    pub fn src(mut self, path: &Path) -> Self {
        self.src = Some(path.to_path_buf());
        self
    }

    pub fn dst(mut self, path: &Path) -> Self {
        let dst = get_artifacts_dir(self.target, self.mode).join(path);
        self.dst = Some(dst);
        self
    }

    pub fn build(self) -> Params {
        Params {
            target: self.target,
            mode: self.mode,
            src: self.src.expect("src is required"),
            dst: self.dst.expect("dst is required"),
        }
    }
}
