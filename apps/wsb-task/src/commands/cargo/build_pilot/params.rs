use crate::core::targets::{BuildTarget, RequireCC};

pub struct Params<T: BuildTarget> {
    pub target: T,
    output_kind: OutputKind,
}

pub enum OutputKind {
    Default,
    FileName,
}

impl<T: BuildTarget> Params<T> {
    pub fn builder(kind: OutputKind) -> ParamsBuilder<T> {
        ParamsBuilder {
            target: None,
            output_kind: kind,
        }
    }
}

impl<T> RequireCC for Params<T>
where
    T: RequireCC + BuildTarget,
{
    const CC: &'static str = T::CC;
}

pub struct ParamsBuilder<T: BuildTarget> {
    target: Option<T>,
    output_kind: OutputKind,
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
            output_kind: self.output_kind,
        }
    }
}
