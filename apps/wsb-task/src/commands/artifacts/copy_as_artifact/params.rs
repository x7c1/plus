use crate::core::targets::{BuildTarget, RequireCC};
use std::path::{Path, PathBuf};

pub struct Params<T: BuildTarget> {
    pub target: T,
    src: PathBuf,
    dst: PathBuf,
}

impl<T: BuildTarget> Params<T> {
    pub fn builder(target: T) -> ParamsBuilder<T> {
        ParamsBuilder {
            target,
            src: None,
            dst: None,
        }
    }
}

impl<T: RequireCC + BuildTarget> RequireCC for Params<T> {
    const CC: &'static str = T::CC;
}

pub struct ParamsBuilder<T: BuildTarget> {
    target: T,
    src: Option<PathBuf>,
    dst: Option<PathBuf>,
}

impl<T> ParamsBuilder<T>
where
    T: BuildTarget,
{
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

    pub fn build(self) -> Params<T> {
        Params {
            target: self.target,
            src: self.src.expect("src is required"),
            dst: self.dst.expect("dst is required"),
        }
    }
}
