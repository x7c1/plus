use crate::commands::Action;
use crate::TaskResult;
use shellwork::core::command;
use shellwork::core::command::{Runner, Unprepared};

pub trait Runnable {
    fn define(&self) -> TaskResult<Runner<Unprepared>>;
}

impl<X> command::CanDefine for Action<X>
where
    X: Runnable,
{
    type Params = X;
    type Err = crate::Error;

    fn define(&self, params: &Self::Params) -> Result<Runner<Unprepared>, Self::Err> {
        let runner = X::define(params)?;
        Ok(runner)
    }
}
