use crate::core::targets::{AsBuildTarget, BuildTarget};
use std::path::Path;

#[derive(Debug)]
pub struct Params<'a> {
    pub target: &'a BuildTarget,
    pub file_path: String,
    pub directory_path: String,
}

impl Params<'_> {
    pub fn builder(target: &BuildTarget) -> ParamsBuilder {
        ParamsBuilder { target }
    }
}

impl AsBuildTarget for Params<'_> {
    fn as_build_target(&self) -> &BuildTarget {
        self.target
    }
}

pub struct ParamsBuilder<'a> {
    target: &'a BuildTarget,
}

impl<'a> ParamsBuilder<'a> {
    pub fn build(self) -> Params<'a> {
        // todo: avoid magic string
        let dir = Path::new("dist");
        let filename = format!("{}.tar.xz", self.target.as_abbr());
        let file_path = dir.join(filename);

        Params {
            target: self.target,
            file_path: file_path.to_string_lossy().to_string(),
            directory_path: dir.to_string_lossy().to_string(),
        }
    }
}
