use crate::commands::Action;
use crate::TaskResult;
use shellwork::core::command;
use shellwork::core::command::{Runner, Unprepared};

pub trait Definable {
    fn define(&self) -> TaskResult<Runner<Unprepared>>;
}

impl<T, P> command::CanDefine for Action<T, P>
where
    P: Definable,
{
    type Params = P;
    type Err = crate::Error;

    fn define(&self, params: &Self::Params) -> Result<Runner<Unprepared>, Self::Err> {
        let runner = P::define(params)?;
        Ok(runner)
    }
}
