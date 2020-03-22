mod params;
use crate::commands::copy_as_artifact;
use crate::commands::support::mac;
use crate::commands::support::should;
use crate::commands::support::Definable;
use crate::core::targets::{BuildTarget, LinuxArmV7, LinuxX86, MacX86};
use crate::TaskResult;
pub use params::{Params, ParamsBuilder};
use shellwork::core::command;
use shellwork::core::command::{Runner, Unprepared};

fn define<T>(params: &copy_as_artifact::Params<T>) -> Runner<Unprepared>
where
    T: BuildTarget,
{
    command::program("cp").args(&[&params.src, &params.dst])
}

mod linux_x86 {
    use super::*;
    impl Definable for copy_as_artifact::Params<LinuxX86> {
        fn define(&self) -> TaskResult<Runner<Unprepared>> {
            Ok(define(self))
        }
    }
    impl should::Run for copy_as_artifact::Params<LinuxX86> {}
}

mod linux_arm_v7 {
    use super::*;
    impl Definable for copy_as_artifact::Params<LinuxArmV7> {
        fn define(&self) -> TaskResult<Runner<Unprepared>> {
            Ok(define(self))
        }
    }
    impl should::Run for copy_as_artifact::Params<LinuxArmV7> {}
}

mod mac_x86 {
    use super::*;
    impl Definable for copy_as_artifact::Params<MacX86> {
        fn define(&self) -> TaskResult<Runner<Unprepared>> {
            Ok(define(self))
        }
    }
    impl mac::RunMaybe for copy_as_artifact::Params<MacX86> {}
}
