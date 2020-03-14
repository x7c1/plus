use crate::commands::cargo::cargo_build::Params;
use crate::core::targets::{BuildTarget, LinuxArmV7, LinuxX86, MacX86};
use crate::TaskResult;
use shellwork::core::command;
use shellwork::core::command::{
    CanDefine, MayRun, Runner, ShouldRun, Unprepared, UnsupportedReport,
};

fn base_runner<T>(params: &Params<T>) -> Runner<Unprepared>
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

    impl CanDefine for Params<LinuxX86> {
        type Params = Self;
        type Err = crate::Error;

        fn define(&self, params: &Self) -> TaskResult<Runner<Unprepared>> {
            let runner = base_runner(params);
            Ok(runner)
        }
    }
    impl ShouldRun for Params<LinuxX86> {}
}

mod linux_arm_v7 {
    use super::*;

    impl CanDefine for Params<LinuxArmV7> {
        type Params = Self;
        type Err = crate::Error;

        fn define(&self, params: &Self) -> TaskResult<Runner<Unprepared>> {
            let runner = base_runner(params).env("CC", "arm-linux-gnueabihf-gcc");
            Ok(runner)
        }
    }
    impl ShouldRun for Params<LinuxArmV7> {}
}

mod mac_x86 {
    use super::*;

    impl CanDefine for Params<MacX86> {
        type Params = Self;
        type Err = crate::Error;

        fn define(&self, params: &Self) -> TaskResult<Runner<Unprepared>> {
            let runner = base_runner(params).env("CC", "x86_64-apple-darwin19-clang");
            Ok(runner)
        }
    }
    impl MayRun for Params<MacX86> {
        fn unsupported(&self) -> Option<UnsupportedReport> {
            // todo: check if sdk exists
            None
        }
    }
}
