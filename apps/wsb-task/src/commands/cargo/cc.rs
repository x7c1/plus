use crate::commands::Action;
use crate::core::targets::{BuildTarget, InsertCC, RequireCC};
use crate::TaskResult;
use shellwork::core::command::{CanDefine, Runner, Unprepared};

pub trait CanDefineByCC {
    fn define(params: &Self) -> TaskResult<Runner<Unprepared>>;
}

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
