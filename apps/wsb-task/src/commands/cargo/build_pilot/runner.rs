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

pub trait AppendCC {}

impl<T> CanDefine for build_pilot::Params<T>
where
    T: BuildTarget,
    T: RequiredCC,
    build_pilot::Params<T>: AppendCC,
{
    type Params = build_pilot::Params<T>;
    type Err = crate::Error;

    fn define(&self, params: &Self::Params) -> Result<Runner<Unprepared>, Self::Err> {
        let runner = base_runner(params).env("CC", T::CC);
        Ok(runner)
    }
}

pub trait RequiredCC {
    const CC: &'static str;
}

impl RequiredCC for LinuxArmV7 {
    const CC: &'static str = "arm-linux-gnueabihf-gcc";
}

impl RequiredCC for MacX86 {
    const CC: &'static str = "x86_64-apple-darwin19-clang";
}

mod linux_arm_v7 {
    use super::*;

    impl AppendCC for build_pilot::Params<LinuxArmV7> {}

    impl ShouldRun for build_pilot::Params<LinuxArmV7> {}
}

mod mac_x86 {
    use super::*;

    impl AppendCC for build_pilot::Params<MacX86> {}

    impl MayRun for build_pilot::Params<MacX86> {
        fn unsupported(&self) -> Option<UnsupportedReport> {
            // todo: check if sdk exists
            None
        }
    }
}
