use crate::core::targets::{AsBuildTarget, BuildTarget};
use std::path::{Path, PathBuf};

#[derive(Debug)]
pub struct Params<'a> {
    pub target: &'a BuildTarget,
    pub src: PathBuf,
    pub dst: PathBuf,
}

impl Params<'_> {
    pub fn builder(target: &BuildTarget) -> ParamsBuilder {
        ParamsBuilder {
            target,
            src: None,
            dst: None,
        }
    }
}

impl AsBuildTarget for Params<'_> {
    fn as_build_target(&self) -> &BuildTarget {
        self.target
    }
}

pub struct ParamsBuilder<'a> {
    target: &'a BuildTarget,
    src: Option<PathBuf>,
    dst: Option<PathBuf>,
}

impl<'a> ParamsBuilder<'a> {
    pub fn src(mut self, path: &Path) -> Self {
        self.src = Some(path.to_path_buf());
        self
    }

    pub fn dst(mut self, path: &Path) -> Self {
        // todo: avoid magic string
        let dst = Path::new("dist").join(self.target.as_abbr()).join(path);
        self.dst = Some(dst);
        self
    }

    pub fn build(self) -> Params<'a> {
        Params {
            target: self.target,
            src: self.src.expect("src is required"),
            dst: self.dst.expect("dst is required"),
        }
    }
}
