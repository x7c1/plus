use crate::core::targets::{BuildTarget, RequireCC};

#[derive(Debug)]
pub struct Params<T: BuildTarget> {
    pub target: T,
}

impl<T: BuildTarget> Params<T> {
    pub fn builder() -> ParamsBuilder<T> {
        ParamsBuilder { target: None }
    }
}

impl<T: RequireCC + BuildTarget> RequireCC for Params<T> {
    const CC: &'static str = T::CC;
}

pub struct ParamsBuilder<T: BuildTarget> {
    target: Option<T>,
}

impl<T> ParamsBuilder<T>
where
    T: BuildTarget,
{
    pub fn target(mut self, target: T) -> Self {
        self.target = Some(target);
        self
    }

    pub fn build(self) -> Params<T> {
        Params {
            target: self.target.expect("target is required"),
        }
    }
}
