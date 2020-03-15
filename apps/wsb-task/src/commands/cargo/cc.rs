use crate::commands::Action;
use crate::core::targets::{BuildTarget, InsertCC, RequireCC};
use crate::{Error, TaskResult};
use shellwork::core::command::{CanDefine, Runner, Unprepared};

// todo: rename
pub trait CanDefineByCC {
    fn with_cc<F>(&self, f: F) -> TaskResult<Runner<Unprepared>>
    where
        Self: Sized,
        F: Fn(&Self) -> Runner<Unprepared>;
}

impl<A> CanDefineByCC for A
where
    A: RequireCC,
{
    fn with_cc<F>(&self, f: F) -> TaskResult<Runner<Unprepared>>
    where
        F: Fn(&Self) -> Runner<Unprepared>,
    {
        let runner = f(self).env("CC", A::CC);
        Ok(runner)
    }
}

/*
impl<X> CanDefine for Action<X>
where
    X: BuildTarget,
    X: RequireCC,
    X: InsertCC,
    X: CanDefineByCC,
{
    type Params = X;
    type Err = crate::Error;

    fn define(&self, params: &Self::Params) -> TaskResult<Runner<Unprepared>> {
        let runner = X::define(params)?.env("CC", X::CC);
        Ok(runner)
    }
}
*/
