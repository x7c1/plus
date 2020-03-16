use crate::core::targets::RequireCC;
use crate::TaskResult;
use shellwork::core::command::{Runner, Unprepared};

pub trait CanInsertCC {
    fn with_cc<F>(&self, f: F) -> TaskResult<Runner<Unprepared>>
    where
        Self: Sized,
        F: Fn(&Self) -> Runner<Unprepared>;
}

impl<A> CanInsertCC for A
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
