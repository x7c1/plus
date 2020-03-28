use crate::core::Action;
use crate::TaskResult;
use shellwork::core::command;
use shellwork::core::command::{Runner, Unprepared};

pub trait Definable {
    fn define(&self) -> TaskResult<Runner<Unprepared>>;
}

impl<A: Definable> command::CanDefine for Action<A> {
    type Params = A;
    type Err = crate::Error;

    fn define(&self, params: &Self::Params) -> Result<Runner<Unprepared>, Self::Err> {
        let runner = A::define(params)?;
        Ok(runner)
    }
}
