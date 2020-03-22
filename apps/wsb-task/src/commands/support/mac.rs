use crate::commands::Action;
use crate::core::targets::MacX86;
use shellwork::core::command;
use shellwork::core::command::{may_run, CanDefine, RunnerOutput, UnsupportedReport};

pub trait RunMaybe {}

impl<A> command::MayRun for Action<MacX86, A>
where
    A: RunMaybe,
    Action<MacX86, A>: CanDefine,
{
    fn unsupported(&self) -> Option<UnsupportedReport> {
        // todo: check if sdk exists
        None
    }
}

impl<A> command::Runnable for Action<MacX86, A>
where
    A: RunMaybe,
    Self: CanDefine,
    <Self as CanDefine>::Err: From<shellwork::Error>,
{
    fn spawn(&self, params: &Self::Params) -> Result<(), Self::Err> {
        may_run::spawn(self, params)
    }
    fn capture(&self, params: &Self::Params) -> Result<Option<RunnerOutput>, Self::Err> {
        may_run::capture(self, params)
    }
}
