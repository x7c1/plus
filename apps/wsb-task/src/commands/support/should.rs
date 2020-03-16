use crate::commands::Action;
use crate::core::targets::{LinuxArmV7, LinuxX86};
use shellwork::core::command;
use shellwork::core::command::CanDefine;

pub trait Run {}

impl<A> command::ShouldRun for Action<A, LinuxX86>
where
    A: Run,
    Action<A, LinuxX86>: CanDefine,
{
}

impl<A> command::ShouldRun for Action<A, LinuxArmV7>
where
    A: Run,
    Action<A, LinuxArmV7>: CanDefine,
{
}
