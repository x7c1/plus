use crate::core::command::{Prepared, Runner, Unprepared};

pub trait CommandProvider: Sized {
    type Params;
    type Err;

    fn prepare(&self, params: &Self::Params) -> Result<Runner<Prepared>, Self::Err> {
        let unprepared = self.define(params)?;
        let prepared = unprepared.prepare(|a| self.satisfy_requirements(a))?;
        Ok(prepared)
    }

    fn define(&self, params: &Self::Params) -> Result<Runner<Unprepared>, Self::Err>;

    fn satisfy_requirements(&self, _runner: &Runner<Unprepared>) -> Result<(), Self::Err> {
        Ok(())
    }
}
