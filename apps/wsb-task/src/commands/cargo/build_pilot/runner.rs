use crate::commands::cargo::build_pilot;
use crate::core::targets::{BuildTarget, LinuxArmV7, LinuxX86, MacX86};
use crate::TaskResult;
use shellwork::core::command;
use shellwork::core::command::{
    CanDefine, MayRun, Runner, ShouldRun, Unprepared, UnsupportedReport,
};

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

    impl CanDefine for build_pilot::Params<LinuxX86> {
        type Params = Self;
        type Err = crate::Error;

        fn define(&self, params: &Self) -> TaskResult<Runner<Unprepared>> {
            let runner = base_runner(params);
            Ok(runner)
        }
    }
    impl ShouldRun for build_pilot::Params<LinuxX86> {}
}

mod linux_arm_v7 {
    use super::*;

    impl CanDefine for build_pilot::Params<LinuxArmV7> {
        type Params = Self;
        type Err = crate::Error;

        fn define(&self, params: &Self) -> TaskResult<Runner<Unprepared>> {
            let runner = base_runner(params).env("CC", "arm-linux-gnueabihf-gcc");
            Ok(runner)
        }
    }
    impl ShouldRun for build_pilot::Params<LinuxArmV7> {}
}

mod mac_x86 {
    use super::*;

    impl CanDefine for build_pilot::Params<MacX86> {
        type Params = Self;
        type Err = crate::Error;

        fn define(&self, params: &Self) -> TaskResult<Runner<Unprepared>> {
            let runner = base_runner(params).env("CC", "x86_64-apple-darwin19-clang");
            Ok(runner)
        }
    }
    impl MayRun for build_pilot::Params<MacX86> {
        fn unsupported(&self) -> Option<UnsupportedReport> {
            // todo: check if sdk exists
            None
        }
    }
}
