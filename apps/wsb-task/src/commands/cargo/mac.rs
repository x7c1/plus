use crate::commands::Action;
use shellwork::core::command;
use shellwork::core::command::{CanDefine, UnsupportedReport};

pub trait RunMaybe {}

impl<A> command::MayRun for Action<A>
where
    A: RunMaybe,
    Action<A>: CanDefine,
{
    fn unsupported(&self) -> Option<UnsupportedReport> {
        // todo: check if sdk exists
        None
    }
}
