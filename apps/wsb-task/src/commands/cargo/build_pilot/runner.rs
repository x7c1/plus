use crate::commands::cargo::mac;
use crate::commands::cargo::{build_pilot, CanDefineByCC};
use crate::commands::Action;
use crate::commands::CanDefine2;
use crate::core::targets::InsertCC;
use crate::core::targets::{BuildTarget, LinuxArmV7, LinuxX86, MacX86};
use crate::TaskResult;
use shellwork::core::command;
use shellwork::core::command::{CanDefine, Runner, ShouldRun, Unprepared};

fn base_runner<T>(params: &build_pilot::Params<T>) -> Runner<Unprepared>
where
    T: BuildTarget,
{
    // todo: move opt-level to params
    // todo: enable to add mode (--release)
    command::program("cargo")
        .arg("test")
        .arg("--verbose")
        .args(&["--target", &params.target.as_triple()])
        .arg("--no-run")
        .env("RUSTFLAGS", "-C opt-level=0")
}

mod linux_x86 {
    use super::*;

    impl CanDefine2 for build_pilot::Params<LinuxX86> {
        fn define(&self) -> TaskResult<Runner<Unprepared>> {
            Ok(base_runner(self))
        }
    }
    /*
    impl CanDefine for Action<build_pilot::Params<LinuxX86>> {
        type Params = build_pilot::Params<LinuxX86>;
        type Err = crate::Error;

        fn define(&self, params: &Self::Params) -> TaskResult<Runner<Unprepared>> {
            let runner = base_runner(params);
            Ok(runner)
        }
    }
    // */
    impl ShouldRun for Action<build_pilot::Params<LinuxX86>> {}
}

mod linux_arm_v7 {
    use super::*;

    impl InsertCC for build_pilot::Params<LinuxArmV7> {}

    impl ShouldRun for Action<build_pilot::Params<LinuxArmV7>> {}

    impl CanDefine2 for build_pilot::Params<LinuxArmV7> {
        fn define(&self) -> TaskResult<Runner<Unprepared>> {
            // todo: add CC
            Ok(base_runner(self))
        }
    }
}

mod mac_x86 {
    use super::*;

    impl InsertCC for build_pilot::Params<MacX86> {}

    impl mac::RunMaybe for build_pilot::Params<MacX86> {}

    impl CanDefine2 for build_pilot::Params<MacX86> {
        fn define(&self) -> TaskResult<Runner<Unprepared>> {
            // todo: add CC
            Ok(base_runner(self))
        }
    }
}

/*
impl<A> CanDefineByCC for build_pilot::Params<A>
where
    A: BuildTarget,
    build_pilot::Params<A>: InsertCC,
{
    fn define(&self) -> TaskResult<Runner<Unprepared>> {
        Ok(base_runner(self))
    }
}
*/
