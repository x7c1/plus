use crate::core::command::{CanDefine, RunnerOutput};

pub trait Runnable: CanDefine {
    fn spawn(&self, params: &Self::Params) -> Result<(), Self::Err>
    where
        <Self as CanDefine>::Err: From<crate::Error>;

    fn capture(&self, params: &Self::Params) -> Result<Option<RunnerOutput>, Self::Err>
    where
        <Self as CanDefine>::Err: From<crate::Error>;
}
