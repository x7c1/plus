use crate::commands::cargo::{mac, CanDefineByCC};
use crate::commands::cargo_build;
use crate::commands::Action;
use crate::commands::CanDefine2;
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

    impl ShouldRun for Action<cargo_build::Params<LinuxArmV7>> {}

    impl CanDefine2 for cargo_build::Params<LinuxArmV7> {
        fn define(&self) -> TaskResult<Runner<Unprepared>> {
            self.with_cc(base_runner)
        }
    }
}

mod mac_x86 {
    use super::*;

    impl mac::RunMaybe for cargo_build::Params<MacX86> {}

    impl CanDefine2 for cargo_build::Params<MacX86> {
        fn define(&self) -> TaskResult<Runner<Unprepared>> {
            self.with_cc(base_runner)
        }
    }
}
