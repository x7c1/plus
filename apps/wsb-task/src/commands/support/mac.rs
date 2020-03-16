use crate::commands::Action;
use crate::core::targets::MacX86;
use shellwork::core::command;
use shellwork::core::command::{CanDefine, UnsupportedReport};

pub trait RunMaybe {}

impl<A> command::MayRun<MacX86> for Action<A>
where
    A: RunMaybe,
    Action<A>: CanDefine,
{
    fn unsupported(&self) -> Option<UnsupportedReport> {
        // todo: check if sdk exists
        None
    }
}
