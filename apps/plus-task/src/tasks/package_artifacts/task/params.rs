use crate::core::build_mode::{AsBuildMode, BuildMode};
use crate::core::env::artifacts_dir;
use crate::core::targets::{AsBuildTarget, BuildTarget};

#[derive(Debug)]
pub struct Params {
    pub dst_path: String,
    pub src_path: String,
}

impl Params {
    pub fn builder<P>(params: P) -> ParamsBuilder
    where
        P: AsBuildTarget,
        P: AsBuildMode,
    {
        ParamsBuilder {
            target: *params.as_build_target(),
            build_mode: *params.as_build_mode(),
            filename: None,
        }
    }
}

pub struct ParamsBuilder {
    pub target: BuildTarget,
    pub build_mode: BuildMode,
    pub filename: Option<String>,
}

impl ParamsBuilder {
    pub fn filename(mut self, name: String) -> Self {
        self.filename = Some(name);
        self
    }

    pub fn build(self) -> Params {
        /*
                let dir = artifacts_dir();
                let filename = format!("{}.tar.xz", self.target.as_abbr());
                let file_path = dir.join(filename);

                Params {
                    src_path: "".to_string(),
                    dst_path: file_path.to_string_lossy().to_string(),
                    // directory_path: dir.to_string_lossy().to_string(),
                }
        */
        unimplemented!()
    }
}
