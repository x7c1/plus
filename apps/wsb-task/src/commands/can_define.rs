use crate::commands::Action;
use crate::TaskResult;
use shellwork::core::command::{CanDefine, Runner, Unprepared};

// todo: rename
pub trait CanDefine2 {
    fn define(&self) -> TaskResult<Runner<Unprepared>>;
}

impl<X> CanDefine for Action<X>
where
    X: CanDefine2,
{
    type Params = X;
    type Err = crate::Error;

    fn define(&self, params: &Self::Params) -> Result<Runner<Unprepared>, Self::Err> {
        let runner = X::define(params)?;
        Ok(runner)
    }
}
