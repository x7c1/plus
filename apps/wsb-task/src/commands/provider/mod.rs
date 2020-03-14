use crate::core::targets::BuildTarget;
use crate::TaskResult;
use shellwork::core::command::{Prepared, Runner, Unprepared};

pub trait CommandProvider: BuildTarget + Sized {
    type Params;

    fn prepare(&self, params: &Self::Params) -> TaskResult<Runner<Prepared>> {
        let unprepared = self.define(params)?;
        let prepared = unprepared.prepare(|a| self.satisfy_requirements(a))?;
        Ok(prepared)
    }

    fn define(&self, params: &Self::Params) -> TaskResult<Runner<Unprepared>>;

    fn satisfy_requirements(&self, _runner: &Runner<Unprepared>) -> TaskResult<()> {
        Ok(())
    }
}
