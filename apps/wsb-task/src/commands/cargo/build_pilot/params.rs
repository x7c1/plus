use crate::core::targets::{BuildTarget, RequireCC};
use failure::_core::marker::PhantomData;

pub struct DefaultFormat;
pub struct ShowFileName;

pub struct Params<T: BuildTarget, FORMAT> {
    pub target: T,
    _format: PhantomData<FORMAT>,
}

impl<T: BuildTarget, F> Params<T, F> {
    pub fn builder(_format: F) -> ParamsBuilder<T, F> {
        ParamsBuilder {
            target: None,
            format: PhantomData::<F>,
        }
    }
}

impl<T, F> RequireCC for Params<T, F>
where
    T: RequireCC + BuildTarget,
{
    const CC: &'static str = T::CC;
}

pub struct ParamsBuilder<T: BuildTarget, F> {
    target: Option<T>,
    format: PhantomData<F>,
}

impl<T, F> ParamsBuilder<T, F>
where
    T: BuildTarget,
{
    pub fn target(mut self, target: T) -> Self {
        self.target = Some(target);
        self
    }

    pub fn build(self) -> Params<T, F> {
        Params {
            target: self.target.expect("target is required"),
            _format: PhantomData::<F>,
        }
    }
}
