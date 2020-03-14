use crate::commands::cargo_build;
use crate::core::targets::{BuildTarget, InsertCC, LinuxArmV7, LinuxX86, MacX86, RequireCC};
use crate::TaskResult;
use shellwork::core::command;
use shellwork::core::command::{
    CanDefine, MayRun, Runner, ShouldRun, Unprepared, UnsupportedReport,
};

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

    impl CanDefine for cargo_build::Params<LinuxX86> {
        type Params = Self;
        type Err = crate::Error;

        fn define(&self, params: &Self) -> TaskResult<Runner<Unprepared>> {
            let runner = base_runner(params);
            Ok(runner)
        }
    }
    impl ShouldRun for cargo_build::Params<LinuxX86> {}
}

mod linux_arm_v7 {
    use super::*;

    impl InsertCC for cargo_build::Params<LinuxArmV7> {}

    impl ShouldRun for cargo_build::Params<LinuxArmV7> {}
}

mod mac_x86 {
    use super::*;

    impl InsertCC for cargo_build::Params<MacX86> {}

    impl MayRun for cargo_build::Params<MacX86> {
        fn unsupported(&self) -> Option<UnsupportedReport> {
            // todo: check if sdk exists
            None
        }
    }
}

impl<T> CanDefine for cargo_build::Params<T>
where
    T: BuildTarget,
    T: RequireCC,
    cargo_build::Params<T>: InsertCC,
{
    type Params = cargo_build::Params<T>;
    type Err = crate::Error;

    fn define(&self, params: &Self::Params) -> Result<Runner<Unprepared>, Self::Err> {
        let runner = base_runner(params).env("CC", T::CC);
        Ok(runner)
    }
}
