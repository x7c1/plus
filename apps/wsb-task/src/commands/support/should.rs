use crate::commands::Action;
use crate::core::targets::{LinuxArmV7, LinuxX86};
use shellwork::core::command;
use shellwork::core::command::{should, CanDefine, RunnerOutput};

pub trait Run {}

mod linux_x86 {
    use super::*;
    impl<A> command::Runnable for Action<LinuxX86, A>
    where
        A: Run,
        Self: CanDefine,
        <Self as CanDefine>::Err: From<shellwork::Error>,
    {
        fn spawn(&self, params: &Self::Params) -> Result<(), Self::Err> {
            should::spawn(self, params)
        }
        fn capture(&self, params: &Self::Params) -> Result<Option<RunnerOutput>, Self::Err> {
            should::capture(self, params)
        }
    }
}

mod linux_arm_v7 {
    use super::*;
    impl<A> command::Runnable for Action<LinuxArmV7, A>
    where
        A: Run,
        Self: CanDefine,
        <Self as CanDefine>::Err: From<shellwork::Error>,
    {
        fn spawn(&self, params: &Self::Params) -> Result<(), Self::Err> {
            should::spawn(self, params)
        }
        fn capture(&self, params: &Self::Params) -> Result<Option<RunnerOutput>, Self::Err> {
            should::capture(self, params)
        }
    }
}
