use crate::commands::{Action, Action2};
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

// todo: rename
pub trait Definable2 {
    fn define(&self) -> TaskResult<Runner<Unprepared>>;
}

impl<A: Definable2> command::CanDefine for Action2<A> {
    type Params = A;
    type Err = crate::Error;

    fn define(&self, params: &Self::Params) -> Result<Runner<Unprepared>, Self::Err> {
        let runner = A::define(params)?;
        Ok(runner)
    }
}
