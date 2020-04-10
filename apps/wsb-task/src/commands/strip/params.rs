use crate::core::targets::{AsBuildTarget, BuildTarget};

#[derive(Debug)]
pub struct Params<'a> {
    pub target: &'a BuildTarget,
    pub file_path: String,
}

impl Params<'_> {
    pub fn builder(target: &BuildTarget) -> ParamsBuilder {
        ParamsBuilder {
            target,
            file_path: None,
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
    file_path: Option<String>,
}

impl<'a> ParamsBuilder<'a> {
    pub fn file_path<A: Into<String>>(mut self, path: A) -> Self {
        self.file_path = Some(path.into());
        self
    }
    pub fn build(self) -> Params<'a> {
        Params {
            target: self.target,
            file_path: self.file_path.expect("file path is required."),
        }
    }
}
