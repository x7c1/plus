#[derive(Clone, Copy, Debug)]
pub enum BuildMode {
    Debug,
    Release,
}

impl BuildMode {
    pub fn all() -> Vec<BuildMode> {
        vec![BuildMode::Debug, BuildMode::Release]
    }
}

pub trait AsBuildMode {
    fn as_build_mode(&self) -> &BuildMode;
}
