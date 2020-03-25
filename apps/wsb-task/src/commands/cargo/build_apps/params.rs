use crate::core::targets::{AsTargetArch, TargetArch};

#[derive(Debug)]
pub struct Params<'a> {
    pub target: &'a TargetArch,
}

impl<'a> Params<'a> {
    pub fn builder() -> ParamsBuilder<'a> {
        ParamsBuilder { target: None }
    }
}

impl AsTargetArch for Params<'_> {
    fn as_target_arch(&self) -> &TargetArch {
        self.target
    }
}

pub struct ParamsBuilder<'a> {
    target: Option<&'a TargetArch>,
}

impl<'a> ParamsBuilder<'a> {
    pub fn target(mut self, target: &'a TargetArch) -> Self {
        self.target = Some(target);
        self
    }

    pub fn build(self) -> Params<'a> {
        Params {
            target: self.target.expect("target is required"),
        }
    }
}
