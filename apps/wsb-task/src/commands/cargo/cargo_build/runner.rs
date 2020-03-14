use crate::commands::cargo::{mac, BaseRunner};
use crate::commands::cargo_build;
use crate::commands::Action;
use crate::core::targets::{BuildTarget, InsertCC, LinuxArmV7, LinuxX86, MacX86};
use crate::TaskResult;
use shellwork::core::command;
use shellwork::core::command::{CanDefine, Runner, ShouldRun, Unprepared};

fn base_runner<T>(params: &cargo_build::Params<T>) -> Runner<Unprepared>
where
    T: BuildTarget,
{
    // todo: move opt-level to params
    command::program("cargo")
        .arg("build")
        .arg("--verbose")
        .args(&["--target", &params.target.as_triple()])
        .env("RUSTFLAGS", "-C opt-level=0")
}

mod linux_x86 {
    use super::*;

    impl CanDefine for Action<cargo_build::Params<LinuxX86>> {
        type Params = cargo_build::Params<LinuxX86>;
        type Err = crate::Error;

        fn define(&self, params: &Self::Params) -> TaskResult<Runner<Unprepared>> {
            let runner = base_runner(params);
            Ok(runner)
        }
    }
    impl ShouldRun for Action<cargo_build::Params<LinuxX86>> {}
}

mod linux_arm_v7 {
    use super::*;
    impl InsertCC for cargo_build::Params<LinuxArmV7> {}
    impl ShouldRun for Action<cargo_build::Params<LinuxArmV7>> {}
}

mod mac_x86 {
    use super::*;
    impl InsertCC for cargo_build::Params<MacX86> {}
    impl mac::RunMaybe for cargo_build::Params<MacX86> {}
}

impl<A> BaseRunner for cargo_build::Params<A>
where
    A: BuildTarget,
    cargo_build::Params<A>: InsertCC,
{
    fn runner(params: &Self) -> Runner<Unprepared> {
        base_runner(&params)
    }
}
