use crate::commands::Action;
use crate::core::targets::{BuildTarget, InsertCC, RequireCC};
use shellwork::core::command::{CanDefine, Runner, Unprepared};

pub trait BaseRunner {
    fn runner(params: &Self) -> Runner<Unprepared>;
}

impl<X> CanDefine for Action<X>
where
    X: BuildTarget,
    X: RequireCC,
    X: InsertCC,
    X: BaseRunner,
{
    type Params = X;
    type Err = crate::Error;

    fn define(&self, params: &Self::Params) -> Result<Runner<Unprepared>, Self::Err> {
        let runner = X::runner(params).env("CC", X::CC);
        Ok(runner)
    }
}
