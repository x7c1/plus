use crate::commands::Action;
use crate::core::targets::{LinuxArmV7, LinuxX86};
use shellwork::core::command;
use shellwork::core::command::CanDefine;

pub trait Run {}

impl<A> command::ShouldRun for Action<LinuxX86, A>
where
    A: Run,
    Action<LinuxX86, A>: CanDefine,
{
}

impl<A> command::ShouldRun for Action<LinuxArmV7, A>
where
    A: Run,
    Action<LinuxArmV7, A>: CanDefine,
{
}
